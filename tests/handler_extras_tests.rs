use intents::prelude::*;

#[test]
fn handler_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
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

    assert_eq!(answer.handle_call_count(), 1);
    assert_eq!(answer.confirm_call_count(), 1);
    assert_eq!(edit.resolve_call_count(), 1);
    assert_eq!(hang.handle_call_count(), 1);
    assert_eq!(send.resolve_call_count(), 1);
    assert_eq!(share.confirm_call_count(), 1);
    assert_eq!(unsend.confirm_call_count(), 1);
    Ok(())
}
