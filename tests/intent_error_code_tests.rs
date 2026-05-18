use intents::prelude::*;

#[test]
fn intent_error_code_roundtrip() {
    assert_eq!(
        IntentErrorCode::from_raw(1901),
        IntentErrorCode::DonatingInteraction
    );
    assert_eq!(IntentErrorCode::NoAppIntent.raw_value(), 10001);
}
