use intents::prelude::*;

#[test]
fn intent_handler_provider_maps_class_names() -> Result<(), Box<dyn std::error::Error>> {
    let intent = Intent::new()?;
    let mut provider = IntentHandlerProvider::new()?;
    provider.register_handler("INIntent", "ExampleIntentHandler")?;

    assert_eq!(
        provider.handler_name_for_intent(&intent)?.as_deref(),
        Some("ExampleIntentHandler")
    );
    Ok(())
}

#[test]
fn start_call_intent_handling_helper_records_callbacks() -> Result<(), Box<dyn std::error::Error>> {
    let mut handling = StartCallIntentHandling::new()?;
    handling.simulate_handle()?;
    handling.simulate_confirm()?;

    assert!(handling
        .class_name()
        .contains("INXStartCallIntentHandlingBox"));
    assert_eq!(handling.handle_call_count(), 1);
    assert_eq!(handling.confirm_call_count(), 1);
    assert!(handling
        .last_intent_class_name()
        .as_deref()
        .is_some_and(|value| value.ends_with("INStartCallIntent")));
    Ok(())
}
