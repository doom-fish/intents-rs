# intents-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 170
VERIFIED: 141
GAPS: 0
EXEMPT: 29
COVERAGE_PCT: 100.0%

This audit re-verifies the previous v1 audit results against macOS 26.2.2.sdk (Xcode 16.0). All 29 EXEMPT entries were validated to have strict SDK availability attributes: 20 items in `_Deprecated.h` header files, and 9 items marked `API_UNAVAILABLE(macos)` in the framework headers. No additional gaps were identified.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| INExtension | interface | INExtension.h | IntentExtension |
| INFile | interface | INFile.h | IntentFile |
| INImage | interface | INImage.h | Image |
| INIntent | interface | INIntent.h | Intent |
| INIntentDonationMetadata | interface | INIntentDonationMetadata.h | IntentDonationMetadata |
| INIntentHandlerProviding | protocol | INIntentHandlerProviding.h | IntentHandlerProvider / IntentExtension::handler_class_name_for_intent |
| INIntentResolutionResult | interface | INIntentResolutionResult.h | IntentResolutionResult |
| INIntentResponse | interface | INIntentResponse.h | IntentResponse |
| INIntentHandlingStatus | enum | INInteraction.h | IntentHandlingStatus |
| INInteraction | interface | INInteraction.h | Interaction |
| INInteractionDirection | enum | INInteraction.h | InteractionDirection |
| INObject | interface | INObject.h | IntentObject |
| INPerson | interface | INPerson.h | Person |
| INPerson (INInteraction) | category | INPerson.h | Person::with_details / aliases_count / suggestion_type |
| INPerson (SiriAdditions) | category | INPerson+SiriAdditions.h | Person::is_me |
| INPersonHandle | interface | INPersonHandle.h | PersonHandle |
| INPersonHandleType | enum | INPersonHandle.h | PersonHandleType |
| INPersonSuggestionType | enum | INPerson.h | PersonSuggestionType |
| INSendMessageIntent | interface | INSendMessageIntent.h | SendMessageIntent |
| INSendMessageIntentDonationMetadata | interface | INSendMessageIntentDonationMetadata.h | SendMessageIntentDonationMetadata |
| INSendMessageIntentResponse | interface | INSendMessageIntentResponse.h | SendMessageIntentResponse |
| INSendMessageIntentResponseCode | enum | INSendMessageIntentResponse.h | SendMessageIntentResponseCode |
| INShortcut | interface | INShortcut.h | Shortcut |
| INSpeakable | protocol | INSpeakable.h | Speakable trait |
| INSpeakableString | interface | INSpeakableString.h | SpeakableString |
| INStartCallIntent | interface | INStartCallIntent.h | StartCallIntent |
| INStartCallIntentHandling | protocol | INStartCallIntent.h | StartCallIntentHandling helper |
| INVoiceShortcut | interface | INVoiceShortcut.h | VoiceShortcut |
| INVoiceShortcutCenter | interface | INVoiceShortcutCenter.h | VoiceShortcutCenter |
| CLPlacemark (INIntentsAdditions) | category | CLPlacemark+IntentsAdditions.h | Placemark::new |
| INAirline | interface | INAirline.h | Airline |
| INAirport | interface | INAirport.h | Airport |
| INAirportGate | interface | INAirportGate.h | AirportGate |
| INAnswerCallIntent | interface | INAnswerCallIntent.h | AnswerCallIntent |
| INAnswerCallIntentHandling | protocol | INAnswerCallIntent.h | AnswerCallIntentHandling helper |
| INAnswerCallIntentResponse | interface | INAnswerCallIntentResponse.h | AnswerCallIntentResponse |
| INAnswerCallIntentResponseCode | enum | INAnswerCallIntentResponse.h | AnswerCallIntentResponseCode |
| INBoatReservation | interface | INBoatReservation.h | BoatReservation |
| INBoatTrip | interface | INBoatTrip.h | BoatTrip |
| INBooleanResolutionResult | interface | INBooleanResolutionResult.h | BooleanResolutionResult |
| INBusReservation | interface | INBusReservation.h | BusReservation |
| INBusTrip | interface | INBusTrip.h | BusTrip |
| INCallAudioRoute | enum | INCallAudioRoute.h | CallAudioRoute |
| INCallCapability | enum | INCallCapability.h | CallCapability |
| INCallCapabilityOptions | options | INCallCapabilityOptions.h | CallCapabilityOptions |
| INCallCapabilityResolutionResult | interface | INCallCapabilityResolutionResult.h | CallCapabilityResolutionResult |
| INCallDestinationType | enum | INCallDestinationType.h | CallDestinationType |
| INCallDestinationTypeResolutionResult | interface | INCallDestinationTypeResolutionResult.h | CallDestinationTypeResolutionResult |
| INCallGroup | interface | INCallGroup.h | CallGroup |
| INCallRecord | interface | INCallRecord.h | CallRecord |
| INCallRecordFilter | interface | INCallRecordFilter.h | CallRecordFilter |
| INCallRecordResolutionResult | interface | INCallRecordResolutionResult.h | CallRecordResolutionResult |
| INCallRecordType | enum | INCallRecordType.h | CallRecordType |
| INCallRecordTypeOptions | options | INCallRecordTypeOptions.h | CallRecordTypeOptions |
| INCallRecordTypeOptionsResolutionResult | interface | INCallRecordTypeOptionsResolutionResult.h | CallRecordTypeOptionsResolutionResult |
| INCallRecordTypeResolutionResult | interface | INCallRecordTypeResolutionResult.h | CallRecordTypeResolutionResult |
| INCurrencyAmount | interface | INCurrencyAmount.h | CurrencyAmount |
| INCurrencyAmountResolutionResult | interface | INCurrencyAmountResolutionResult.h | CurrencyAmountResolutionResult |
| INDateComponentsRange | interface | INDateComponentsRange.h | DateComponentsRange |
| INDateComponentsResolutionResult | interface | INDateComponentsResolutionResult.h | DateComponentsResolutionResult |
| INDoubleResolutionResult | interface | INDoubleResolutionResult.h | DoubleResolutionResult |
| INEditMessageIntent | interface | INEditMessageIntent.h | EditMessageIntent |
| INEditMessageIntentHandling | protocol | INEditMessageIntent.h | EditMessageIntentHandling helper |
| INEditMessageIntentResponse | interface | INEditMessageIntentResponse.h | EditMessageIntentResponse |
| INEditMessageIntentResponseCode | enum | INEditMessageIntentResponse.h | EditMessageIntentResponseCode |
| INEnergyResolutionResult | interface | INEnergyResolutionResult.h | EnergyResolutionResult |
| INEnumResolutionResult | interface | INEnumResolutionResult.h | EnumResolutionResult |
| INFileResolutionResult | interface | INFileResolutionResult.h | FileResolutionResult |
| INFlight | interface | INFlight.h | Flight |
| INFlightReservation | interface | INFlightReservation.h | FlightReservation |
| INFocusStatus | interface | INFocusStatus.h | FocusStatus |
| INFocusStatusAuthorizationStatus | enum | INFocusStatusCenter.h | FocusStatusAuthorizationStatus |
| INFocusStatusCenter | interface | INFocusStatusCenter.h | FocusStatusCenter |
| INGetReservationDetailsIntent | interface | INGetReservationDetailsIntent.h | GetReservationDetailsIntent |
| INGetReservationDetailsIntentResponse | interface | INGetReservationDetailsIntentResponse.h | GetReservationDetailsIntentResponse |
| INGetReservationDetailsIntentResponseCode | enum | INGetReservationDetailsIntentResponse.h | GetReservationDetailsIntentResponseCode |
| INHangUpCallIntent | interface | INHangUpCallIntent.h | HangUpCallIntent |
| INHangUpCallIntentHandling | protocol | INHangUpCallIntent.h | HangUpCallIntentHandling helper |
| INHangUpCallIntentResponse | interface | INHangUpCallIntentResponse.h | HangUpCallIntentResponse |
| INHangUpCallIntentResponseCode | enum | INHangUpCallIntentResponse.h | HangUpCallIntentResponseCode |
| INIntegerResolutionResult | interface | INIntegerResolutionResult.h | IntegerResolutionResult |
| INIntentErrorCode | enum | INIntentErrors.h | IntentErrorCode |
| INIntentResolutionResult (Custom) | category | INIntentResolutionResult+Custom.h | IntentResolutionResult::unsupported_with_reason / confirmation_required_with_item_for_reason |
| INLengthResolutionResult | interface | INLengthResolutionResult.h | LengthResolutionResult |
| INLodgingReservation | interface | INLodgingReservation.h | LodgingReservation |
| INMassResolutionResult | interface | INMassResolutionResult.h | MassResolutionResult |
| INMessageLinkMetadata | interface | INMessageLinkMetadata.h | MessageLinkMetadata |
| INMessageReaction | interface | INMessageReaction.h | MessageReaction |
| INMessageReactionType | enum | INMessageReactionType.h | MessageReactionType |
| INObjectCollection | interface | INObjectCollection.h | ObjectCollection |
| INObjectResolutionResult | interface | INObjectResolutionResult.h | ObjectResolutionResult |
| INObjectSection | interface | INObjectSection.h | ObjectSection |
| INOutgoingMessageType | enum | INOutgoingMessageType.h | OutgoingMessageType |
| INOutgoingMessageTypeResolutionResult | interface | INOutgoingMessageTypeResolutionResult.h | OutgoingMessageTypeResolutionResult |
| INPaymentMethod | interface | INPaymentMethod.h | PaymentMethod |
| INPaymentMethodResolutionResult | interface | INPaymentMethodResolutionResult.h | PaymentMethodResolutionResult |
| INPaymentMethodType | enum | INPaymentMethodType.h | PaymentMethodType |
| INPersonResolutionResult | interface | INPersonResolutionResult.h | PersonResolutionResult |
| INPlacemarkResolutionResult | interface | INPlacemarkResolutionResult.h | PlacemarkResolutionResult |
| INRecurrenceFrequency | enum | INRecurrenceFrequency.h | RecurrenceFrequency |
| INRentalCar | interface | INRentalCar.h | RentalCar |
| INRentalCarReservation | interface | INRentalCarReservation.h | RentalCarReservation |
| INReservation | interface | INReservation.h | Reservation |
| INReservationAction | interface | INReservationAction.h | ReservationAction |
| INReservationActionType | enum | INReservationActionType.h | ReservationActionType |
| INReservationStatus | enum | INReservationStatus.h | ReservationStatus |
| INRestaurantReservation | interface | INRestaurantReservation.h | RestaurantReservation |
| INSeat | interface | INSeat.h | Seat |
| INSendMessageAttachment | interface | INSendMessageAttachment.h | SendMessageAttachment |
| INSendMessageIntent (UserNotifications) | category | INSendMessageIntent+UserNotifications.h | SendMessageIntent::supports_user_notifications |
| INSendMessageIntentHandling | protocol | INSendMessageIntent.h | SendMessageIntentHandling helper |
| INSendMessageRecipientResolutionResult | interface | INSendMessageRecipientResolutionResult.h | SendMessageRecipientResolutionResult |
| INSendMessageRecipientUnsupportedReason | enum | INSendMessageRecipientResolutionResult.h | SendMessageRecipientUnsupportedReason |
| INShareFocusStatusIntent | interface | INShareFocusStatusIntent.h | ShareFocusStatusIntent |
| INShareFocusStatusIntentHandling | protocol | INShareFocusStatusIntent.h | ShareFocusStatusIntentHandling helper |
| INShareFocusStatusIntentResponse | interface | INShareFocusStatusIntentResponse.h | ShareFocusStatusIntentResponse |
| INShareFocusStatusIntentResponseCode | enum | INShareFocusStatusIntentResponse.h | ShareFocusStatusIntentResponseCode |
| INSpeedResolutionResult | interface | INSpeedResolutionResult.h | SpeedResolutionResult |
| INStartCallCallRecordToCallBackResolutionResult | interface | INStartCallCallRecordToCallBackResolutionResult.h | StartCallCallRecordToCallBackResolutionResult |
| INStartCallCallRecordToCallBackUnsupportedReason | enum | INStartCallCallRecordToCallBackResolutionResult.h | StartCallCallRecordToCallBackUnsupportedReason |
| INStartCallIntent (UserNotifications) | category | INStartCallIntent+UserNotifications.h | StartCallIntent::supports_user_notifications |
| INSticker | interface | INSticker.h | Sticker |
| INStickerType | enum | INStickerType.h | StickerType |
| INStringResolutionResult | interface | INStringResolutionResult.h | StringResolutionResult |
| INTemperatureResolutionResult | interface | INTemperatureResolutionResult.h | TemperatureResolutionResult |
| INTicketedEvent | interface | INTicketedEvent.h | TicketedEvent |
| INTicketedEventCategory | enum | INTicketedEventCategory.h | TicketedEventCategory |
| INTicketedEventReservation | interface | INTicketedEventReservation.h | TicketedEventReservation |
| INTimeIntervalResolutionResult | interface | INTimeIntervalResolutionResult.h | TimeIntervalResolutionResult |
| INTrainReservation | interface | INTrainReservation.h | TrainReservation |
| INTrainTrip | interface | INTrainTrip.h | TrainTrip |
| INURLResolutionResult | interface | INURLResolutionResult.h | URLResolutionResult |
| INUnsendMessagesIntent | interface | INUnsendMessagesIntent.h | UnsendMessagesIntent |
| INUnsendMessagesIntentHandling | protocol | INUnsendMessagesIntent.h | UnsendMessagesIntentHandling helper |
| INUnsendMessagesIntentResponse | interface | INUnsendMessagesIntentResponse.h | UnsendMessagesIntentResponse |
| INUnsendMessagesIntentResponseCode | enum | INUnsendMessagesIntentResponse.h | UnsendMessagesIntentResponseCode |
| INVolumeResolutionResult | interface | INVolumeResolutionResult.h | VolumeResolutionResult |
| IntentsVersionNumber | constant | Intents.h | intents_version_number |
| IntentsVersionString | constant | Intents.h | intents_version_string |
| NSString (Intents) | category | NSString+Intents.h | deferred_localized_intents_string* |
| NSUserActivity (IntentsAdditions) | category | NSUserActivity+IntentsAdditions.h | UserActivity::interaction / suggested_invocation_phrase |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| INAddTasksIntent (Deprecated) | category | INAddTasksIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INCallRecord (Deprecated) | category | INCallRecord_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INMediaSearch (Deprecated) | category | INMediaSearch_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INPerson (Deprecated) | category | INPerson_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INPlayMediaIntent (Deprecated) | category | INPlayMediaIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INRequestRideIntent (Deprecated) | category | INRequestRideIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INRideDriver (Deprecated) | category | INRideDriver_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INRideOption (Deprecated) | category | INRideOption.h | Unavailable on macOS; iOS-only ride-sharing UI category. | API_UNAVAILABLE(macos) |
| INSaveProfileInCarIntent (Deprecated) | category | INSaveProfileInCarIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSearchCallHistoryIntent (Deprecated) | category | INSearchCallHistoryIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSearchForMessagesIntent (DeprecatedSearchForMessages) | category | INSearchForMessagesIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSearchForNotebookItemsIntent (Deprecated) | category | INSearchForNotebookItemsIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSendMessageIntentResponse (Deprecated) | category | INSendMessageIntentResponse_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSendMessageIntent (Deprecated) | category | INSendMessageIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSetClimateSettingsInCarIntent (Deprecated) | category | INSetClimateSettingsInCarIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSetDefrosterSettingsInCarIntent (Deprecated) | category | INSetDefrosterSettingsInCarIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSetProfileInCarIntent (Deprecated) | category | INSetProfileInCarIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSetSeatSettingsInCarIntent (Deprecated) | category | INSetSeatSettingsInCarIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INSetTaskAttributeIntent (Deprecated) | category | INSetTaskAttributeIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INStartAudioCallIntent (Deprecated) | category | INStartAudioCallIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INStartCallIntent (Deprecated) | category | INStartCallIntent_Deprecated.h | Deprecated API excluded per audit v2 instructions. | _Deprecated.h header |
| INBookRestaurantReservationIntentCode | enum | INBookRestaurantReservationIntentResponse.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| INGetAvailableRestaurantReservationBookingDefaultsIntentResponseCode | enum | INGetAvailableRestaurantReservationBookingDefaultsIntentResponse.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| INGetAvailableRestaurantReservationBookingsIntentCode | enum | INGetAvailableRestaurantReservationBookingsIntentResponse.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| INGetRestaurantGuestIntentResponseCode | enum | INGetRestaurantGuestIntentResponse.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| INGetUserCurrentRestaurantReservationBookingsIntentResponseCode | enum | INGetUserCurrentRestaurantReservationBookingsIntentResponse.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| INRestaurantReservationBooking | interface | INRestaurantReservationBooking.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| INRestaurantReservationUserBookingStatus | enum | INRestaurantReservationUserBooking.h | Unavailable on macOS; restaurant-reservation flow not exposed for macOS SDK. | API_UNAVAILABLE(macos, watchos, tvos) |
| NSExtensionContext (ShareExtension) | category | NSExtensionContext+ShareExtension.h | Unavailable on macOS; share-extension additions not exposed for macOS SDK. | API_UNAVAILABLE(macos, tvos) |
