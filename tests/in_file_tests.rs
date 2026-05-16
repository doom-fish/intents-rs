use intents::prelude::*;

#[test]
fn intent_file_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = IntentFile::from_data(br#"{\"ok\":true}"#, "demo.json", Some("public.json"))?;
    assert_eq!(file.class_name(), "INFile");
    assert_eq!(file.data()?, br#"{\"ok\":true}"#);
    assert_eq!(file.filename().as_deref(), Some("demo.json"));
    assert_eq!(file.type_identifier().as_deref(), Some("public.json"));

    file.set_filename("renamed.json")?;
    assert_eq!(file.filename().as_deref(), Some("renamed.json"));

    let manifest_url = format!("file://{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
    let file_from_url =
        IntentFile::from_file_url(&manifest_url, Some("Cargo.toml"), Some("public.plain-text"))?;
    assert_eq!(file_from_url.filename().as_deref(), Some("Cargo.toml"));
    assert_eq!(
        file_from_url.type_identifier().as_deref(),
        Some("public.plain-text")
    );
    assert!(file_from_url
        .file_url()
        .as_deref()
        .is_some_and(|value| value.ends_with("/Cargo.toml")));
    assert!(!file_from_url.data()?.is_empty());
    Ok(())
}
