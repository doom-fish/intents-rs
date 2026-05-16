use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let speakable = SpeakableString::new("demo-vocab", "Demo Object", Some("Demo"))?;
    let mut object = IntentObject::new(Some("demo-object"), "Demo Object")?;
    object.set_subtitle_string("Example subtitle")?;
    object.set_alternative_speakable_matches(&[&speakable])?;

    println!("object display: {:?}", object.display_string());
    println!("object subtitle: {:?}", object.subtitle_string());
    println!(
        "speakable count: {}",
        object.alternative_speakable_matches_count()
    );
    println!("✅ INObject OK");
    Ok(())
}
