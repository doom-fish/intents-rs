use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let intent = Intent::new()?;
    let mut provider = IntentHandlerProvider::new()?;
    provider.register_handler("INIntent", "ExampleIntentHandler")?;

    println!("handler: {:?}", provider.handler_name_for_intent(&intent)?);
    println!("✅ intent handler OK");
    Ok(())
}
