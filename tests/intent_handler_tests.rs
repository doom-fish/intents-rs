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
