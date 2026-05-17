# intents

Safe Rust bindings for Apple's [Intents](https://developer.apple.com/documentation/intents) framework on macOS.

> **Status:** v0.3.0 adds an async API module (Tier 1) on top of the 100%-audited v0.2.2 sync surface.

## Quick start

```rust,no_run
use intents::prelude::*;

fn main() {
    let status = Preferences::siri_authorization_status();
    println!("Siri authorization status: {status:?}");
}
```

## Async API

Enable the `async` feature for executor-agnostic `Future` wrappers over every
`Intents.framework` completion-handler API:

```toml
[dependencies]
intents = { version = "0.3", features = ["async"] }
```

```rust,no_run
use intents::async_api::{AsyncInteraction, AsyncPreferences, AsyncVoiceShortcutCenter};
use intents::{Intent, Interaction, VoiceShortcutCenter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        // Donate an interaction
        let intent = Intent::new()?;
        let interaction = Interaction::new(&intent, None)?;
        AsyncInteraction::donate(&interaction).await?;

        // Delete all interactions
        AsyncInteraction::delete_all().await?;

        Ok::<_, Box<dyn std::error::Error>>(())
    })
}
```

### Async API surface

| Entry point | Future type | Swift API |
|---|---|---|
| `AsyncInteraction::donate` | `InteractionDonateFuture` | `INInteraction.donate(completion:)` |
| `AsyncInteraction::delete` | `InteractionDeleteFuture` | `INInteraction.delete(with:[String],completion:)` |
| `AsyncInteraction::delete_by_group` | `InteractionDeleteFuture` | `INInteraction.delete(with:String,completion:)` |
| `AsyncInteraction::delete_all` | `InteractionDeleteAllFuture` | `INInteraction.deleteAll(completion:)` |
| `AsyncPreferences::request_siri_authorization` | `SiriAuthorizationFuture` | `INPreferences.requestSiriAuthorization(_:)` |
| `AsyncVoiceShortcutCenter::get_all` | `AllVoiceShortcutsFuture` | `INVoiceShortcutCenter.getAllVoiceShortcuts(completion:)` |
| `AsyncVoiceShortcutCenter::get` | `VoiceShortcutFuture` | `INVoiceShortcutCenter.getVoiceShortcut(with:completion:)` |

## Highlights

- Core intent, response, donation, shortcut, interaction, vocabulary, relevant-shortcut, file, person, and voice-shortcut APIs from v0.1.0–v0.2.1 remain available.
- Support and model helpers for `CLPlacemark (INIntentsAdditions)`, `NSString (Intents)`, `NSUserActivity (IntentsAdditions)`, `INObjectSection`, `INObjectCollection`, `IntentsVersionNumber`, and `IntentsVersionString`.
- Call, focus, message, and reservation/travel wrappers including `INCall*`, `INFocusStatus*`, `INMessage*`, `INSticker*`, `INAirline`, `INAirport`, `INFlight`, `INPaymentMethod`, `INReservationAction`, `INSeat`, and `INTicketedEvent`.
- Additional Siri intent families: `INAnswerCallIntent`, `INEditMessageIntent`, `INGetReservationDetailsIntent`, `INHangUpCallIntent`, `INShareFocusStatusIntent`, and `INUnsendMessagesIntent`, along with their response enums and handling helpers.
- Extra resolution-result families plus `INIntentResolutionResult (Custom)` helpers and `INIntentErrorCode` coverage.

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 21_intent_extras_roundtrip
cargo run --example 25_intent_error_codes
# async examples
cargo run --features async --example 26_async_interaction
cargo run --features async --example 27_async_preferences
cargo run --features async --example 28_async_voice_shortcuts
```

The crate ships 28 numbered examples and 25 integration-test files.

## Coverage

See [COVERAGE_AUDIT.md](COVERAGE_AUDIT.md) for the current v0.2.2 SDK symbol audit, and [COVERAGE.md](COVERAGE.md) for the logical-area coverage notes.

## Notes

- Apple still marks several Siri suggestion APIs as unavailable in the macOS headers even though the Objective-C runtime exposes them. This crate keeps the existing runtime-backed wrappers for `INPreferences`, `INParameter`, `INVocabulary`, `INRelevantShortcut*`, `INRelevanceProvider*`, `INPlayMediaIntent`, `INSearchForMessagesIntent`, and `INAddTasksIntent`.
- Restaurant-booking-only types such as `INRestaurantReservationBooking` and the related response enums remain `EXEMPT` because the macOS SDK marks them unavailable.
- `NSUserActivity.shortcutAvailability` and `NSExtensionContext (ShareExtension)` are also unavailable on macOS and are documented as exempt rather than wrapped.
- `INShortcut` created from `NSUserActivity` needs a title to avoid framework validation warnings. The bridge assigns the activity type as the activity title automatically.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
