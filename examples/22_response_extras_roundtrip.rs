use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut activity = UserActivity::new("com.doomfish.intents-rs.response-extras")?;
    activity.set_suggested_invocation_phrase("Reply now")?;

    let answer =
        AnswerCallIntentResponse::new(AnswerCallIntentResponseCode::Ready, Some(&activity))?;
    let edit =
        EditMessageIntentResponse::new(EditMessageIntentResponseCode::Success, Some(&activity))?;
    let reservation = GetReservationDetailsIntentResponse::new(
        GetReservationDetailsIntentResponseCode::Ready,
        Some(&activity),
    )?;
    let hang =
        HangUpCallIntentResponse::new(HangUpCallIntentResponseCode::Success, Some(&activity))?;
    let focus = ShareFocusStatusIntentResponse::new(
        ShareFocusStatusIntentResponseCode::Ready,
        Some(&activity),
    )?;
    let unsend = UnsendMessagesIntentResponse::new(
        UnsendMessagesIntentResponseCode::InProgress,
        Some(&activity),
    )?;

    println!("phrase: {:?}", activity.suggested_invocation_phrase());
    println!("answer: {:?}", answer.code());
    println!("edit: {:?}", edit.code());
    println!("reservation: {:?}", reservation.code());
    println!("hang: {:?}", hang.code());
    println!("focus: {:?}", focus.code());
    println!("unsend: {:?}", unsend.code());
    println!("✅ response extras OK");
    Ok(())
}
