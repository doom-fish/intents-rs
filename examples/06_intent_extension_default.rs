use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extension = IntentExtension::new()?;
    let intent = Intent::new()?;

    println!("extension class: {}", extension.class_name());
    println!(
        "default handler: {:?}",
        extension.handler_class_name_for_intent(&intent)
    );
    println!("✅ intent extension OK");
    Ok(())
}
