use intents::prelude::*;

#[test]
fn relevant_shortcut_store_accepts_empty_updates() -> Result<(), Box<dyn std::error::Error>> {
    let store = RelevantShortcutStore::default_store()?;
    store.set_relevant_shortcuts(&[])?;
    Ok(())
}
