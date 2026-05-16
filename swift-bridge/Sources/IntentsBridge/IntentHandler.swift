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

@available(macOS 12.0, *)
private final class INXStartCallIntentHandlingBox: NSObject {
    @objc dynamic private(set) var handleCallCount = 0
    @objc dynamic private(set) var confirmCallCount = 0
    @objc dynamic private(set) var lastIntentClassName: String?

    private func makeIntent() -> INStartCallIntent {
        INStartCallIntent(
            callRecordFilter: nil,
            callRecordToCallBack: nil,
            audioRoute: .unknown,
            destinationType: .unknown,
            contacts: nil,
            callCapability: .unknown
        )
    }

    func simulateHandle() -> Bool {
        var completionCalled = false
        let intent = makeIntent()
        handleStartCall(intent) { _ in completionCalled = true }
        return completionCalled
    }

    func simulateConfirm() -> Bool {
        var completionCalled = false
        let intent = makeIntent()
        confirmStartCall(intent) { _ in completionCalled = true }
        return completionCalled
    }

    func handleStartCall(_ intent: INStartCallIntent, completion: @escaping (AnyObject?) -> Void) {
        handleCallCount += 1
        lastIntentClassName = NSStringFromClass(type(of: intent))
        completion(nil)
    }

    func confirmStartCall(_ intent: INStartCallIntent, completion: @escaping (AnyObject?) -> Void) {
        confirmCallCount += 1
        lastIntentClassName = NSStringFromClass(type(of: intent))
        completion(nil)
    }
}

@_cdecl("inx_start_call_intent_handling_create")
public func inx_start_call_intent_handling_create(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INStartCallIntentHandling requires macOS 12+")
        return nil
    }

    return inxRetain(INXStartCallIntentHandlingBox())
}

@_cdecl("inx_start_call_intent_handling_simulate_handle")
public func inx_start_call_intent_handling_simulate_handle(
    _ helperPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INStartCallIntentHandling requires macOS 12+")
        return false
    }
    guard let helperPtr else {
        outError?.pointee = inxCString("start-call intent handling helper pointer was NULL")
        return false
    }

    let helper: INXStartCallIntentHandlingBox = inxUnretained(helperPtr)
    let completionCalled = helper.simulateHandle()
    if !completionCalled {
        outError?.pointee = inxCString("start-call intent handling did not invoke completion")
    }
    return completionCalled
}

@_cdecl("inx_start_call_intent_handling_simulate_confirm")
public func inx_start_call_intent_handling_simulate_confirm(
    _ helperPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INStartCallIntentHandling requires macOS 12+")
        return false
    }
    guard let helperPtr else {
        outError?.pointee = inxCString("start-call intent handling helper pointer was NULL")
        return false
    }

    let helper: INXStartCallIntentHandlingBox = inxUnretained(helperPtr)
    let completionCalled = helper.simulateConfirm()
    if !completionCalled {
        outError?.pointee = inxCString("start-call intent confirmation did not invoke completion")
    }
    return completionCalled
}
