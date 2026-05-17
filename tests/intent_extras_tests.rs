use intents::prelude::*;

#[test]
fn intent_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let answer = AnswerCallIntent::new(CallAudioRoute::SpeakerphoneAudioRoute, Some("call-1"))?;
    let edit = EditMessageIntent::new(Some("message-1"), Some("updated"))?;
    let focus = FocusStatus::new(Some(true))?;
    let share = ShareFocusStatusIntent::new(Some(&focus))?;
    let reference = SpeakableString::new("ref", "Reference", None)?;
    let reservation = GetReservationDetailsIntent::new(Some(&reference), &[&reference])?;
    let unsend = UnsendMessagesIntent::new(&["message-1", "message-2"])?;
    let hang = HangUpCallIntent::new(Some("call-2"))?;

    assert_eq!(answer.call_identifier().as_deref(), Some("call-1"));
    assert_eq!(edit.edited_content().as_deref(), Some("updated"));
    assert!(share.focus_status().is_some());
    assert_eq!(reservation.reservation_item_references_count(), 1);
    assert_eq!(unsend.message_identifiers()?.unwrap_or_default().len(), 2);
    assert_eq!(hang.call_identifier().as_deref(), Some("call-2"));
    assert!(SendMessageIntent::supports_user_notifications()?);
    assert!(StartCallIntent::supports_user_notifications()?);
    Ok(())
}
