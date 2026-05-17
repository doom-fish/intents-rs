use intents::prelude::*;

#[test]
fn focus_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let status = FocusStatus::new(Some(true))?;
    let center = FocusStatusCenter::default_center()?;

    assert_eq!(status.class_name(), FocusStatus::OBJC_CLASS);
    assert_eq!(status.is_focused(), Some(true));
    assert_eq!(center.class_name(), FocusStatusCenter::OBJC_CLASS);
    Ok(())
}
