# intents

Safe Rust bindings for Apple's [Intents](https://developer.apple.com/documentation/intents) framework on macOS.

> **Status:** v0.2.1 adds `INPerson`, `INFile`, `INIntentResolutionResult`, `INSendMessageIntentResponse`, and `INStartCallIntentHandling` coverage on top of the v0.2.0 IntentDefinition / IntentResponse / IntentHandler / IntentDonation / IntentExtension / INParameter / INObject / INVocabulary / INRelevantShortcut / INInteraction / INRelevantShortcutStore surface.

## Quick start

```rust,no_run
use intents::prelude::*;

fn main() {
    let status = Preferences::siri_authorization_status();
    println!("Siri authorization status: {status:?}");
}
```

## Highlights

- `Intent::new`, `Shortcut::new_with_user_activity`, `UserActivity`, `Image`, `Person`, `PersonHandle`, and typed wrappers for common generated intent classes.
- `IntentResponse::new`, `SendMessageIntentResponse`, `IntentResolutionResult`, `IntentDonationMetadata`, and `SendMessageIntentDonationMetadata`.
- `IntentHandlerProvider`, `StartCallIntentHandling`, and `IntentExtension` helpers for `INIntentHandlerProviding`, `INStartCallIntentHandling`, and `INExtension` flows.
- `IntentFile`, `IntentParameter`, `IntentObject`, `SpeakableString`, `IntentVocabulary`, `RelevantShortcut`, `RelevantShortcutStore`, and `Interaction`.
- Existing Siri preferences, interaction donation, relevant-shortcut, and voice-shortcut APIs remain available.

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 13_in_person_roundtrip
cargo run --example 15_intent_helpers
```

The crate now ships fifteen numbered examples and fourteen integration-test files, with at least one example and one test for every new logical area.

## Coverage audit

See [COVERAGE_AUDIT.md](COVERAGE_AUDIT.md) for the current v0.2.1 SDK symbol audit, and [COVERAGE.md](COVERAGE.md) for the v0.2.0 logical-area audit.

## Notes

- Apple marks several Siri suggestions APIs as unavailable in the macOS headers even though the Objective-C runtime still exposes the classes. This crate uses the Swift bridge plus dynamic Objective-C lookup for `INParameter`, `INVocabulary`, `INRelevantShortcut`, `INRelevanceProvider`, and `INRelevantShortcutStore`.
- `INShortcut` created from `NSUserActivity` needs a title to avoid framework validation warnings. The bridge assigns the activity type as the activity title automatically.
- Xcode 26's SDK does not ship `INRelevantShape` or `INSetTimerIntent` headers for macOS. `RelevantShape` is included as a placeholder Rust type, and `SetTimerIntent` remains a declaration-only wrapper for forward compatibility.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
