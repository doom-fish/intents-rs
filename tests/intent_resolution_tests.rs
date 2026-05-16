use intents::prelude::*;

#[test]
fn resolution_result_factories_return_objects() -> Result<(), Box<dyn std::error::Error>> {
    let needs_value = IntentResolutionResult::needs_value()?;
    let not_required = IntentResolutionResult::not_required()?;
    let unsupported = IntentResolutionResult::unsupported()?;

    assert_eq!(needs_value.class_name(), "INIntentResolutionResult");
    assert_eq!(not_required.class_name(), "INIntentResolutionResult");
    assert_eq!(unsupported.class_name(), "INIntentResolutionResult");
    Ok(())
}
