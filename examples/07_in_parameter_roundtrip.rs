use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parameter = IntentParameter::new("INSendMessageIntent", "content")?;
    parameter.set_index_for_sub_key_path(0, "recipients")?;

    println!("parameter class: {:?}", parameter.parameter_class_name());
    println!("parameter key path: {:?}", parameter.parameter_key_path());
    println!(
        "recipient index: {:?}",
        parameter.index_for_sub_key_path("recipients")?
    );
    println!("✅ INParameter OK");
    Ok(())
}
