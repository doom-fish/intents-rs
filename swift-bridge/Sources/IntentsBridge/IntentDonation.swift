import Foundation
import Intents

@_cdecl("inx_send_message_intent_donation_metadata_create")
public func inx_send_message_intent_donation_metadata_create(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INSendMessageIntentDonationMetadata requires macOS 12+")
        return nil
    }

    return inxRetain(INSendMessageIntentDonationMetadata())
}
