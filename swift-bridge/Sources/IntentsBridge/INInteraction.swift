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

@_cdecl("inx_interaction_delete_all")
public func inx_interaction_delete_all(
    _ callback: @escaping INXErrorCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ErrorCallbackBox(callback: callback, refcon: refcon)
    INInteraction.deleteAll { error in
        box.send(errorMessage: error?.localizedDescription)
    }
}

@_cdecl("inx_interaction_delete_by_identifiers")
public func inx_interaction_delete_by_identifiers(
    _ identifiers: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ callback: @escaping INXErrorCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ErrorCallbackBox(callback: callback, refcon: refcon)
    guard count >= 0 else {
        box.send(errorMessage: "interaction identifier count must be non-negative")
        return
    }

    let values: [String]
    if count == 0 {
        values = []
    } else if let identifiers {
        values = (0..<count).compactMap { index in
            identifiers[index].map(String.init(cString:))
        }
        guard values.count == count else {
            box.send(errorMessage: "interaction identifiers contained NULL entries")
            return
        }
    } else {
        box.send(errorMessage: "interaction identifiers pointer was NULL")
        return
    }

    INInteraction.delete(with: values) { error in
        box.send(errorMessage: error?.localizedDescription)
    }
}

@_cdecl("inx_interaction_delete_by_group_identifier")
public func inx_interaction_delete_by_group_identifier(
    _ groupIdentifier: UnsafePointer<CChar>?,
    _ callback: @escaping INXErrorCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ErrorCallbackBox(callback: callback, refcon: refcon)
    guard let groupIdentifier else {
        box.send(errorMessage: "interaction group identifier was NULL")
        return
    }

    INInteraction.delete(with: String(cString: groupIdentifier)) { error in
        box.send(errorMessage: error?.localizedDescription)
    }
}
