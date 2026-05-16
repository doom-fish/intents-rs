use intents::prelude::*;

#[test]
fn parameter_index_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter = IntentParameter::new("INSendMessageIntent", "content")?;
    parameter.set_index_for_sub_key_path(1, "recipients")?;

    assert_eq!(parameter.parameter_class_name().as_deref(), Some("INSendMessageIntent"));
    assert_eq!(parameter.parameter_key_path().as_deref(), Some("content"));
    assert_eq!(parameter.index_for_sub_key_path("recipients")?, Some(1));
    assert!(parameter.is_equal_to_parameter(&parameter));
    Ok(())
}
