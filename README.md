# intents

Safe Rust bindings for Apple's [Intents](https://developer.apple.com/documentation/intents) framework on macOS.

## Scope: `SiriKit` Intents and App Intents

On macOS, the `SiriKit` Intents framework is largely superseded by App Intents, the Swift framework that Shortcuts, Siri and Spotlight use for app actions. New Mac features should use App Intents. This crate covers the part of the Intents framework that macOS still exposes: interactions and donations, shortcuts and voice shortcuts, intent responses and resolution results, and the call, message, focus and reservation types.

The doom-fish family has no App Intents crate. App Intents are declared as Swift types, and the build tools extract their metadata from the compiled Swift code, so they can't be wrapped from Rust like the Objective-C frameworks.

Classes that the macOS SDK marks `API_UNAVAILABLE(macos)` aren't wrapped. Versions before 0.4 reached `INPreferences`, `INVocabulary`, `INRelevantShortcut`, `INRelevantShortcutStore`, the `INRelevanceProvider` family, `INParameter`, `INPlayMediaIntent`, `INSearchForMessagesIntent` and `INAddTasksIntent` through the Objective-C runtime; 0.4 removed those wrappers.

## Requirements

- macOS 11 or later; the Swift bridge's deployment target is macOS 11.
- Newer types return an error on older systems: `CallGroup` needs macOS 11.3; `CallRecord`, `CallRecordFilter`, the focus-status and share-focus-status types, `VoiceShortcutCenter`, `SendMessageAttachment`, the send-message response and donation metadata, the start-call handling helpers and `CurrencyAmount` need macOS 12; the answer-call and hang-up-call types need macOS 13.1; the edit-message and unsend-messages types and `MessageLinkMetadata` need macOS 14; `MessageReaction` and `Sticker` need macOS 15.

## Installation

```toml
[dependencies]
intents = "0.4"
```

## Quick start

```rust,no_run
use intents::prelude::*;

fn main() -> Result<(), IntentsError> {
    let intent = Intent::new()?;
    let interaction = Interaction::new(&intent, None)?;
    interaction.donate()?;
    Ok(())
}
```

## Async API

Enable the `async` feature for executor-agnostic `Future` wrappers over every
`Intents.framework` completion-handler API:

```toml
[dependencies]
intents = { version = "0.4", features = ["async"] }
```

```rust,no_run
#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use intents::async_api::AsyncInteraction;
    use intents::{Intent, Interaction};

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

#[cfg(not(feature = "async"))]
fn main() {}
```

### Async API surface

| Entry point | Future type | Swift API |
|---|---|---|
| `AsyncInteraction::donate` | `InteractionDonateFuture` | `INInteraction.donate(completion:)` |
| `AsyncInteraction::delete` | `InteractionDeleteFuture` | `INInteraction.delete(with:[String],completion:)` |
| `AsyncInteraction::delete_by_group` | `InteractionDeleteFuture` | `INInteraction.delete(with:String,completion:)` |
| `AsyncInteraction::delete_all` | `InteractionDeleteAllFuture` | `INInteraction.deleteAll(completion:)` |
| `AsyncVoiceShortcutCenter::get_all` | `AllVoiceShortcutsFuture` | `INVoiceShortcutCenter.getAllVoiceShortcuts(completion:)` |
| `AsyncVoiceShortcutCenter::get` | `VoiceShortcutFuture` | `INVoiceShortcutCenter.getVoiceShortcut(with:completion:)` |

## Highlights

- Core intent, response, donation, shortcut, interaction, file, person, and voice-shortcut APIs.
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
cargo run --features async --example 28_async_voice_shortcuts
```

The crate ships 23 numbered examples and 22 integration-test files.

## Coverage

See [COVERAGE_AUDIT.md](COVERAGE_AUDIT.md) for the SDK symbol audit and [COVERAGE.md](COVERAGE.md) for the logical-area coverage notes.

## Notes

- Restaurant-booking-only types such as `INRestaurantReservationBooking` and the related response enums remain `EXEMPT` because the macOS SDK marks them unavailable.
- `NSUserActivity.shortcutAvailability` and `NSExtensionContext (ShareExtension)` are also unavailable on macOS and are documented as exempt rather than wrapped.
- `INShortcut` created from `NSUserActivity` needs a title to avoid framework validation warnings. The bridge assigns the activity type as the activity title automatically.
- The blocking `Interaction::donate`, `Interaction::delete_*` and `VoiceShortcutCenter` lookups wait for Intents.framework's completion handler without a timeout; use the `async` feature to avoid blocking a thread.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
