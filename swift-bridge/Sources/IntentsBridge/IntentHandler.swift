import Foundation
import Intents

private final class INXIntentHandlerProviderBox: NSObject, INIntentHandlerProviding {
    private var handlers: [String: NSString] = [:]

    func register(intentClassName: String, handlerName: String) {
        handlers[intentClassName] = NSString(string: handlerName)
    }

    func handler(for intent: INIntent) -> Any? {
        handlers[NSStringFromClass(type(of: intent))]
    }
}

@_cdecl("inx_intent_handler_provider_create")
public func inx_intent_handler_provider_create() -> UnsafeMutableRawPointer? {
    inxRetain(INXIntentHandlerProviderBox())
}

@_cdecl("inx_intent_handler_provider_register")
public func inx_intent_handler_provider_register(
    _ providerPtr: UnsafeMutableRawPointer?,
    _ intentClassName: UnsafePointer<CChar>?,
    _ handlerName: UnsafePointer<CChar>?
) -> Bool {
    guard let providerPtr,
          let intentClassName,
          let handlerName
    else {
        return false
    }

    let provider: INXIntentHandlerProviderBox = inxUnretained(providerPtr)
    provider.register(
        intentClassName: String(cString: intentClassName),
        handlerName: String(cString: handlerName)
    )
    return true
}

@_cdecl("inx_intent_handler_provider_copy_handler_name_for_intent")
public func inx_intent_handler_provider_copy_handler_name_for_intent(
    _ providerPtr: UnsafeMutableRawPointer?,
    _ intentPtr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let providerPtr, let intentPtr else { return nil }

    let provider: INXIntentHandlerProviderBox = inxUnretained(providerPtr)
    let intent: INIntent = inxUnretained(intentPtr)
    guard let name = provider.handler(for: intent) as? NSString else { return nil }
    return inxCString(name as String)
}
