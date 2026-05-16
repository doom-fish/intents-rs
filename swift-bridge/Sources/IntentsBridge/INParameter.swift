import Foundation
import Intents

private func inxParameterClass() -> NSObject.Type? {
    NSClassFromString("INParameter") as? NSObject.Type
}

@_cdecl("inx_parameter_create")
public func inx_parameter_create(
    _ intentClassName: UnsafePointer<CChar>?,
    _ keyPath: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let intentClassName else {
        outError?.pointee = inxCString("intent class name was NULL")
        return nil
    }
    guard let keyPath else {
        outError?.pointee = inxCString("parameter key path was NULL")
        return nil
    }
    guard let parameterClass = inxParameterClass() else {
        outError?.pointee = inxCString("INParameter is unavailable on this system")
        return nil
    }

    let className = String(cString: intentClassName)
    guard let targetClass = NSClassFromString(className) else {
        outError?.pointee = inxCString("intent class \(className) is unavailable")
        return nil
    }

    let selector = NSSelectorFromString("parameterForClass:keyPath:")
    guard let parameter = parameterClass
        .perform(selector, with: targetClass, with: NSString(string: String(cString: keyPath)))?
        .takeUnretainedValue()
    else {
        outError?.pointee = inxCString("failed to construct INParameter")
        return nil
    }
    return inxRetain(parameter as AnyObject)
}

@_cdecl("inx_parameter_set_index")
public func inx_parameter_set_index(
    _ parameterPtr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ subKeyPath: UnsafePointer<CChar>?
) -> Bool {
    guard let parameterPtr, let subKeyPath else { return false }

    let parameter: AnyObject = inxUnretained(parameterPtr)
    let selector = NSSelectorFromString("setIndex:forSubKeyPath:")
    guard parameter.responds(to: selector) else { return false }

    typealias Fn = @convention(c) (AnyObject, Selector, Int, NSString) -> Void
    let function = unsafeBitCast(parameter.method(for: selector), to: Fn.self)
    function(parameter, selector, index, NSString(string: String(cString: subKeyPath)))
    return true
}

@_cdecl("inx_parameter_get_index")
public func inx_parameter_get_index(
    _ parameterPtr: UnsafeMutableRawPointer?,
    _ subKeyPath: UnsafePointer<CChar>?,
    _ outPresent: UnsafeMutablePointer<Bool>?
) -> Int {
    guard let parameterPtr, let subKeyPath else {
        outPresent?.pointee = false
        return 0
    }

    let parameter: AnyObject = inxUnretained(parameterPtr)
    let selector = NSSelectorFromString("indexForSubKeyPath:")
    guard parameter.responds(to: selector) else {
        outPresent?.pointee = false
        return 0
    }

    typealias Fn = @convention(c) (AnyObject, Selector, NSString) -> Int
    let function = unsafeBitCast(parameter.method(for: selector), to: Fn.self)
    let index = function(parameter, selector, NSString(string: String(cString: subKeyPath)))
    let present = index != NSNotFound
    outPresent?.pointee = present
    return present ? index : 0
}
