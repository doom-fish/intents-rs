# intents

Safe Rust bindings for Apple's [Intents](https://developer.apple.com/documentation/intents) framework on macOS.

> **Status:** v0.1.0 covers Siri authorization status, interaction donation, voice-shortcut lookup, relevant-shortcut construction, base `INIntent` / `INIntentResponse` wrappers, and opaque accessors for common built-in intent types.

## Quick start

```rust,no_run
use intents::prelude::*;

fn main() {
    let status = Preferences::siri_authorization_status();
    println!("Siri authorization status: {status:?}");
}
```

## Highlights

- `Preferences::siri_authorization_status` and `request_siri_authorization`
- `Interaction::new`, `donate`, identifier/direction/date-interval accessors
- `VoiceShortcutCenter::shared`, `get_all_voice_shortcuts`, `get_voice_shortcut`
- `Shortcut`, `RelevantShortcut`, and `RelevanceProvider::date` / `daily_routine`
- `Intent` / `IntentResponse` plus typed wrappers for `INSendMessageIntent`, `INSearchForMessagesIntent`, `INStartCallIntent`, `INPlayMediaIntent`, `INAddTasksIntent`, and `INSetTimerIntent`

## Smoke example

```bash
cargo run --example 01_smoke
```

The smoke example reads the current Siri authorization status without triggering the authorization prompt and prints `✅ intents preferences OK`.

## Notes

- Apple marks several Siri suggestions APIs as unavailable in the macOS headers even though the Objective-C runtime still exposes the classes. This crate uses dynamic Objective-C dispatch for `INPreferences`, `INRelevantShortcut`, and `INRelevanceProvider`.
- Xcode 26's SDK does not ship `INRelevantShape` or `INSetTimerIntent` headers for macOS. `RelevantShape` is included as a placeholder Rust type, and `SetTimerIntent` is a declaration-only wrapper for forward compatibility.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
