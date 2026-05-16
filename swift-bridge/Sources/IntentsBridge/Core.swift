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
