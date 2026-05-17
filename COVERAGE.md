# Intents.framework coverage notes (v0.2.2)

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped / exempt

## Core surface retained from v0.1.0–v0.2.1

| Area | Status | Notes |
| --- | --- | --- |
| Preferences / authorization | ✅ | `Preferences` wraps Siri authorization status and callback-based authorization requests. |
| IntentDefinition / Shortcut | ✅ | `Intent`, `Shortcut`, `SendMessageIntent`, and `StartCallIntent` remain available, including image and donation metadata helpers. |
| IntentResponse / UserActivity | ✅ | `IntentResponse`, `SendMessageIntentResponse`, and `UserActivity` remain available. |
| IntentHandler / IntentExtension | ✅ | `IntentHandlerProvider`, `StartCallIntentHandling`, and `IntentExtension` remain available. |
| IntentDonation | ✅ | `IntentDonationMetadata` and `SendMessageIntentDonationMetadata` remain available. |
| INParameter / INObject / INFile / INPerson | ✅ | `IntentParameter`, `IntentObject`, `IntentFile`, `Person`, `PersonHandle`, and related enums remain available. |
| Vocabulary / relevant shortcuts / interaction | ✅ | `IntentVocabulary`, `RelevantShortcut`, `RelevantShortcutStore`, `Interaction`, and related helpers remain available. |
| Voice shortcuts | ✅ | `VoiceShortcut` and `VoiceShortcutCenter` remain available. |

## v0.2.2 gap-closure areas

### Support and category-backed helpers

| API | Status | Notes |
| --- | --- | --- |
| `CLPlacemark (INIntentsAdditions)` | ✅ | `Placemark::new` creates `CLPlacemark` instances via the Intents category factory. |
| `NSString (Intents)` | ✅ | `deferred_localized_intents_string` and `deferred_localized_intents_string_from_table` wrap the category methods. |
| `NSUserActivity (IntentsAdditions)` | ✅ | `UserActivity::interaction`, `suggested_invocation_phrase`, and `set_suggested_invocation_phrase` cover the macOS-available additions. |
| `INObjectSection` / `INObjectCollection` | ✅ | `ObjectSection` and `ObjectCollection` expose section/item construction and collation helpers. |
| `IntentsVersionNumber` / `IntentsVersionString` | ✅ | `intents_version_number` and `intents_version_string` surface the exported framework version constants. |

### Call surface

| API | Status | Notes |
| --- | --- | --- |
| `INCallAudioRoute`, `INCallCapability`, `INCallCapabilityOptions`, `INCallDestinationType`, `INCallRecordType`, `INCallRecordTypeOptions` | ✅ | Exposed as Rust enums / option sets. |
| `INCallGroup`, `INCallRecord`, `INCallRecordFilter` | ✅ | Safe constructors and property accessors are exposed. |
| Call-related resolution results | ✅ | `CallCapabilityResolutionResult`, `CallDestinationTypeResolutionResult`, `CallRecordResolutionResult`, `CallRecordTypeResolutionResult`, and `CallRecordTypeOptionsResolutionResult` are exposed. |

### Focus surface

| API | Status | Notes |
| --- | --- | --- |
| `INFocusStatus` | ✅ | `FocusStatus::new` and `is_focused` are exposed. |
| `INFocusStatusCenter` | ✅ | `FocusStatusCenter::default_center` is exposed. |
| `INFocusStatusAuthorizationStatus` | ✅ | Exposed as `FocusStatusAuthorizationStatus`. |

### Message surface

| API | Status | Notes |
| --- | --- | --- |
| `INMessageLinkMetadata` | ✅ | `MessageLinkMetadata::new` plus property accessors. |
| `INMessageReaction` / `INMessageReactionType` | ✅ | `MessageReaction` and `MessageReactionType` are exposed. |
| `INSendMessageAttachment` | ✅ | `SendMessageAttachment::audio_message_file` wraps the audio attachment factory. |
| `INSticker` / `INStickerType` | ✅ | `Sticker` and `StickerType` are exposed. |
| `INOutgoingMessageType` | ✅ | Exposed as `OutgoingMessageType`. |

### Reservation, travel, and payment surface

