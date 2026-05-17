import Foundation
import Intents

@_cdecl("inx_answer_call_intent_create")
public func inx_answer_call_intent_create(
    _ audioRoute: Int,
    _ callIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.1, *) else {
        outError?.pointee = inxCString("INAnswerCallIntent requires macOS 13.1+")
        return nil
    }
    guard let audioRoute = INCallAudioRoute(rawValue: audioRoute) else {
        outError?.pointee = inxCString("invalid INCallAudioRoute raw value")
        return nil
    }
    return inxRetain(INAnswerCallIntent(
        audioRoute: audioRoute,
        callIdentifier: callIdentifier.map { String(cString: $0) }
    ))
}

@_cdecl("inx_edit_message_intent_create")
public func inx_edit_message_intent_create(
    _ messageIdentifier: UnsafePointer<CChar>?,
    _ editedContent: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = inxCString("INEditMessageIntent requires macOS 14+")
        return nil
    }
    return inxRetain(INEditMessageIntent(
        messageIdentifier: messageIdentifier.map { String(cString: $0) },
        editedContent: editedContent.map { String(cString: $0) }
    ))
}

@_cdecl("inx_get_reservation_details_intent_create")
public func inx_get_reservation_details_intent_create(
    _ reservationContainerReferencePtr: UnsafeMutableRawPointer?,
    _ reservationItemReferences: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let reservationContainerReference: INSpeakableString? = reservationContainerReferencePtr.map { inxUnretained($0) }
    let itemReferences: [INSpeakableString]
    if count == 0 {
        itemReferences = []
    } else if let reservationItemReferences {
        itemReferences = (0..<count).compactMap { index -> INSpeakableString? in
            guard let value = reservationItemReferences[index] else { return nil }
            return Unmanaged<INSpeakableString>.fromOpaque(value).takeUnretainedValue()
        }
        guard itemReferences.count == count else {
            outError?.pointee = inxCString("reservation item references were invalid")
            return nil
        }
    } else {
        outError?.pointee = inxCString("reservation item references were NULL")
        return nil
    }

    return inxRetain(INGetReservationDetailsIntent(
        reservationContainerReference: reservationContainerReference,
        reservationItemReferences: itemReferences.isEmpty ? nil : itemReferences
    ))
}

@_cdecl("inx_hang_up_call_intent_create")
public func inx_hang_up_call_intent_create(
    _ callIdentifier: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 13.1, *) else {
        outError?.pointee = inxCString("INHangUpCallIntent requires macOS 13.1+")
        return nil
    }
    return inxRetain(INHangUpCallIntent(callIdentifier: callIdentifier.map { String(cString: $0) }))
}

@_cdecl("inx_share_focus_status_intent_create")
public func inx_share_focus_status_intent_create(
    _ focusStatusPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INShareFocusStatusIntent requires macOS 12+")
        return nil
    }
    let focusStatus: INFocusStatus? = focusStatusPtr.map { inxUnretained($0) }
    return inxRetain(INShareFocusStatusIntent(focusStatus: focusStatus))
}

@_cdecl("inx_unsend_messages_intent_create")
public func inx_unsend_messages_intent_create(
    _ messageIdentifiers: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = inxCString("INUnsendMessagesIntent requires macOS 14+")
        return nil
    }
    let identifiers: [String]
    if count == 0 {
        identifiers = []
    } else if let messageIdentifiers {
        identifiers = (0..<count).compactMap { index -> String? in
            guard let value = messageIdentifiers[index] else { return nil }
            return String(cString: value)
        }
        guard identifiers.count == count else {
            outError?.pointee = inxCString("unsend message identifiers were invalid")
            return nil
        }
    } else {
        outError?.pointee = inxCString("unsend message identifiers were NULL")
        return nil
    }

    return inxRetain(INUnsendMessagesIntent(messageIdentifiers: identifiers.isEmpty ? nil : identifiers))
}
