use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut response = IntentResponse::new()?;
    response.set_user_activity_type("com.doomfish.intents-rs.intent-response")?;

    println!("response class: {}", response.class_name());
    println!("activity type: {:?}", response.user_activity_type());
    println!("✅ intent response OK");
    Ok(())
}
