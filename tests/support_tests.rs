use intents::prelude::*;

#[test]
fn support_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let placemark = Placemark::new(Some("Test Place"))?;
    let item = IntentObject::new(Some("one"), "One")?;
    let section = ObjectSection::new(Some("Section"), &[&item])?;
    let mut collection = ObjectCollection::new_with_sections(&[&section])?;
    collection.set_uses_indexed_collation(true)?;

    assert_eq!(placemark.name().as_deref(), Some("Test Place"));
    assert_eq!(section.title().as_deref(), Some("Section"));
    assert_eq!(collection.sections_count(), 1);
    assert_eq!(collection.all_items_count(), 1);
    assert!(collection.uses_indexed_collation());
    assert!(intents_version_number() > 0.0);
    assert!(!intents_version_string().is_empty());
    assert!(deferred_localized_intents_string("Hello").is_ok());
    Ok(())
}
