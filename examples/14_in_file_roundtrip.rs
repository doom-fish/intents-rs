use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = IntentFile::from_data(b"hello", "demo.txt", Some("public.plain-text"))?;
    file.set_filename("renamed.txt")?;

    println!("file class: {}", file.class_name());
    println!("file name: {:?}", file.filename());
    println!("file bytes: {:?}", file.data()?);
    println!("✅ INFile OK");
    Ok(())
}
