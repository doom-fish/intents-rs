use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = SendMessageIntentResponse::new(SendMessageIntentResponseCode::Ready, None)?;
    let resolution = IntentResolutionResult::needs_value()?;
    let mut handling = StartCallIntentHandling::new()?;
    handling.simulate_handle()?;
    handling.simulate_confirm()?;

    println!("response code: {:?}", response.code());
    println!("resolution class: {}", resolution.class_name());
    println!("handle calls: {}", handling.handle_call_count());
    println!("confirm calls: {}", handling.confirm_call_count());
    println!("✅ intent helpers OK");
    Ok(())
}
