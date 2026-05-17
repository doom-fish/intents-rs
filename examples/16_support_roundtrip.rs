use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let placemark = Placemark::new(Some("Test Place"))?;
    let item = IntentObject::new(Some("one"), "One")?;
    let section = ObjectSection::new(Some("Section"), &[&item])?;
    let mut collection = ObjectCollection::new_with_sections(&[&section])?;
    collection.set_uses_indexed_collation(true)?;

    println!("placemark: {:?}", placemark.name());
    println!("section title: {:?}", section.title());
    println!("sections: {}", collection.sections_count());
    println!("all items: {}", collection.all_items_count());
    println!("localized: {}", deferred_localized_intents_string("Hello")?);
    println!("version: {} / {}", intents_version_number(), intents_version_string());
    println!("✅ support OK");
    Ok(())
}
