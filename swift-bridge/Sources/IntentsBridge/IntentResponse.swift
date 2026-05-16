import Foundation
import Intents

@_cdecl("inx_intent_response_create")
public func inx_intent_response_create() -> UnsafeMutableRawPointer? {
    inxRetain(INIntentResponse())
}
