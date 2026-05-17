use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unsupported = IntentResolutionResult::unsupported_with_reason(7)?;
    let recipient = SendMessageRecipientResolutionResult::unsupported_for_reason(
        SendMessageRecipientUnsupportedReason::NoAccount,
    )?;
    let callback = StartCallCallRecordToCallBackResolutionResult::unsupported_for_reason(
        StartCallCallRecordToCallBackUnsupportedReason::NoMatchingCall,
    )?;
    let object = IntentObject::new(Some("id"), "Object")?;
    let confirmation = IntentResolutionResult::confirmation_required_with_item_for_reason(Some(&object), 42)?;

    println!("unsupported: {}", unsupported.class_name());
    println!("recipient: {}", recipient.class_name());
    println!("callback: {}", callback.class_name());
    println!("confirmation: {}", confirmation.class_name());
    println!("✅ resolution extras OK");
    Ok(())
}
