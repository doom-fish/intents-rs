use intents::prelude::*;

#[test]
fn object_and_speakable_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let speakable = SpeakableString::new("demo-object", "Demo Object", Some("Demo"))?;
    let mut object = IntentObject::new(Some("demo-object"), "Demo Object")?;
    object.set_subtitle_string("Example subtitle")?;
    object.set_alternative_speakable_matches(&[&speakable])?;

    assert_eq!(object.identifier().as_deref(), Some("demo-object"));
    assert_eq!(object.display_string().as_deref(), Some("Demo Object"));
    assert_eq!(object.subtitle_string().as_deref(), Some("Example subtitle"));
    assert_eq!(object.alternative_speakable_matches_count(), 1);
    Ok(())
}
