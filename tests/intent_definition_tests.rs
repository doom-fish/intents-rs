use intents::prelude::*;

#[test]
fn intent_definition_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let mut intent = Intent::new()?;
    intent.set_suggested_invocation_phrase("Test shortcut")?;

    let activity = UserActivity::new("com.doomfish.intents-rs.tests.intent-definition")?;
    let shortcut = Shortcut::new_with_user_activity(&activity)?;

    assert_eq!(intent.class_name(), "INIntent");
    assert_eq!(
        shortcut.user_activity_type().as_deref(),
        Some("com.doomfish.intents-rs.tests.intent-definition")
    );
    Ok(())
}
