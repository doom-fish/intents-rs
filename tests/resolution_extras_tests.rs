use intents::prelude::*;

#[test]
fn resolution_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let unsupported = IntentResolutionResult::unsupported_with_reason(7)?;
    let recipient = SendMessageRecipientResolutionResult::unsupported_for_reason(
        SendMessageRecipientUnsupportedReason::NoAccount,
    )?;
    let callback = StartCallCallRecordToCallBackResolutionResult::unsupported_for_reason(
        StartCallCallRecordToCallBackUnsupportedReason::NoMatchingCall,
    )?;
    let object = IntentObject::new(Some("id"), "Object")?;
    let confirmation =
        IntentResolutionResult::confirmation_required_with_item_for_reason(Some(&object), 42)?;

    assert_eq!(unsupported.class_name(), "INIntentResolutionResult");
    assert_eq!(
        recipient.class_name(),
        SendMessageRecipientResolutionResult::OBJC_CLASS
    );
    assert_eq!(
        callback.class_name(),
        StartCallCallRecordToCallBackResolutionResult::OBJC_CLASS
    );
    assert_eq!(confirmation.class_name(), "INIntentResolutionResult");
    Ok(())
}
