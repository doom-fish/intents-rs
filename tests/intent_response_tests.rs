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
