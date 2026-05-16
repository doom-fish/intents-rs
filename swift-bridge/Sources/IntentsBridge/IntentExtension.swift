import Foundation
import Intents

@_cdecl("inx_intent_extension_create")
public func inx_intent_extension_create() -> UnsafeMutableRawPointer? {
    inxRetain(INExtension())
}

@_cdecl("inx_intent_extension_copy_handler_class_name_for_intent")
public func inx_intent_extension_copy_handler_class_name_for_intent(
    _ extensionPtr: UnsafeMutableRawPointer?,
    _ intentPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let extensionPtr, let intentPtr else { return nil }

    let `extension`: INExtension = inxUnretained(extensionPtr)
    let intent: INIntent = inxUnretained(intentPtr)
    guard let handler = `extension`.handler(for: intent) else { return nil }
    return inxCString(NSStringFromClass(type(of: handler as AnyObject)))
}
