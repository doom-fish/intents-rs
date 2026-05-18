use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("group: {:?}", group.group_name());
    println!("record type: {:?}", record.call_record_type());
    println!("participants: {}", filter.participants_count());
    println!("call types: {:?}", filter.call_types());
    println!("✅ call OK");
    Ok(())
}
