use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let activity = UserActivity::new("com.doomfish.intents-rs.relevant-shortcut")?;
    let shortcut = Shortcut::new_with_user_activity(&activity)?;
    let provider = RelevanceProvider::date(1_700_000_000.0, Some(1_700_000_060.0))?;
    let mut relevant = RelevantShortcut::new(&shortcut)?;
    relevant.set_shortcut_role(RelevantShortcutRole::Action)?;
    relevant.set_widget_kind("demo.widget")?;
    relevant.set_relevance_providers(&[&provider])?;

    println!("widget kind: {:?}", relevant.widget_kind());
    println!("provider count: {}", relevant.relevance_provider_count());
    println!("✅ INRelevantShortcut OK");
    Ok(())
}
