# Changelog

All notable changes to `intents` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - Unreleased

### Fixed

- Objects created through `+alloc` and `init` method pointers leaked, because
  the pointers were typed as returning an unretained object, and a nil result
  was read as a non-optional reference (undefined behavior). The six
  intent-response subclasses are now created with their typed initializers,
  and the class-factory calls made through method pointers treat a nil result
  as an error.
- `CallRecord::new` and `CallRecordFilter::new` called the unavailable
  `-init` and wrote read-only properties through KVC; they now use the public
  initializers.
- The blocking interaction and voice-shortcut calls use doom-fish-utils'
  `SyncCompletion` and shared panic helpers instead of an `mpsc` channel and
  raw `catch_unwind`, and no longer move `VoiceShortcut` values between
  threads.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment
  directory (`usr/lib/swift-5.5/macosx`) to the link search path or the
  rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's
  `libswift_Concurrency.tbd` in every binary that depends on this crate, so
  linking failed next to a Swift bridge that uses newer concurrency APIs,
  such as apple-localauthentication's.

### Changed

- The `doom-fish-utils` requirement is `>=0.4.1, <0.5`.
- `rust-version` is 1.82 (was 1.76), the fleet baseline.

### Removed

- **Breaking:** the wrappers for classes that macOS marks
  `API_UNAVAILABLE(macos)`, which the bridge reached through
  `NSClassFromString`, KVC and `unsafeBitCast`: `Preferences`,
  `SiriAuthorizationStatus`, `async_api::AsyncPreferences`,
  `async_api::SiriAuthorizationFuture`, `IntentVocabulary`,
  `VocabularyStringType`, `RelevantShortcut`, `RelevanceProvider`,
  `RelevantShortcutRole`, `DailyRoutineSituation`, `RelevantShape`,
  `RelevantShortcutStore`, `IntentParameter`, `PlayMediaIntent`,
  `SearchForMessagesIntent` and `AddTasksIntent`, with their modules, raw
  `ffi` declarations, examples and tests. SiriKit Intents on macOS is largely
  superseded by App Intents; see the README.

## [0.3.6] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

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
