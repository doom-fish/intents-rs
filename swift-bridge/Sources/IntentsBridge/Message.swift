import Foundation
import Intents

@_cdecl("inx_message_link_metadata_create")
public func inx_message_link_metadata_create(
    _ siteName: UnsafePointer<CChar>?,
    _ summary: UnsafePointer<CChar>?,
    _ title: UnsafePointer<CChar>?,
    _ openGraphType: UnsafePointer<CChar>?,
    _ linkURL: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 14.0, *) else {
        outError?.pointee = inxCString("INMessageLinkMetadata requires macOS 14+")
        return nil
    }

    let url = linkURL.flatMap { URL(string: String(cString: $0)) }
    if linkURL != nil && url == nil {
        outError?.pointee = inxCString("message link metadata URL was invalid")
        return nil
    }

    let metadata = INMessageLinkMetadata(
        siteName: siteName.map { String(cString: $0) },
        summary: summary.map { String(cString: $0) },
        title: title.map { String(cString: $0) },
        openGraphType: openGraphType.map { String(cString: $0) },
        linkURL: url
    )
    return inxRetain(metadata)
}

@_cdecl("inx_message_reaction_create")
public func inx_message_reaction_create(
    _ reactionType: Int,
    _ reactionDescription: UnsafePointer<CChar>?,
    _ emoji: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.0, *) else {
        outError?.pointee = inxCString("INMessageReaction requires macOS 15+")
        return nil
    }
    guard let reactionType = INMessageReactionType(rawValue: reactionType) else {
        outError?.pointee = inxCString("invalid INMessageReactionType raw value")
        return nil
    }

    let reaction = INMessageReaction(
        reactionType: reactionType,
        reactionDescription: reactionDescription.map { String(cString: $0) },
        emoji: emoji.map { String(cString: $0) }
    )
    return inxRetain(reaction)
}

@_cdecl("inx_send_message_attachment_create_with_audio_file")
public func inx_send_message_attachment_create_with_audio_file(
    _ filePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INSendMessageAttachment requires macOS 12+")
        return nil
    }
    guard let filePtr else {
        outError?.pointee = inxCString("send-message attachment file pointer was NULL")
        return nil
    }

    let file: INFile = inxUnretained(filePtr)
    let selector = NSSelectorFromString("attachmentWithAudioMessageFile:")
    guard let metaClass = object_getClass(INSendMessageAttachment.self), class_respondsToSelector(metaClass, selector) else {
        outError?.pointee = inxCString("INSendMessageAttachment attachmentWithAudioMessageFile: was unavailable")
        return nil
    }
    typealias Fn = @convention(c) (AnyClass, Selector, INFile) -> AnyObject
    let imp = class_getMethodImplementation(metaClass, selector)
    let function = unsafeBitCast(imp, to: Fn.self)
    return inxRetain(function(INSendMessageAttachment.self, selector, file))
}

@_cdecl("inx_sticker_create")
public func inx_sticker_create(
    _ stickerType: Int,
    _ emoji: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.0, *) else {
        outError?.pointee = inxCString("INSticker requires macOS 15+")
        return nil
    }
    guard let stickerType = INSticker.StickerType(rawValue: stickerType) else {
        outError?.pointee = inxCString("invalid INStickerType raw value")
        return nil
    }

    return inxRetain(INSticker(type: stickerType, emoji: emoji.map { String(cString: $0) }))
}
