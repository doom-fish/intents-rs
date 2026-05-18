use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let answer = AnswerCallIntent::new(CallAudioRoute::SpeakerphoneAudioRoute, Some("call-1"))?;
    let edit = EditMessageIntent::new(Some("message-1"), Some("updated"))?;
    let focus = FocusStatus::new(Some(true))?;
    let share = ShareFocusStatusIntent::new(Some(&focus))?;
    let reference = SpeakableString::new("ref", "Reference", None)?;
    let reservation = GetReservationDetailsIntent::new(Some(&reference), &[&reference])?;
    let unsend = UnsendMessagesIntent::new(&["message-1", "message-2"])?;
    let hang = HangUpCallIntent::new(Some("call-2"))?;

    println!("answer call id: {:?}", answer.call_identifier());
    println!("edit content: {:?}", edit.edited_content());
    println!(
        "share focused: {:?}",
        share.focus_status().and_then(|value| value.is_focused())
    );
    println!(
        "reservation refs: {}",
        reservation.reservation_item_references_count()
    );
    println!("unsend ids: {:?}", unsend.message_identifiers()?);
    println!("hang id: {:?}", hang.call_identifier());
    println!(
        "user notifications: send={} start={}",
        SendMessageIntent::supports_user_notifications()?,
        StartCallIntent::supports_user_notifications()?
    );
    println!("✅ intent extras OK");
    Ok(())
}
