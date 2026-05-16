use intents::prelude::*;

#[test]
fn relevant_shortcut_accepts_providers() -> Result<(), Box<dyn std::error::Error>> {
    let activity = UserActivity::new("com.doomfish.intents-rs.tests.relevant-shortcut")?;
    let shortcut = Shortcut::new_with_user_activity(&activity)?;
    let provider = RelevanceProvider::date(1_700_000_000.0, Some(1_700_000_060.0))?;
    let mut relevant = RelevantShortcut::new(&shortcut)?;
    relevant.set_relevance_providers(&[&provider])?;
    relevant.set_shortcut_role(RelevantShortcutRole::Information)?;

    assert_eq!(relevant.relevance_provider_count(), 1);
    assert!(matches!(
        relevant.shortcut_role(),
        RelevantShortcutRole::Information
    ));
    Ok(())
}
