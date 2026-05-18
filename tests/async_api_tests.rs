//! Integration tests for `async_api` module.
//!
//! Tests use `pollster::block_on` so no async runtime is required.
//! Permission-gated operations (Siri auth, `VoiceShortcutCenter`) may return
//! errors on headless CI — those are treated as advisory rather than hard
//! failures. Core Future plumbing and error-path validation are tested
//! unconditionally.

#[cfg(feature = "async")]
mod async_api_tests {
    use intents::async_api::{AsyncInteraction, AsyncPreferences, AsyncVoiceShortcutCenter};
    use intents::{Intent, Interaction, VoiceShortcutCenter};

    // -----------------------------------------------------------------------
    // AsyncInteraction — happy paths
    // -----------------------------------------------------------------------

    #[test]
    fn async_donate_resolves() {
        pollster::block_on(async {
            let intent = Intent::new().expect("Intent::new");
            let interaction = Interaction::new(&intent, None).expect("Interaction::new");
            // Donation may fail on headless CI, but the Future must resolve.
            let result = AsyncInteraction::donate(&interaction).await;
            println!("async_donate_resolves: {result:?}");
            // We accept both Ok and Err — the Future itself must resolve.
            let _ = result;
        });
    }

    #[test]
    fn async_delete_all_resolves() {
        pollster::block_on(async {
            let result = AsyncInteraction::delete_all().await;
            println!("async_delete_all_resolves: {result:?}");
            let _ = result;
        });
    }

    #[test]
    fn async_delete_by_ids_resolves() {
        pollster::block_on(async {
            let result = AsyncInteraction::delete(&["async-test-id"])
                .expect("delete arg valid")
                .await;
            println!("async_delete_by_ids_resolves: {result:?}");
            let _ = result;
        });
    }

    #[test]
    fn async_delete_empty_ids_resolves() {
        pollster::block_on(async {
            let result = AsyncInteraction::delete(&[])
                .expect("delete empty arg valid")
                .await;
            println!("async_delete_empty_ids_resolves: {result:?}");
            let _ = result;
        });
    }

    #[test]
    fn async_delete_by_group_resolves() {
        pollster::block_on(async {
            let result = AsyncInteraction::delete_by_group("async-test-group")
                .expect("delete_by_group arg valid")
                .await;
            println!("async_delete_by_group_resolves: {result:?}");
            let _ = result;
        });
    }

    // -----------------------------------------------------------------------
    // AsyncInteraction — error paths (invalid arguments)
    // -----------------------------------------------------------------------

    #[test]
    fn async_delete_nul_identifier_is_error() {
        let result = AsyncInteraction::delete(&["bad\0id"]);
        assert!(
            result.is_err(),
            "expected Err for NUL-containing identifier, got Ok"
        );
    }

    #[test]
    fn async_delete_by_group_nul_is_error() {
        let result = AsyncInteraction::delete_by_group("bad\0group");
        assert!(
            result.is_err(),
            "expected Err for NUL-containing group identifier, got Ok"
        );
    }

    // -----------------------------------------------------------------------
    // AsyncPreferences
    // -----------------------------------------------------------------------

    #[test]
    fn async_siri_auth_resolves() {
        use intents::Preferences;
        use intents::SiriAuthorizationStatus;
        // Skip if status is NotDetermined — calling requestSiriAuthorization
        // would show a system dialog and hang.
        if matches!(
            Preferences::siri_authorization_status(),
            SiriAuthorizationStatus::NotDetermined
        ) {
            println!("async_siri_auth_resolves: status=NotDetermined — skipping to avoid dialog");
            return;
        }
        pollster::block_on(async {
            let result = AsyncPreferences::request_siri_authorization().await;
            println!("async_siri_auth_resolves: {result:?}");
            let _ = result;
        });
    }

    // -----------------------------------------------------------------------
    // AsyncVoiceShortcutCenter
    // -----------------------------------------------------------------------

    #[test]
    fn async_get_all_voice_shortcuts_resolves() {
        let center = match VoiceShortcutCenter::shared() {
            Ok(c) => c,
            Err(e) => {
                println!("VoiceShortcutCenter unavailable: {e} — skipping");
                return;
            }
        };
        pollster::block_on(async {
            let result = AsyncVoiceShortcutCenter::get_all(&center).await;
            println!("async_get_all_voice_shortcuts_resolves: {result:?}");
            let _ = result;
        });
    }

    #[test]
    fn async_get_voice_shortcut_missing_uuid_resolves_none() {
        let center = match VoiceShortcutCenter::shared() {
            Ok(c) => c,
            Err(e) => {
                println!("VoiceShortcutCenter unavailable: {e} — skipping");
                return;
            }
        };
        pollster::block_on(async {
            let result =
                AsyncVoiceShortcutCenter::get(&center, "00000000-0000-0000-0000-000000000000")
                    .expect("get arg valid")
                    .await;
            println!("async_get_voice_shortcut_missing: {result:?}");
            // Accept any outcome: Ok(None) = not found, Ok(Some)/Err = CI quirks.
            let _ = result;
        });
    }

    #[test]
    fn async_get_voice_shortcut_nul_identifier_is_error() {
        let center = match VoiceShortcutCenter::shared() {
            Ok(c) => c,
            Err(e) => {
                println!("VoiceShortcutCenter unavailable: {e} — skipping");
                return;
            }
        };
        let result = AsyncVoiceShortcutCenter::get(&center, "bad\0uuid");
        assert!(
            result.is_err(),
            "expected Err for NUL-containing identifier, got Ok"
        );
    }
}
