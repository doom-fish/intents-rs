use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let sticker = Sticker::new(StickerType::Emoji, Some("😀"))?;

    println!("site: {:?}", metadata.site_name());
    println!("reaction: {:?}", reaction.reaction_type());
    println!("attachment has audio: {}", attachment.audio_message_file_present());
    println!("sticker: {:?} {:?}", sticker.sticker_type(), sticker.emoji());
    println!("✅ message OK");
    Ok(())
}
