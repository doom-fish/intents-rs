import Foundation
import Intents

public typealias INXStatusCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int64,
    UnsafePointer<CChar>?
) -> Void
public typealias INXObjectCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void
public typealias INXArrayCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    Int,
    UnsafePointer<CChar>?
) -> Void
public typealias INXErrorCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void

@inline(__always)
func inxCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func inxRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func inxUnretained<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func inxRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@_cdecl("inx_string_free")
public func inx_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    free(string)
}

@_cdecl("inx_object_release")
public func inx_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    inxRelease(ptr)
}

func inxClass(named name: String) -> AnyClass? {
    NSClassFromString(name)
}

func inxClassIntMethod(_ className: String, selectorName: String) -> Int? {
    guard let cls = inxClass(named: className),
          let metaClass = object_getClass(cls)
    else {
        return nil
    }

    let selector = NSSelectorFromString(selectorName)
    guard class_respondsToSelector(metaClass, selector) else {
        return nil
    }

    typealias Fn = @convention(c) (AnyClass, Selector) -> Int
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    return function(cls, selector)
}

func inxAllocObject(className: String) -> NSObject? {
    guard let cls = inxClass(named: className), let metaClass = object_getClass(cls) else {
        return nil
    }

    let selector = NSSelectorFromString("alloc")
    guard class_respondsToSelector(metaClass, selector) else {
        return nil
    }

    typealias Fn = @convention(c) (AnyClass, Selector) -> AnyObject
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    return function(cls, selector) as? NSObject
}

@_cdecl("inx_object_create_blank")
public func inx_object_create_blank(
    _ className: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let className else {
        outError?.pointee = inxCString("class name was NULL")
        return nil
    }

    let name = String(cString: className)
    guard let cls = inxClass(named: name) as? NSObject.Type else {
        outError?.pointee = inxCString("unknown Objective-C class \(name)")
        return nil
    }

    return inxRetain(cls.init())
}

@_cdecl("inx_class_conforms_to_protocol")
public func inx_class_conforms_to_protocol(
    _ className: UnsafePointer<CChar>?,
    _ protocolName: UnsafePointer<CChar>?
) -> Bool {
    guard let className, let protocolName else { return false }
    let name = String(cString: className)
    let protocolValue = String(cString: protocolName)
    guard let cls = inxClass(named: name), let proto = NSProtocolFromString(protocolValue) else {
        return false
    }
    return class_conformsToProtocol(cls, proto)
}

@_cdecl("inx_intents_version_number")
public func inx_intents_version_number() -> Double {
    IntentsVersionNumber
}

@_cdecl("inx_intents_version_string")
public func inx_intents_version_string() -> UnsafeMutablePointer<CChar>? {
    let bundle = Bundle(path: "/System/Library/Frameworks/Intents.framework")
    let version = (bundle?.object(forInfoDictionaryKey: "CFBundleVersion") as? String)
        ?? (bundle?.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String)
        ?? ""
    return inxCString(version)
}

final class StatusCallbackBox {
    private let callback: INXStatusCallback
    private let refcon: UnsafeMutableRawPointer?

    init(callback: @escaping INXStatusCallback, refcon: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.refcon = refcon
    }

    func send(status: Int64, errorMessage: String? = nil) {
        if let errorMessage {
            errorMessage.withCString { callback(refcon, status, $0) }
        } else {
            callback(refcon, status, nil)
        }
    }
}

final class ObjectCallbackBox {
    private let callback: INXObjectCallback
    private let refcon: UnsafeMutableRawPointer?

    init(callback: @escaping INXObjectCallback, refcon: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.refcon = refcon
    }

    func send(object: AnyObject?, errorMessage: String? = nil) {
        let pointer = object.map(inxRetain)
        if let errorMessage {
            errorMessage.withCString { callback(refcon, pointer, $0) }
        } else {
            callback(refcon, pointer, nil)
        }
    }
}

final class ArrayCallbackBox {
    private let callback: INXArrayCallback
    private let refcon: UnsafeMutableRawPointer?

    init(callback: @escaping INXArrayCallback, refcon: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.refcon = refcon
    }

    func send(objects: [AnyObject]?, errorMessage: String? = nil) {
        let count = objects?.count ?? 0
        let buffer = count == 0
            ? nil
            : UnsafeMutablePointer<UnsafeMutableRawPointer?>.allocate(capacity: count)
        defer { buffer?.deallocate() }

        if let objects, let buffer {
            for (index, object) in objects.enumerated() {
                buffer[index] = inxRetain(object)
            }
        }

        if let errorMessage {
            errorMessage.withCString { callback(refcon, buffer, count, $0) }
        } else {
            callback(refcon, buffer, count, nil)
        }
    }
}

final class ErrorCallbackBox {
    private let callback: INXErrorCallback
    private let refcon: UnsafeMutableRawPointer?

    init(callback: @escaping INXErrorCallback, refcon: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.refcon = refcon
    }

    func send(errorMessage: String? = nil) {
        if let errorMessage {
            errorMessage.withCString { callback(refcon, $0) }
        } else {
            callback(refcon, nil)
        }
    }
}
