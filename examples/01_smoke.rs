use intents::prelude::*;

fn main() {
    println!("Intents.framework version: {}", intents_version_number());
    let intent = Intent::new().expect("INIntent");
    println!("created {}", intent.class_name());
    println!("✅ intents smoke OK");
}
