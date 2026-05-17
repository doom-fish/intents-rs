use intents::prelude::*;

fn main() {
    let code = IntentErrorCode::from_raw(1901);
    println!("from raw 1901: {code:?}");
    println!("no app intent raw: {}", IntentErrorCode::NoAppIntent.raw_value());
    println!("✅ intent error codes OK");
}
