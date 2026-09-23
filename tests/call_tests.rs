use intents::prelude::*;

#[test]
fn call_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let group = CallGroup::new(Some("Friends"), Some("group-1"))?;
    let record = CallRecord::new(
        "call-1",
        CallRecordType::Outgoing,
        CallCapability::AudioCall,
    )?;
    let handle = PersonHandle::new(Some("friend@example.com"), PersonHandleType::EmailAddress)?;
    let person = Person::new(&handle, Some("Friend"))?;
    let filter = CallRecordFilter::new(
        &[&person],
        CallRecordTypeOptions::OUTGOING,
        CallCapability::AudioCall,
    )?;

    assert_eq!(group.group_name().as_deref(), Some("Friends"));
    assert_eq!(record.identifier().as_deref(), Some("call-1"));
    assert_eq!(record.call_record_type(), CallRecordType::Outgoing);
    assert_eq!(record.call_capability(), CallCapability::AudioCall);
    assert_eq!(filter.call_capability(), CallCapability::AudioCall);
    assert_eq!(filter.participants_count(), 1);
    assert!(filter
        .call_types()
        .contains(CallRecordTypeOptions::OUTGOING));
    Ok(())
}
