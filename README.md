# intents

Safe Rust bindings for Apple's [Intents](https://developer.apple.com/documentation/intents) framework on macOS.

> **Status:** v0.2.2 reaches 100% audited coverage of the macOS 26.2 `Intents.framework` surface: 141 verified symbols, 0 remaining gaps, and 29 exempt unavailable/deprecated symbols.

## Quick start

```rust,no_run
use intents::prelude::*;

fn main() {
    let status = Preferences::siri_authorization_status();
    println!("Siri authorization status: {status:?}");
}
```

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
```

The crate now ships 25 numbered examples and 24 integration-test files, with coverage for every logical area added in the v0.2.2 gap-closing pass.

## Coverage

See [COVERAGE_AUDIT.md](COVERAGE_AUDIT.md) for the current v0.2.2 SDK symbol audit, and [COVERAGE.md](COVERAGE.md) for the logical-area coverage notes.

## Notes

- Apple still marks several Siri suggestion APIs as unavailable in the macOS headers even though the Objective-C runtime exposes them. This crate keeps the existing runtime-backed wrappers for `INPreferences`, `INParameter`, `INVocabulary`, `INRelevantShortcut*`, `INRelevanceProvider*`, `INPlayMediaIntent`, `INSearchForMessagesIntent`, and `INAddTasksIntent`.
- Restaurant-booking-only types such as `INRestaurantReservationBooking` and the related response enums remain `EXEMPT` because the macOS SDK marks them unavailable.
- `NSUserActivity.shortcutAvailability` and `NSExtensionContext (ShareExtension)` are also unavailable on macOS and are documented as exempt rather than wrapped.
- `INShortcut` created from `NSUserActivity` needs a title to avoid framework validation warnings. The bridge assigns the activity type as the activity title automatically.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
