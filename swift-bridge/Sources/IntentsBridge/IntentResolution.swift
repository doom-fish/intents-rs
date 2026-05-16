import Foundation
import Intents

@_cdecl("inx_intent_resolution_result_needs_value")
public func inx_intent_resolution_result_needs_value() -> UnsafeMutableRawPointer? {
    inxRetain(INIntentResolutionResult.needsValue())
}

@_cdecl("inx_intent_resolution_result_not_required")
public func inx_intent_resolution_result_not_required() -> UnsafeMutableRawPointer? {
    inxRetain(INIntentResolutionResult.notRequired())
}

@_cdecl("inx_intent_resolution_result_unsupported")
public func inx_intent_resolution_result_unsupported() -> UnsafeMutableRawPointer? {
    inxRetain(INIntentResolutionResult.unsupported())
}
