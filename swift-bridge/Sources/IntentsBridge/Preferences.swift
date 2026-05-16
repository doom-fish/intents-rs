import Foundation
import Intents

@_cdecl("inx_preferences_siri_authorization_status")
public func inx_preferences_siri_authorization_status() -> Int64 {
    Int64(inxClassIntMethod("INPreferences", selectorName: "siriAuthorizationStatus") ?? -1)
}

@_cdecl("inx_preferences_request_siri_authorization")
public func inx_preferences_request_siri_authorization(
    _ callback: @escaping INXStatusCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    guard let cls = inxClass(named: "INPreferences"), let metaClass = object_getClass(cls) else {
        StatusCallbackBox(callback: callback, refcon: refcon).send(
            status: -1,
            errorMessage: "INPreferences is unavailable on this system"
        )
        return
    }

    let selector = NSSelectorFromString("requestSiriAuthorization:")
    guard class_respondsToSelector(metaClass, selector) else {
        StatusCallbackBox(callback: callback, refcon: refcon).send(
            status: -1,
            errorMessage: "INPreferences.requestSiriAuthorization is unavailable"
        )
        return
    }

    let box = StatusCallbackBox(callback: callback, refcon: refcon)
    typealias Fn = @convention(c) (AnyClass, Selector, @convention(block) (Int) -> Void) -> Void
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    let block: @convention(block) (Int) -> Void = { status in
        box.send(status: Int64(status))
    }
    function(cls, selector, block)
}