| API | Status | Notes |
| --- | --- | --- |
| `INAirline`, `INAirport`, `INAirportGate`, `INFlight` | ✅ | Constructors and property accessors are exposed. |
| `INCurrencyAmount`, `INDateComponentsRange`, `INPaymentMethod`, `INPaymentMethodType`, `INRecurrenceFrequency` | ✅ | Exposed as safe wrappers / enums. |
| `INReservation`, `INReservationAction`, `INReservationActionType`, `INReservationStatus` | ✅ | Shared reservation surface is wrapped. |
| `INBoatReservation`, `INBoatTrip`, `INBusReservation`, `INBusTrip`, `INFlightReservation`, `INLodgingReservation`, `INRentalCar`, `INRentalCarReservation`, `INRestaurantReservation`, `INTrainReservation`, `INTrainTrip` | ✅ | Public wrapper types are exported for the macOS-available reservation classes. |
| `INSeat`, `INTicketedEvent`, `INTicketedEventCategory`, `INTicketedEventReservation` | ✅ | Seat and ticketed-event helpers are exposed. |

### Additional intent, response, and handling families

| API | Status | Notes |
| --- | --- | --- |
| `INAnswerCallIntent*` | ✅ | `AnswerCallIntent`, `AnswerCallIntentResponse`, `AnswerCallIntentResponseCode`, and `AnswerCallIntentHandling` helper are exposed. |
| `INEditMessageIntent*` | ✅ | `EditMessageIntent`, `EditMessageIntentResponse`, `EditMessageIntentResponseCode`, and `EditMessageIntentHandling` helper are exposed. |
| `INGetReservationDetailsIntent*` | ✅ | `GetReservationDetailsIntent`, `GetReservationDetailsIntentResponse`, and `GetReservationDetailsIntentResponseCode` are exposed. |
| `INHangUpCallIntent*` | ✅ | `HangUpCallIntent`, `HangUpCallIntentResponse`, `HangUpCallIntentResponseCode`, and `HangUpCallIntentHandling` helper are exposed. |
| `INShareFocusStatusIntent*` | ✅ | `ShareFocusStatusIntent`, `ShareFocusStatusIntentResponse`, `ShareFocusStatusIntentResponseCode`, and `ShareFocusStatusIntentHandling` helper are exposed. |
| `INUnsendMessagesIntent*` | ✅ | `UnsendMessagesIntent`, `UnsendMessagesIntentResponse`, `UnsendMessagesIntentResponseCode`, and `UnsendMessagesIntentHandling` helper are exposed. |
| `INSendMessageIntentHandling` | ✅ | `SendMessageIntentHandling` helper is exposed. |
| `INSendMessageIntent (UserNotifications)` / `INStartCallIntent (UserNotifications)` | ✅ | `SendMessageIntent::supports_user_notifications` and `StartCallIntent::supports_user_notifications` expose the protocol-conformance checks. |

### Resolution-result extras and errors

| API | Status | Notes |
| --- | --- | --- |
| Scalar/object resolution results (`INBooleanResolutionResult`, `INDoubleResolutionResult`, `INIntegerResolutionResult`, `INStringResolutionResult`, `INURLResolutionResult`, `INFileResolutionResult`, etc.) | ✅ | Exposed through the typed resolution-result wrappers in `resolution_extras.rs`. |
| Message/call-specific resolution results | ✅ | `SendMessageRecipientResolutionResult`, `OutgoingMessageTypeResolutionResult`, and `StartCallCallRecordToCallBackResolutionResult` are exposed, including unsupported-reason enums. |
| `INIntentResolutionResult (Custom)` | ✅ | `unsupported_with_reason` and `confirmation_required_with_item_for_reason` are exposed on `IntentResolutionResult`. |
| `INIntentErrorCode` | ✅ | Exposed as `IntentErrorCode`. |

## Deferred / exempt

| API | Status | Reason |
| --- | --- | --- |
| `INRestaurantReservationBooking` and restaurant-booking-only response enums | ⏭️ | The macOS SDK marks these types unavailable; they are recorded in `COVERAGE_AUDIT.md` as `EXEMPT`. |
| `NSExtensionContext (ShareExtension)` | ⏭️ | The share-extension category is unavailable on macOS. |
| `NSUserActivity.shortcutAvailability` | ⏭️ | The property is unavailable on macOS even though the rest of `NSUserActivity (IntentsAdditions)` is available. |
| Deprecated `*_Deprecated.h` Intents categories | ⏭️ | Kept as `EXEMPT` per the audit instructions. |
