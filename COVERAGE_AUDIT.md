# intents coverage audit (vs MacOSX26.2.sdk)

This audit enumerates top-level Objective-C/C public symbols from `Intents.framework` headers (interfaces, protocols, categories, enums/options, structs, exported constants) in the macOS SDK. macOS-unavailable symbols are excluded from the denominator; deprecated symbols are kept as `EXEMPT` per the audit instructions.

`intents-rs` also exposes several runtime-only wrappers for symbols that the macOS 26.2 SDK marks unavailable (for example `INPreferences`, `INParameter`, `INVocabulary`, `INRelevantShortcut*`, `INRelevanceProvider*`, `INPlayMediaIntent`, `INSearchForMessagesIntent`, and `INAddTasksIntent`). Those wrappers are intentionally excluded from the `VERIFIED` / `GAPS` counts here.

SDK_PUBLIC_SYMBOLS: 170
VERIFIED: 18
GAPS: 131
EXEMPT: 21
COVERAGE_PCT: 12.1%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| INExtension | interface | INExtension.h | IntentExtension |
| INImage | interface | INImage.h | Image |
| INIntent | interface | INIntent.h | Intent |
| INIntentDonationMetadata | interface | INIntentDonationMetadata.h | IntentDonationMetadata |
| INIntentHandlerProviding | protocol | INIntentHandlerProviding.h | IntentHandlerProvider / IntentExtension::handler_class_name_for_intent |
| INIntentResponse | interface | INIntentResponse.h | IntentResponse |
| INIntentHandlingStatus | enum | INInteraction.h | IntentHandlingStatus |
| INInteraction | interface | INInteraction.h | Interaction |
| INInteractionDirection | enum | INInteraction.h | InteractionDirection |
| INObject | interface | INObject.h | IntentObject |
| INSendMessageIntent | interface | INSendMessageIntent.h | SendMessageIntent |
| INSendMessageIntentDonationMetadata | interface | INSendMessageIntentDonationMetadata.h | SendMessageIntentDonationMetadata |
| INShortcut | interface | INShortcut.h | Shortcut |
| INSpeakable | protocol | INSpeakable.h | Speakable trait |
| INSpeakableString | interface | INSpeakableString.h | SpeakableString |
| INStartCallIntent | interface | INStartCallIntent.h | StartCallIntent |
| INVoiceShortcut | interface | INVoiceShortcut.h | VoiceShortcut |
| INVoiceShortcutCenter | interface | INVoiceShortcutCenter.h | VoiceShortcutCenter |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| CLPlacemark (INIntentsAdditions) | category | CLPlacemark+IntentsAdditions.h | Objective-C category not surfaced. |
| INAirline | interface | INAirline.h | No public Rust wrapper. |
| INAirport | interface | INAirport.h | No public Rust wrapper. |
| INAirportGate | interface | INAirportGate.h | No public Rust wrapper. |
| INAnswerCallIntent | interface | INAnswerCallIntent.h | No public Rust wrapper. |
| INAnswerCallIntentHandling | protocol | INAnswerCallIntent.h | Protocol not surfaced. |
| INAnswerCallIntentResponse | interface | INAnswerCallIntentResponse.h | No public Rust wrapper. |
| INAnswerCallIntentResponseCode | enum | INAnswerCallIntentResponse.h | Enum not exposed. |
| INBoatReservation | interface | INBoatReservation.h | No public Rust wrapper. |
| INBoatTrip | interface | INBoatTrip.h | No public Rust wrapper. |
| INBookRestaurantReservationIntentCode | enum | INBookRestaurantReservationIntentResponse.h | Enum not exposed. |
| INBooleanResolutionResult | interface | INBooleanResolutionResult.h | No public Rust wrapper. |
| INBusReservation | interface | INBusReservation.h | No public Rust wrapper. |
| INBusTrip | interface | INBusTrip.h | No public Rust wrapper. |
| INCallAudioRoute | enum | INCallAudioRoute.h | Enum not exposed. |
| INCallCapability | enum | INCallCapability.h | Enum not exposed. |
| INCallCapabilityOptions | options | INCallCapabilityOptions.h | Option set not exposed. |
| INCallCapabilityResolutionResult | interface | INCallCapabilityResolutionResult.h | No public Rust wrapper. |
| INCallDestinationType | enum | INCallDestinationType.h | Enum not exposed. |
| INCallDestinationTypeResolutionResult | interface | INCallDestinationTypeResolutionResult.h | No public Rust wrapper. |
| INCallGroup | interface | INCallGroup.h | No public Rust wrapper. |
| INCallRecord | interface | INCallRecord.h | No public Rust wrapper. |
| INCallRecordFilter | interface | INCallRecordFilter.h | No public Rust wrapper. |
| INCallRecordResolutionResult | interface | INCallRecordResolutionResult.h | No public Rust wrapper. |
| INCallRecordType | enum | INCallRecordType.h | Enum not exposed. |
| INCallRecordTypeOptions | options | INCallRecordTypeOptions.h | Option set not exposed. |
| INCallRecordTypeOptionsResolutionResult | interface | INCallRecordTypeOptionsResolutionResult.h | No public Rust wrapper. |
| INCallRecordTypeResolutionResult | interface | INCallRecordTypeResolutionResult.h | No public Rust wrapper. |
| INCurrencyAmount | interface | INCurrencyAmount.h | No public Rust wrapper. |
| INCurrencyAmountResolutionResult | interface | INCurrencyAmountResolutionResult.h | No public Rust wrapper. |
| INDateComponentsRange | interface | INDateComponentsRange.h | No public Rust wrapper. |
| INDateComponentsResolutionResult | interface | INDateComponentsResolutionResult.h | No public Rust wrapper. |
| INDoubleResolutionResult | interface | INDoubleResolutionResult.h | No public Rust wrapper. |
| INEditMessageIntent | interface | INEditMessageIntent.h | No public Rust wrapper. |
| INEditMessageIntentHandling | protocol | INEditMessageIntent.h | Protocol not surfaced. |
| INEditMessageIntentResponse | interface | INEditMessageIntentResponse.h | No public Rust wrapper. |
| INEditMessageIntentResponseCode | enum | INEditMessageIntentResponse.h | Enum not exposed. |
| INEnergyResolutionResult | interface | INEnergyResolutionResult.h | No public Rust wrapper. |
| INEnumResolutionResult | interface | INEnumResolutionResult.h | No public Rust wrapper. |
| INFile | interface | INFile.h | No public Rust wrapper. |
| INFileResolutionResult | interface | INFileResolutionResult.h | No public Rust wrapper. |
| INFlight | interface | INFlight.h | No public Rust wrapper. |
| INFlightReservation | interface | INFlightReservation.h | No public Rust wrapper. |
| INFocusStatus | interface | INFocusStatus.h | No public Rust wrapper. |
| INFocusStatusAuthorizationStatus | enum | INFocusStatusCenter.h | Enum not exposed. |
| INFocusStatusCenter | interface | INFocusStatusCenter.h | No public Rust wrapper. |
| INGetAvailableRestaurantReservationBookingDefaultsIntentResponseCode | enum | INGetAvailableRestaurantReservationBookingDefaultsIntentResponse.h | Enum not exposed. |
| INGetAvailableRestaurantReservationBookingsIntentCode | enum | INGetAvailableRestaurantReservationBookingsIntentResponse.h | Enum not exposed. |
| INGetReservationDetailsIntent | interface | INGetReservationDetailsIntent.h | No public Rust wrapper. |
| INGetReservationDetailsIntentResponse | interface | INGetReservationDetailsIntentResponse.h | No public Rust wrapper. |
| INGetReservationDetailsIntentResponseCode | enum | INGetReservationDetailsIntentResponse.h | Enum not exposed. |
| INGetRestaurantGuestIntentResponseCode | enum | INGetRestaurantGuestIntentResponse.h | Enum not exposed. |
| INGetUserCurrentRestaurantReservationBookingsIntentResponseCode | enum | INGetUserCurrentRestaurantReservationBookingsIntentResponse.h | Enum not exposed. |
| INHangUpCallIntent | interface | INHangUpCallIntent.h | No public Rust wrapper. |
| INHangUpCallIntentHandling | protocol | INHangUpCallIntent.h | Protocol not surfaced. |
| INHangUpCallIntentResponse | interface | INHangUpCallIntentResponse.h | No public Rust wrapper. |
| INHangUpCallIntentResponseCode | enum | INHangUpCallIntentResponse.h | Enum not exposed. |
| INIntegerResolutionResult | interface | INIntegerResolutionResult.h | No public Rust wrapper. |
| INIntentErrorCode | enum | INIntentErrors.h | Enum not exposed. |
| INIntentResolutionResult (Custom) | category | INIntentResolutionResult+Custom.h | Objective-C category not surfaced. |
| INIntentResolutionResult | interface | INIntentResolutionResult.h | No public Rust wrapper. |
| INLengthResolutionResult | interface | INLengthResolutionResult.h | No public Rust wrapper. |
| INLodgingReservation | interface | INLodgingReservation.h | No public Rust wrapper. |
| INMassResolutionResult | interface | INMassResolutionResult.h | No public Rust wrapper. |
| INMessageLinkMetadata | interface | INMessageLinkMetadata.h | No public Rust wrapper. |
| INMessageReaction | interface | INMessageReaction.h | No public Rust wrapper. |
| INMessageReactionType | enum | INMessageReactionType.h | Enum not exposed. |
| INObjectCollection | interface | INObjectCollection.h | No public Rust wrapper. |
| INObjectResolutionResult | interface | INObjectResolutionResult.h | No public Rust wrapper. |
| INObjectSection | interface | INObjectSection.h | No public Rust wrapper. |
| INOutgoingMessageType | enum | INOutgoingMessageType.h | Enum not exposed. |
| INOutgoingMessageTypeResolutionResult | interface | INOutgoingMessageTypeResolutionResult.h | No public Rust wrapper. |
| INPaymentMethod | interface | INPaymentMethod.h | No public Rust wrapper. |
| INPaymentMethodResolutionResult | interface | INPaymentMethodResolutionResult.h | No public Rust wrapper. |
| INPaymentMethodType | enum | INPaymentMethodType.h | Enum not exposed. |
| INPerson (SiriAdditions) | category | INPerson+SiriAdditions.h | Objective-C category not surfaced. |
| INPerson | interface | INPerson.h | No public Rust wrapper. |
| INPerson (INInteraction) | category | INPerson.h | Objective-C category not surfaced. |
| INPersonSuggestionType | enum | INPerson.h | Enum not exposed. |
| INPersonHandle | interface | INPersonHandle.h | No public Rust wrapper. |
| INPersonHandleType | enum | INPersonHandle.h | Enum not exposed. |
| INPersonResolutionResult | interface | INPersonResolutionResult.h | No public Rust wrapper. |
| INPlacemarkResolutionResult | interface | INPlacemarkResolutionResult.h | No public Rust wrapper. |
| INRecurrenceFrequency | enum | INRecurrenceFrequency.h | Enum not exposed. |
| INRentalCar | interface | INRentalCar.h | No public Rust wrapper. |
| INRentalCarReservation | interface | INRentalCarReservation.h | No public Rust wrapper. |
| INReservation | interface | INReservation.h | No public Rust wrapper. |
| INReservationAction | interface | INReservationAction.h | No public Rust wrapper. |
| INReservationActionType | enum | INReservationActionType.h | Enum not exposed. |
| INReservationStatus | enum | INReservationStatus.h | Enum not exposed. |
| INRestaurantReservation | interface | INRestaurantReservation.h | No public Rust wrapper. |
| INRestaurantReservationBooking | interface | INRestaurantReservationBooking.h | No public Rust wrapper. |
| INRestaurantReservationUserBookingStatus | enum | INRestaurantReservationUserBooking.h | Enum not exposed. |
| INSeat | interface | INSeat.h | No public Rust wrapper. |
| INSendMessageAttachment | interface | INSendMessageAttachment.h | No public Rust wrapper. |
| INSendMessageIntent (UserNotifications) | category | INSendMessageIntent+UserNotifications.h | Objective-C category not surfaced. |
| INSendMessageIntentHandling | protocol | INSendMessageIntent.h | Protocol not surfaced. |
| INSendMessageIntentResponse | interface | INSendMessageIntentResponse.h | No public Rust wrapper. |
| INSendMessageIntentResponseCode | enum | INSendMessageIntentResponse.h | Enum not exposed. |
| INSendMessageRecipientResolutionResult | interface | INSendMessageRecipientResolutionResult.h | No public Rust wrapper. |
| INSendMessageRecipientUnsupportedReason | enum | INSendMessageRecipientResolutionResult.h | Enum not exposed. |
| INShareFocusStatusIntent | interface | INShareFocusStatusIntent.h | No public Rust wrapper. |
| INShareFocusStatusIntentHandling | protocol | INShareFocusStatusIntent.h | Protocol not surfaced. |
| INShareFocusStatusIntentResponse | interface | INShareFocusStatusIntentResponse.h | No public Rust wrapper. |
| INShareFocusStatusIntentResponseCode | enum | INShareFocusStatusIntentResponse.h | Enum not exposed. |
| INSpeedResolutionResult | interface | INSpeedResolutionResult.h | No public Rust wrapper. |
| INStartCallCallRecordToCallBackResolutionResult | interface | INStartCallCallRecordToCallBackResolutionResult.h | No public Rust wrapper. |
| INStartCallCallRecordToCallBackUnsupportedReason | enum | INStartCallCallRecordToCallBackResolutionResult.h | Enum not exposed. |
| INStartCallIntent (UserNotifications) | category | INStartCallIntent+UserNotifications.h | Objective-C category not surfaced. |
| INStartCallIntentHandling | protocol | INStartCallIntent.h | Protocol not surfaced. |
| INSticker | interface | INSticker.h | No public Rust wrapper. |
| INStickerType | enum | INStickerType.h | Enum not exposed. |
| INStringResolutionResult | interface | INStringResolutionResult.h | No public Rust wrapper. |
| INTemperatureResolutionResult | interface | INTemperatureResolutionResult.h | No public Rust wrapper. |
| INTicketedEvent | interface | INTicketedEvent.h | No public Rust wrapper. |
| INTicketedEventCategory | enum | INTicketedEventCategory.h | Enum not exposed. |
| INTicketedEventReservation | interface | INTicketedEventReservation.h | No public Rust wrapper. |
| INTimeIntervalResolutionResult | interface | INTimeIntervalResolutionResult.h | No public Rust wrapper. |
| INTrainReservation | interface | INTrainReservation.h | No public Rust wrapper. |
| INTrainTrip | interface | INTrainTrip.h | No public Rust wrapper. |
| INURLResolutionResult | interface | INURLResolutionResult.h | No public Rust wrapper. |
| INUnsendMessagesIntent | interface | INUnsendMessagesIntent.h | No public Rust wrapper. |
| INUnsendMessagesIntentHandling | protocol | INUnsendMessagesIntent.h | Protocol not surfaced. |
| INUnsendMessagesIntentResponse | interface | INUnsendMessagesIntentResponse.h | No public Rust wrapper. |
| INUnsendMessagesIntentResponseCode | enum | INUnsendMessagesIntentResponse.h | Enum not exposed. |
| INVolumeResolutionResult | interface | INVolumeResolutionResult.h | No public Rust wrapper. |
| IntentsVersionNumber | constant | Intents.h | Exported framework constant not surfaced. |
| IntentsVersionString | constant | Intents.h | Exported framework constant not surfaced. |
| NSExtensionContext (ShareExtension) | category | NSExtensionContext+ShareExtension.h | Objective-C category not surfaced. |
| NSString (Intents) | category | NSString+Intents.h | Objective-C category not surfaced. |
| NSUserActivity (IntentsAdditions) | category | NSUserActivity+IntentsAdditions.h | Objective-C category not surfaced. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| INAddTasksIntent (Deprecated) | category | INAddTasksIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INCallRecord (Deprecated) | category | INCallRecord_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INMediaSearch (Deprecated) | category | INMediaSearch_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INPerson (Deprecated) | category | INPerson_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INPlayMediaIntent (Deprecated) | category | INPlayMediaIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INRequestRideIntent (Deprecated) | category | INRequestRideIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INRideDriver (Deprecated) | category | INRideDriver_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INRideOption (Deprecated) | category | INRideOption.h | Deprecated API skipped per audit instructions. | Category name includes Deprecated |
| INSaveProfileInCarIntent (Deprecated) | category | INSaveProfileInCarIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSearchCallHistoryIntent (Deprecated) | category | INSearchCallHistoryIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSearchForMessagesIntent (DeprecatedSearchForMessages) | category | INSearchForMessagesIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSearchForNotebookItemsIntent (Deprecated) | category | INSearchForNotebookItemsIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSendMessageIntentResponse (Deprecated) | category | INSendMessageIntentResponse_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSendMessageIntent (Deprecated) | category | INSendMessageIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSetClimateSettingsInCarIntent (Deprecated) | category | INSetClimateSettingsInCarIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSetDefrosterSettingsInCarIntent (Deprecated) | category | INSetDefrosterSettingsInCarIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSetProfileInCarIntent (Deprecated) | category | INSetProfileInCarIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSetSeatSettingsInCarIntent (Deprecated) | category | INSetSeatSettingsInCarIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INSetTaskAttributeIntent (Deprecated) | category | INSetTaskAttributeIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INStartAudioCallIntent (Deprecated) | category | INStartAudioCallIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
| INStartCallIntent (Deprecated) | category | INStartCallIntent_Deprecated.h | Deprecated API skipped per audit instructions. | Header name ends with _Deprecated.h |
