use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut intent = Intent::new()?;
    let mut metadata = SendMessageIntentDonationMetadata::new()?;
    metadata.set_mentions_current_user(true)?;
    metadata.set_reply_to_current_user(true)?;
    metadata.set_notify_recipient_anyway(false)?;
    metadata.set_recipient_count(2)?;
    intent.set_donation_metadata(Some(&metadata))?;

    println!(
        "donation metadata: {:?}",
        intent.donation_metadata().map(|value| value.class_name())
    );
    println!("✅ intent donation OK");
    Ok(())
}
