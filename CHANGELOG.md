# Changelog

## [0.3.5] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.4] - 2026-05-19

- Bump MSRV from 1.70 to 1.76 to match fleet baseline.

## [0.3.3] - 2026-05-18

### Changed

- Added `///` documentation across the public Rust surface in `src/` (excluding `src/ffi/`), with each wrapper referencing its Intents.framework counterpart so the crate now clears the doc-pass target and the public API is meaningfully browsable in rustdoc.
- Fixed the README async example so the default-feature doctest compiles cleanly while the `async` example still shows the feature-gated workflow.

## [0.3.2] - 2026-05-18

### Changed

- Derived `Debug` for the six public async future wrappers (`InteractionDonateFuture`, `InteractionDeleteFuture`, `InteractionDeleteAllFuture`, `SiriAuthorizationFuture`, `AllVoiceShortcutsFuture`, and `VoiceShortcutFuture`) by routing their private completion state through an internal opaque helper, bringing every public struct in `intents` onto `#[derive(Debug)]`.

## [0.3.1] - 2026-05-17

### Fixed

- Wrapped all `unsafe extern "C"` callbacks in panic guards (`catch_unwind` /
  `catch_user_panic`) so that a panic inside a callback no longer causes
  undefined behaviour by unwinding across the C ABI boundary.
- Added `# Safety` documentation to the three `unsafe fn` signatures in
  `src/private.rs` (`RetainedObject::from_owned`, `take_string`, `take_error`).
- Widened `doom-fish-utils` version constraint from `"0.1"` to `">=0.1, <0.3"`
  to accommodate the next minor release without a forced bump.

## [0.3.0] - 2026-05-17

### Added — `async_api` module (Tier 1)

New optional `async` Cargo feature that exposes an executor-agnostic, `Future`-based
async API for every `Intents.framework` completion-handler surface.

#### INInteraction async wrappers (`AsyncInteraction`)

| Future type | Wraps |
|---|---|
| `InteractionDonateFuture` | `INInteraction.donate(completion:)` |
| `InteractionDeleteFuture` | `INInteraction.delete(with:[String],completion:)` |
| `InteractionDeleteFuture` | `INInteraction.delete(with:String,completion:)` |
| `InteractionDeleteAllFuture` | `INInteraction.deleteAll(completion:)` |

#### INPreferences async wrapper (`AsyncPreferences`)

| Future type | Wraps |
|---|---|
| `SiriAuthorizationFuture` | `INPreferences.requestSiriAuthorization(_:)` |

#### INVoiceShortcutCenter async wrappers (`AsyncVoiceShortcutCenter`)

| Future type | Wraps |
|---|---|
| `AllVoiceShortcutsFuture` | `INVoiceShortcutCenter.getAllVoiceShortcuts(completion:)` |
| `VoiceShortcutFuture` | `INVoiceShortcutCenter.getVoiceShortcut(with:completion:)` |

#### Noted Tier-2 deferrals

* `INSpeechRecognitionRequest.start(handler:)` — not available on macOS (Speech framework, iOS/watchOS only).
* `IntentHandler.handle(intent:completion:)` — protocol-based; the system calls *into* your handler. Belongs in a Tier-2 async-trait bridge.

#### New examples
- `26_async_interaction` — donate + delete variants
- `27_async_preferences` — Siri auth (skips dialog on headless systems)
- `28_async_voice_shortcuts` — getAllVoiceShortcuts + getVoiceShortcut

#### New test file
- `tests/async_api_tests.rs` — 11 tests covering happy paths and error paths

## [0.2.2] - 2026-05-17

- Closed the remaining public macOS `Intents.framework` audit gaps, bringing `COVERAGE_AUDIT.md` to 141 verified symbols, 0 remaining gaps, and 29 exempt unavailable/deprecated symbols.
- Added support, call, focus, message, reservation, extra intent/response/handler, resolution-result, and `INIntentErrorCode` wrappers, including category-backed helpers for `CLPlacemark`, `NSString`, `NSUserActivity`, `INSendMessageIntent`, `INStartCallIntent`, and `INIntentResolutionResult`.
- Added ten new numbered examples and ten new integration-test files for the new logical areas, and refreshed `README.md` / `COVERAGE.md` for the v0.2.2 surface.

## [0.2.1] - 2026-05-16

- Added `Person`, `PersonHandle`, `IntentFile`, `IntentResolutionResult`, `SendMessageIntentResponse`, and `StartCallIntentHandling` helpers, plus the supporting Intents enums needed to mirror the new macOS SDK surface.
- Added three new numbered examples and three new integration-test files covering person, file, resolution-result, send-message response, and start-call handling flows.
- Refreshed `COVERAGE_AUDIT.md` for the newly closed `INPerson*`, `INIntentResolutionResult`, `INSendMessageIntentResponse*`, `INStartCallIntentHandling`, and `INFile` gaps.

## [0.2.0] - 2026-05-16

- Added dedicated bridge and Rust modules for IntentDefinition, IntentResponse, IntentHandler, IntentDonation, IntentExtension, INParameter, INObject, INVocabulary, INRelevantShortcut, INInteraction, and INRelevantShortcutStore.
- Added support wrappers for `NSUserActivity`, `INImage`, `INSpeakableString`, `INIntentDonationMetadata`, and dynamic runtime-backed macOS-unavailable Intents classes.
- Added eleven new integration test files and eleven new numbered examples covering the expanded Intents surface.
- Added `COVERAGE.md` and refreshed the README for the v0.2.0 API surface.

## [0.1.0] - 2026-05-16

- Initial release.
- Added bindings for Siri preferences, interaction donation, voice shortcuts, and relevant shortcuts on macOS.
