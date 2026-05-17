use intents::prelude::*;

#[test]
fn message_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = MessageLinkMetadata::new(
        Some("doom.fish"),
        Some("summary"),
        Some("title"),
        Some("article"),
        Some("https://doom.fish"),
    )?;
    let reaction = MessageReaction::new(MessageReactionType::Emoji, Some("Like"), Some("👍"))?;
    let file = IntentFile::from_data(b"audio-bytes", "voice.m4a", Some("public.mpeg-4-audio"))?;
    let attachment = SendMessageAttachment::audio_message_file(&file)?;
    let sticker = Sticker::new(StickerType::Emoji, Some("��"))?;

    assert_eq!(metadata.site_name().as_deref(), Some("doom.fish"));
    assert_eq!(reaction.reaction_type(), MessageReactionType::Emoji);
    assert!(attachment.audio_message_file_present());
    assert_eq!(sticker.sticker_type(), StickerType::Emoji);
    Ok(())
}
