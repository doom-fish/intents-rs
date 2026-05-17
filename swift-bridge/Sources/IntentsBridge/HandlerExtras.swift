import Foundation
import Intents

private class INXIntentHandlingHelperBase: NSObject {
    @objc dynamic private(set) var handleCallCount = 0
    @objc dynamic private(set) var confirmCallCount = 0
    @objc dynamic private(set) var resolveCallCount = 0
    @objc dynamic private(set) var lastIntentClassName: String?

    func recordHandle(intent: NSObject) {
        handleCallCount += 1
        lastIntentClassName = NSStringFromClass(type(of: intent))
    }

    func recordConfirm(intent: NSObject) {
        confirmCallCount += 1
        lastIntentClassName = NSStringFromClass(type(of: intent))
    }

    func recordResolve(intent: NSObject) {
        resolveCallCount += 1
        lastIntentClassName = NSStringFromClass(type(of: intent))
    }

    func simulateHandle() -> Bool { false }
    func simulateConfirm() -> Bool { false }
    func simulateResolve() -> Bool { false }
}

@available(macOS 13.1, *)
private final class INXAnswerCallIntentHandlingBox: INXIntentHandlingHelperBase, INAnswerCallIntentHandling {
    override func simulateHandle() -> Bool {
        var completionCalled = false
        handle(intent: INAnswerCallIntent(audioRoute: .speakerphoneAudioRoute, callIdentifier: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateConfirm() -> Bool {
        var completionCalled = false
        confirm(intent: INAnswerCallIntent(audioRoute: .speakerphoneAudioRoute, callIdentifier: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    func handle(intent: INAnswerCallIntent, completion: @escaping (INAnswerCallIntentResponse) -> Void) {
        recordHandle(intent: intent)
        completion(INAnswerCallIntentResponse(code: .ready, userActivity: nil))
    }

    func confirm(intent: INAnswerCallIntent, completion: @escaping (INAnswerCallIntentResponse) -> Void) {
        recordConfirm(intent: intent)
        completion(INAnswerCallIntentResponse(code: .ready, userActivity: nil))
    }
}

@available(macOS 14.0, *)
private final class INXEditMessageIntentHandlingBox: INXIntentHandlingHelperBase, INEditMessageIntentHandling {
    override func simulateHandle() -> Bool {
        var completionCalled = false
        handle(intent: INEditMessageIntent(messageIdentifier: nil, editedContent: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateConfirm() -> Bool {
        var completionCalled = false
        confirm(intent: INEditMessageIntent(messageIdentifier: nil, editedContent: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateResolve() -> Bool {
        var completionCalled = false
        resolveEditedContent(for: INEditMessageIntent(messageIdentifier: nil, editedContent: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    func handle(intent: INEditMessageIntent, completion: @escaping (INEditMessageIntentResponse) -> Void) {
        recordHandle(intent: intent)
        completion(INEditMessageIntentResponse(code: .ready, userActivity: nil))
    }

    func confirm(intent: INEditMessageIntent, completion: @escaping (INEditMessageIntentResponse) -> Void) {
        recordConfirm(intent: intent)
        completion(INEditMessageIntentResponse(code: .ready, userActivity: nil))
    }

    func resolveEditedContent(for intent: INEditMessageIntent, with completion: @escaping (INStringResolutionResult) -> Void) {
        recordResolve(intent: intent)
        completion(INStringResolutionResult.needsValue())
    }
}

@available(macOS 13.1, *)
private final class INXHangUpCallIntentHandlingBox: INXIntentHandlingHelperBase, INHangUpCallIntentHandling {
    override func simulateHandle() -> Bool {
        var completionCalled = false
        handle(intent: INHangUpCallIntent(callIdentifier: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateConfirm() -> Bool {
        var completionCalled = false
        confirm(intent: INHangUpCallIntent(callIdentifier: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    func handle(intent: INHangUpCallIntent, completion: @escaping (INHangUpCallIntentResponse) -> Void) {
        recordHandle(intent: intent)
        completion(INHangUpCallIntentResponse(code: .ready, userActivity: nil))
    }

    func confirm(intent: INHangUpCallIntent, completion: @escaping (INHangUpCallIntentResponse) -> Void) {
        recordConfirm(intent: intent)
        completion(INHangUpCallIntentResponse(code: .ready, userActivity: nil))
    }
}

@available(macOS 12.0, *)
private final class INXSendMessageIntentHandlingBox: INXIntentHandlingHelperBase, INSendMessageIntentHandling {
    private func makeIntent() -> INSendMessageIntent {
        INSendMessageIntent(
            recipients: nil,
            outgoingMessageType: .unknown,
            content: nil,
            speakableGroupName: nil,
            conversationIdentifier: nil,
            serviceName: nil,
            sender: nil,
            attachments: nil
        )
    }

    override func simulateHandle() -> Bool {
        var completionCalled = false
        handle(intent: makeIntent()) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateConfirm() -> Bool {
        var completionCalled = false
        confirm(intent: makeIntent()) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateResolve() -> Bool {
        var completionCalled = false
        resolveContent(for: makeIntent()) { _ in completionCalled = true }
        return completionCalled
    }

    func handle(intent: INSendMessageIntent, completion: @escaping (INSendMessageIntentResponse) -> Void) {
        recordHandle(intent: intent)
        completion(INSendMessageIntentResponse(code: .ready, userActivity: nil))
    }

    func confirm(intent: INSendMessageIntent, completion: @escaping (INSendMessageIntentResponse) -> Void) {
        recordConfirm(intent: intent)
        completion(INSendMessageIntentResponse(code: .ready, userActivity: nil))
    }

    func resolveContent(for intent: INSendMessageIntent, with completion: @escaping (INStringResolutionResult) -> Void) {
        recordResolve(intent: intent)
        completion(INStringResolutionResult.needsValue())
    }
}

@available(macOS 12.0, *)
private final class INXShareFocusStatusIntentHandlingBox: INXIntentHandlingHelperBase, INShareFocusStatusIntentHandling {
    override func simulateHandle() -> Bool {
        var completionCalled = false
        handle(intent: INShareFocusStatusIntent(focusStatus: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateConfirm() -> Bool {
        var completionCalled = false
        confirm(intent: INShareFocusStatusIntent(focusStatus: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    func handle(intent: INShareFocusStatusIntent, completion: @escaping (INShareFocusStatusIntentResponse) -> Void) {
        recordHandle(intent: intent)
        completion(INShareFocusStatusIntentResponse(code: .ready, userActivity: nil))
    }

    func confirm(intent: INShareFocusStatusIntent, completion: @escaping (INShareFocusStatusIntentResponse) -> Void) {
        recordConfirm(intent: intent)
        completion(INShareFocusStatusIntentResponse(code: .ready, userActivity: nil))
    }
}

@available(macOS 14.0, *)
private final class INXUnsendMessagesIntentHandlingBox: INXIntentHandlingHelperBase, INUnsendMessagesIntentHandling {
    override func simulateHandle() -> Bool {
        var completionCalled = false
        handle(intent: INUnsendMessagesIntent(messageIdentifiers: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    override func simulateConfirm() -> Bool {
        var completionCalled = false
        confirm(intent: INUnsendMessagesIntent(messageIdentifiers: nil)) { _ in completionCalled = true }
        return completionCalled
    }

    func handle(intent: INUnsendMessagesIntent, completion: @escaping (INUnsendMessagesIntentResponse) -> Void) {
        recordHandle(intent: intent)
        completion(INUnsendMessagesIntentResponse(code: .ready, userActivity: nil))
    }

    func confirm(intent: INUnsendMessagesIntent, completion: @escaping (INUnsendMessagesIntentResponse) -> Void) {
        recordConfirm(intent: intent)
        completion(INUnsendMessagesIntentResponse(code: .ready, userActivity: nil))
    }
}

private func inxMakeIntentHandlingHelper(kind: String) -> INXIntentHandlingHelperBase? {
    switch kind {
    case "answer_call":
        if #available(macOS 13.1, *) { return INXAnswerCallIntentHandlingBox() }
    case "edit_message":
        if #available(macOS 14.0, *) { return INXEditMessageIntentHandlingBox() }
    case "hang_up_call":
        if #available(macOS 13.1, *) { return INXHangUpCallIntentHandlingBox() }
    case "send_message":
        if #available(macOS 12.0, *) { return INXSendMessageIntentHandlingBox() }
    case "share_focus_status":
        if #available(macOS 12.0, *) { return INXShareFocusStatusIntentHandlingBox() }
    case "unsend_messages":
        if #available(macOS 14.0, *) { return INXUnsendMessagesIntentHandlingBox() }
    default:
        return nil
    }
    return nil
}

@_cdecl("inx_intent_handling_helper_create")
public func inx_intent_handling_helper_create(
    _ kind: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let kind else {
        outError?.pointee = inxCString("intent handling helper kind was NULL")
        return nil
    }
    let kindValue = String(cString: kind)
    guard let helper = inxMakeIntentHandlingHelper(kind: kindValue) else {
        outError?.pointee = inxCString("unsupported intent handling helper kind \(kindValue)")
        return nil
    }
    return inxRetain(helper)
}

@_cdecl("inx_intent_handling_helper_simulate_handle")
public func inx_intent_handling_helper_simulate_handle(
    _ helperPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let helperPtr else {
        outError?.pointee = inxCString("intent handling helper pointer was NULL")
        return false
    }
    let helper: INXIntentHandlingHelperBase = inxUnretained(helperPtr)
    let ok = helper.simulateHandle()
    if !ok { outError?.pointee = inxCString("intent handling helper did not invoke handle completion") }
    return ok
}

@_cdecl("inx_intent_handling_helper_simulate_confirm")
public func inx_intent_handling_helper_simulate_confirm(
    _ helperPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let helperPtr else {
        outError?.pointee = inxCString("intent handling helper pointer was NULL")
        return false
    }
    let helper: INXIntentHandlingHelperBase = inxUnretained(helperPtr)
    let ok = helper.simulateConfirm()
    if !ok { outError?.pointee = inxCString("intent handling helper did not invoke confirm completion") }
    return ok
}

@_cdecl("inx_intent_handling_helper_simulate_resolve")
public func inx_intent_handling_helper_simulate_resolve(
    _ helperPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let helperPtr else {
        outError?.pointee = inxCString("intent handling helper pointer was NULL")
        return false
    }
    let helper: INXIntentHandlingHelperBase = inxUnretained(helperPtr)
    let ok = helper.simulateResolve()
    if !ok { outError?.pointee = inxCString("intent handling helper did not invoke resolve completion") }
    return ok
}
