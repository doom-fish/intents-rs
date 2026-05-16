use intents::prelude::*;

#[test]
fn intent_response_user_activity_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let mut response = IntentResponse::new()?;
    response.set_user_activity_type("com.doomfish.intents-rs.tests.intent-response")?;

    assert_eq!(response.class_name(), "INIntentResponse");
    assert_eq!(
        response.user_activity_type().as_deref(),
        Some("com.doomfish.intents-rs.tests.intent-response")
    );
    Ok(())
}

#[test]
fn send_message_intent_response_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let activity = UserActivity::new("com.doomfish.intents-rs.tests.send-message-response")?;
    let response =
        SendMessageIntentResponse::new(SendMessageIntentResponseCode::Ready, Some(&activity))?;

    assert_eq!(response.class_name(), SendMessageIntentResponse::OBJC_CLASS);
    assert_eq!(response.code(), SendMessageIntentResponseCode::Ready);
    assert_eq!(
        response.user_activity_type().as_deref(),
        Some("com.doomfish.intents-rs.tests.send-message-response")
    );

    let erased: IntentResponse = response.into();
    let typed = SendMessageIntentResponse::try_from(erased)?;
    assert_eq!(typed.code(), SendMessageIntentResponseCode::Ready);
    Ok(())
}
