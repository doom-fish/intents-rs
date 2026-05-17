use intents::prelude::*;

#[test]
fn response_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let mut activity = UserActivity::new("com.doomfish.intents-rs.response-extras")?;
    activity.set_suggested_invocation_phrase("Reply now")?;

    let answer = AnswerCallIntentResponse::new(AnswerCallIntentResponseCode::Ready, Some(&activity))?;
    let edit = EditMessageIntentResponse::new(EditMessageIntentResponseCode::Success, Some(&activity))?;
    let reservation =
        GetReservationDetailsIntentResponse::new(GetReservationDetailsIntentResponseCode::Ready, Some(&activity))?;
    let hang = HangUpCallIntentResponse::new(HangUpCallIntentResponseCode::Success, Some(&activity))?;
    let focus = ShareFocusStatusIntentResponse::new(ShareFocusStatusIntentResponseCode::Ready, Some(&activity))?;
    let unsend = UnsendMessagesIntentResponse::new(UnsendMessagesIntentResponseCode::InProgress, Some(&activity))?;

    assert_eq!(activity.suggested_invocation_phrase().as_deref(), Some("Reply now"));
    assert_eq!(answer.code(), AnswerCallIntentResponseCode::Ready);
    assert_eq!(edit.code(), EditMessageIntentResponseCode::Success);
    assert_eq!(reservation.code(), GetReservationDetailsIntentResponseCode::Ready);
    assert_eq!(hang.code(), HangUpCallIntentResponseCode::Success);
    assert_eq!(focus.code(), ShareFocusStatusIntentResponseCode::Ready);
    assert_eq!(unsend.code(), UnsendMessagesIntentResponseCode::InProgress);
    Ok(())
}
