# Changelog

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
