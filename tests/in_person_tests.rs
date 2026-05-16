use intents::prelude::*;

#[test]
fn person_roundtrip_with_aliases() -> Result<(), Box<dyn std::error::Error>> {
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

    let person_handle = person
        .person_handle()
        .expect("person handle should roundtrip");
    assert_eq!(person.class_name(), "INPerson");
    assert_eq!(person.display_name().as_deref(), Some("Test Person"));
    assert_eq!(person.contact_identifier().as_deref(), Some("contact-1"));
    assert_eq!(person.custom_identifier().as_deref(), Some("custom-1"));
    assert_eq!(person_handle.value().as_deref(), Some("test@example.com"));
    assert_eq!(person_handle.handle_type(), PersonHandleType::EmailAddress);
    assert_eq!(person.aliases_count(), 1);
    assert_eq!(
        person.suggestion_type(),
        PersonSuggestionType::SocialProfile
    );
    assert!(!person.is_me());
    Ok(())
}
