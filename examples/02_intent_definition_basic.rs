use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut intent = Intent::new()?;
    intent.set_suggested_invocation_phrase("Open demo shortcut")?;

    let activity = UserActivity::new("com.doomfish.intents-rs.intent-definition")?;
    let shortcut = Shortcut::new_with_user_activity(&activity)?;

    println!("intent class: {}", intent.class_name());
    println!("shortcut activity: {:?}", shortcut.user_activity_type());
    println!("✅ intent definition OK");
    Ok(())
}
