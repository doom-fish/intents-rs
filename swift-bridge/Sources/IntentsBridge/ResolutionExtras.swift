import Foundation
import Intents

private func inxResolutionResultUnsupportedForReason(
    className: String,
    reason: Int,
    outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let cls = inxClass(named: className), let metaClass = object_getClass(cls) else {
        outError?.pointee = inxCString("unknown intent resolution result class \(className)")
        return nil
    }
    let selector = NSSelectorFromString("unsupportedWithReason:")
    guard class_respondsToSelector(metaClass, selector) else {
        outError?.pointee = inxCString("\(className) does not respond to unsupportedWithReason:")
        return nil
    }
    typealias Fn = @convention(c) (AnyClass, Selector, Int) -> AnyObject
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    return inxRetain(function(cls, selector, reason))
}

@_cdecl("inx_intent_resolution_result_unsupported_with_reason")
public func inx_intent_resolution_result_unsupported_with_reason(
    _ reason: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    inxResolutionResultUnsupportedForReason(
        className: "INIntentResolutionResult",
        reason: reason,
        outError: outError
    )
}

@_cdecl("inx_intent_resolution_result_confirmation_required_with_item_for_reason")
public func inx_intent_resolution_result_confirmation_required_with_item_for_reason(
    _ itemPtr: UnsafeMutableRawPointer?,
    _ reason: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let cls = inxClass(named: "INIntentResolutionResult"), let metaClass = object_getClass(cls) else {
        outError?.pointee = inxCString("INIntentResolutionResult class was unavailable")
        return nil
    }
    let selector = NSSelectorFromString("confirmationRequiredWithItemToConfirm:forReason:")
    guard class_respondsToSelector(metaClass, selector) else {
        outError?.pointee = inxCString("INIntentResolutionResult does not respond to confirmationRequiredWithItemToConfirm:forReason:")
        return nil
    }

    typealias Fn = @convention(c) (AnyClass, Selector, AnyObject?, Int) -> AnyObject
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    return inxRetain(function(cls, selector, itemPtr.map { Unmanaged<AnyObject>.fromOpaque($0).takeUnretainedValue() }, reason))
}

@_cdecl("inx_typed_intent_resolution_result_unsupported_for_reason")
public func inx_typed_intent_resolution_result_unsupported_for_reason(
    _ className: UnsafePointer<CChar>?,
    _ reason: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let className else {
        outError?.pointee = inxCString("typed intent resolution result class name was NULL")
        return nil
    }
    return inxResolutionResultUnsupportedForReason(
        className: String(cString: className),
        reason: reason,
        outError: outError
    )
}
