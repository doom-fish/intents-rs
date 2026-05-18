use intents::prelude::*;

#[test]
fn send_message_donation_metadata_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata = SendMessageIntentDonationMetadata::new()?;
    metadata.set_mentions_current_user(true)?;
    metadata.set_reply_to_current_user(true)?;
    metadata.set_notify_recipient_anyway(false)?;
    metadata.set_recipient_count(3)?;

    let mut intent = Intent::new()?;
    intent.set_donation_metadata(Some(&metadata))?;
    let metadata = intent
        .donation_metadata()
        .expect("metadata should be attached");

    assert_eq!(
        metadata.class_name(),
        SendMessageIntentDonationMetadata::OBJC_CLASS
    );
    Ok(())
}
