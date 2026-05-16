use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = RelevantShortcutStore::default_store()?;
    store.set_relevant_shortcuts(&[])?;

    println!("✅ INRelevantShortcutStore OK");
    Ok(())
}
