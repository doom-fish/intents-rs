use intents::prelude::*;

fn main() {
    let status = Preferences::siri_authorization_status();
    println!("Siri authorization status: {status:?}");
    println!("✅ intents preferences OK");
}
