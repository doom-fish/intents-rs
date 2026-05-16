import Foundation
import Intents

@_cdecl("inx_intent_response_create")
public func inx_intent_response_create() -> UnsafeMutableRawPointer? {
    inxRetain(INIntentResponse())
}

@_cdecl("inx_send_message_intent_response_create")
public func inx_send_message_intent_response_create(
    _ code: Int,
    _ userActivityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INSendMessageIntentResponse requires macOS 12+")
        return nil
    }
    guard let code = INSendMessageIntentResponseCode(rawValue: code) else {
        outError?.pointee = inxCString("invalid INSendMessageIntentResponseCode raw value")
        return nil
    }

    let userActivity: NSUserActivity? = userActivityPtr.map { inxUnretained($0) }
    return inxRetain(INSendMessageIntentResponse(code: code, userActivity: userActivity))
}
