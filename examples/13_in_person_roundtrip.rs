use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = PersonHandle::new(Some("test@example.com"), PersonHandleType::EmailAddress)?;
    let alias = PersonHandle::new(Some("+1234567890"), PersonHandleType::PhoneNumber)?;
    let person = Person::with_details(
        &handle,
        Some("Test Person"),
        None,
        Some("contact-1"),
        Some("custom-1"),
        &[&alias],
        PersonSuggestionType::SocialProfile,
    )?;

    println!("person display: {:?}", person.display_name());
    println!("aliases: {}", person.aliases_count());
    println!("suggestion: {:?}", person.suggestion_type());
    println!("✅ INPerson OK");
    Ok(())
}
