use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = FocusStatus::new(Some(true))?;
    let center = FocusStatusCenter::default_center()?;

    println!("focus status: {:?}", status.is_focused());
    println!("focus center class: {}", center.class_name());
    println!("✅ focus OK");
    Ok(())
}
