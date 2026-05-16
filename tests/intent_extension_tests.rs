use intents::prelude::*;

#[test]
fn default_extension_has_no_handler_for_base_intent() -> Result<(), Box<dyn std::error::Error>> {
    let extension = IntentExtension::new()?;
    let intent = Intent::new()?;

    assert_eq!(extension.class_name(), "INExtension");
    assert!(extension.handler_class_name_for_intent(&intent).is_none());
    Ok(())
}
