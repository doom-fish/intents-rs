import Foundation
import Intents

private func inxResponseError(_ message: String) -> NSError {
    NSError(domain: "IntentsBridge", code: -1, userInfo: [NSLocalizedDescriptionKey: message])
}

private func inxResponseSubclass(named name: String, code: Int, userActivity: NSUserActivity?) throws -> INIntentResponse {
    switch name {
    case "INAnswerCallIntentResponse":
        guard #available(macOS 13.1, *) else { throw inxResponseError("\(name) requires macOS 13.1") }
        return INAnswerCallIntentResponse(code: INAnswerCallIntentResponseCode(rawValue: code) ?? .unspecified, userActivity: userActivity)
    case "INEditMessageIntentResponse":
        guard #available(macOS 14.0, *) else { throw inxResponseError("\(name) requires macOS 14") }
        return INEditMessageIntentResponse(code: INEditMessageIntentResponseCode(rawValue: code) ?? .unspecified, userActivity: userActivity)
    case "INGetReservationDetailsIntentResponse":
        return INGetReservationDetailsIntentResponse(code: INGetReservationDetailsIntentResponseCode(rawValue: code) ?? .unspecified, userActivity: userActivity)
    case "INHangUpCallIntentResponse":
        guard #available(macOS 13.1, *) else { throw inxResponseError("\(name) requires macOS 13.1") }
        return INHangUpCallIntentResponse(code: INHangUpCallIntentResponseCode(rawValue: code) ?? .unspecified, userActivity: userActivity)
    case "INShareFocusStatusIntentResponse":
        guard #available(macOS 12.0, *) else { throw inxResponseError("\(name) requires macOS 12") }
        return INShareFocusStatusIntentResponse(code: INShareFocusStatusIntentResponseCode(rawValue: code) ?? .unspecified, userActivity: userActivity)
    case "INUnsendMessagesIntentResponse":
        guard #available(macOS 14.0, *) else { throw inxResponseError("\(name) requires macOS 14") }
        return INUnsendMessagesIntentResponse(code: INUnsendMessagesIntentResponseCode(rawValue: code) ?? .unspecified, userActivity: userActivity)
    default:
        throw inxResponseError("unknown intent response subclass \(name)")
    }
}

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
    do {
        let response = try inxResponseSubclass(
            named: String(cString: className),
            code: code,
            userActivity: userActivityPtr.map { inxUnretained($0) as NSUserActivity }
        )
        return inxRetain(response)
    } catch {
        outError?.pointee = inxCString(error.localizedDescription)
        return nil
    }
}
