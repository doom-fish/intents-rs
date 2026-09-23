use intents::prelude::*;

#[test]
fn voice_shortcut_center_lookups_complete() {
    let Ok(center) = VoiceShortcutCenter::shared() else {
        return;
    };

    let error = center.get_voice_shortcut("not-a-uuid").unwrap_err();
    assert!(error.to_string().contains("UUID"), "{error}");

    let missing = center.get_voice_shortcut("00000000-0000-0000-0000-000000000000");
    assert!(!matches!(missing, Ok(Some(_))), "{missing:?}");

    if let Ok(shortcuts) = center.get_all_voice_shortcuts() {
        assert!(shortcuts.iter().all(|shortcut| shortcut.identifier().is_some()));
    }
}
