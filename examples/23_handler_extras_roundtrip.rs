use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut answer = AnswerCallIntentHandling::new()?;
    let mut edit = EditMessageIntentHandling::new()?;
    let mut hang = HangUpCallIntentHandling::new()?;
    let mut send = SendMessageIntentHandling::new()?;
    let mut share = ShareFocusStatusIntentHandling::new()?;
    let mut unsend = UnsendMessagesIntentHandling::new()?;

    answer.simulate_handle()?;
    answer.simulate_confirm()?;
    edit.simulate_handle()?;
    edit.simulate_confirm()?;
    edit.simulate_resolve()?;
    hang.simulate_handle()?;
    hang.simulate_confirm()?;
    send.simulate_handle()?;
    send.simulate_confirm()?;
    send.simulate_resolve()?;
    share.simulate_handle()?;
    share.simulate_confirm()?;
    unsend.simulate_handle()?;
    unsend.simulate_confirm()?;

    println!("answer calls: {} / {}", answer.handle_call_count(), answer.confirm_call_count());
    println!("edit resolve: {}", edit.resolve_call_count());
    println!("hang calls: {} / {}", hang.handle_call_count(), hang.confirm_call_count());
    println!("send resolve: {}", send.resolve_call_count());
    println!("share calls: {} / {}", share.handle_call_count(), share.confirm_call_count());
    println!("unsend calls: {} / {}", unsend.handle_call_count(), unsend.confirm_call_count());
    println!("✅ handler extras OK");
    Ok(())
}
