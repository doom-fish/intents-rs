import Foundation
import Intents

@_cdecl("inx_intent_response_subclass_create")
public func inx_intent_response_subclass_create(
    _ className: UnsafePointer<CChar>?,
    _ code: Int,
    _ userActivityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let className else {
        outError?.pointee = inxCString("intent response subclass class name was NULL")
        return nil
    }

    let name = String(cString: className)
    guard let object = inxAllocObject(className: name) else {
        outError?.pointee = inxCString("unknown intent response subclass \(name)")
        return nil
    }

    let selector = NSSelectorFromString("initWithCode:userActivity:")
    guard object.responds(to: selector) else {
        outError?.pointee = inxCString("\(name) does not respond to initWithCode:userActivity:")
        return nil
    }

    typealias Fn = @convention(c) (AnyObject, Selector, Int, NSUserActivity?) -> AnyObject
    let imp = object.method(for: selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    let response = function(
        object,
        selector,
        code,
        userActivityPtr.map { inxUnretained($0) as NSUserActivity }
    )
    return inxRetain(response)
}
