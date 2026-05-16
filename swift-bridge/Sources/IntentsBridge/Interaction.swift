import Foundation
import Intents

@_cdecl("inx_interaction_create")
public func inx_interaction_create(
    _ intentPtr: UnsafeMutableRawPointer?,
    _ responsePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let intentPtr else {
        outError?.pointee = inxCString("intent pointer was NULL")
        return nil
    }

    let intent: INIntent = inxUnretained(intentPtr)
    let response: INIntentResponse? = responsePtr.map { inxUnretained($0) }
    let interaction = INInteraction(intent: intent, response: response)
    return inxRetain(interaction)
}

@_cdecl("inx_interaction_donate")
public func inx_interaction_donate(
    _ interactionPtr: UnsafeMutableRawPointer?,
    _ callback: @escaping INXErrorCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ErrorCallbackBox(callback: callback, refcon: refcon)
    guard let interactionPtr else {
        box.send(errorMessage: "interaction pointer was NULL")
        return
    }

    let interaction: INInteraction = inxUnretained(interactionPtr)
    interaction.donate { error in
        box.send(errorMessage: error?.localizedDescription)
    }
}
