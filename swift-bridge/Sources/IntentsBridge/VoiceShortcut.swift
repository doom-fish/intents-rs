import Foundation
import Intents

@_cdecl("inx_shortcut_create_with_intent")
public func inx_shortcut_create_with_intent(
    _ intentPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let intentPtr else {
        outError?.pointee = inxCString("intent pointer was NULL")
        return nil
    }

    let intent: INIntent = inxUnretained(intentPtr)
    guard let shortcut = INShortcut(intent: intent) else {
        outError?.pointee = inxCString("INShortcut rejected the provided intent")
        return nil
    }
    return inxRetain(shortcut as AnyObject)
}

@_cdecl("inx_voice_shortcut_center_shared")
public func inx_voice_shortcut_center_shared() -> UnsafeMutableRawPointer? {
    if #available(macOS 12.0, *) {
        return inxRetain(INVoiceShortcutCenter.shared)
    }
    return nil
}

@_cdecl("inx_voice_shortcut_center_get_all")
public func inx_voice_shortcut_center_get_all(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ callback: @escaping INXArrayCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ArrayCallbackBox(callback: callback, refcon: refcon)
    guard let centerPtr else {
        box.send(objects: nil, errorMessage: "voice shortcut center pointer was NULL")
        return
    }

    guard #available(macOS 12.0, *) else {
        box.send(objects: nil, errorMessage: "INVoiceShortcutCenter requires macOS 12+")
        return
    }

    let center: INVoiceShortcutCenter = inxUnretained(centerPtr)
    center.getAllVoiceShortcuts { shortcuts, error in
        box.send(objects: shortcuts, errorMessage: error?.localizedDescription)
    }
}

@_cdecl("inx_voice_shortcut_center_get_by_identifier")
public func inx_voice_shortcut_center_get_by_identifier(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ callback: @escaping INXObjectCallback,
    _ refcon: UnsafeMutableRawPointer?
) {
    let box = ObjectCallbackBox(callback: callback, refcon: refcon)
    guard let centerPtr else {
        box.send(object: nil, errorMessage: "voice shortcut center pointer was NULL")
        return
    }
    guard let identifier else {
        box.send(object: nil, errorMessage: "voice shortcut identifier was NULL")
        return
    }
    guard let uuid = UUID(uuidString: String(cString: identifier)) else {
        box.send(object: nil, errorMessage: "voice shortcut identifier was not a valid UUID")
        return
    }

    guard #available(macOS 12.0, *) else {
        box.send(object: nil, errorMessage: "INVoiceShortcutCenter requires macOS 12+")
        return
    }

    let center: INVoiceShortcutCenter = inxUnretained(centerPtr)
    center.getVoiceShortcut(with: uuid) { shortcut, error in
        box.send(object: shortcut, errorMessage: error?.localizedDescription)
    }
}
