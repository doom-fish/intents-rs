#!/usr/bin/env python3
# From v1: VERIFIED=141, EXEMPT=29, so SDK_PUBLIC_SYMBOLS should be 141+29=170
# Let me only count symbols present in 170 symbol SDK

verified_v1 = [
    "INExtension", "INFile", "INImage", "INIntent", "INIntentDonationMetadata",
    "INIntentHandlerProviding", "INIntentResolutionResult", "INIntentResponse",
    "INIntentHandlingStatus", "INInteraction", "INInteractionDirection", "INObject",
    "INPerson", "INPersonHandle", "INPersonHandleType", "INPersonSuggestionType",
    "INSendMessageIntent", "INSendMessageIntentDonationMetadata", "INSendMessageIntentResponse",
    "INSendMessageIntentResponseCode", "INShortcut", "INSpeakable", "INSpeakableString",
    "INStartCallIntent", "INVoiceShortcut", "INVoiceShortcutCenter", "INAirline",
    "INAirport", "INAirportGate", "INAnswerCallIntent", "INAnswerCallIntentResponse",
    "INAnswerCallIntentResponseCode", "INBoatReservation", "INBoatTrip",
    "INBooleanResolutionResult", "INBusReservation", "INBusTrip", "INCallAudioRoute",
    "INCallCapability", "INCallCapabilityOptions", "INCallCapabilityResolutionResult",
    "INCallDestinationType", "INCallDestinationTypeResolutionResult", "INCallGroup",
    "INCallRecord", "INCallRecordFilter", "INCallRecordResolutionResult", "INCallRecordType",
    "INCallRecordTypeOptions", "INCallRecordTypeOptionsResolutionResult",
    "INCallRecordTypeResolutionResult", "INCurrencyAmount", "INCurrencyAmountResolutionResult",
    "INDateComponentsRange", "INDateComponentsResolutionResult", "INDoubleResolutionResult",
    "INEditMessageIntent", "INEditMessageIntentResponse", "INEditMessageIntentResponseCode",
    "INEnergyResolutionResult", "INEnumResolutionResult", "INFileResolutionResult", "INFlight",
    "INFlightReservation", "INFocusStatus", "INFocusStatusAuthorizationStatus",
    "INFocusStatusCenter", "INGetReservationDetailsIntent", "INGetReservationDetailsIntentResponse",
    "INGetReservationDetailsIntentResponseCode", "INHangUpCallIntent", "INHangUpCallIntentResponse",
    "INHangUpCallIntentResponseCode", "INIntegerResolutionResult", "INIntentErrorCode",
    "INLengthResolutionResult", "INLodgingReservation", "INMassResolutionResult",
    "INMessageLinkMetadata", "INMessageReaction", "INMessageReactionType", "INObjectCollection",
    "INObjectResolutionResult", "INObjectSection", "INOutgoingMessageType",
    "INOutgoingMessageTypeResolutionResult", "INPaymentMethod", "INPaymentMethodResolutionResult",
    "INPaymentMethodType", "INPersonResolutionResult", "INPlacemarkResolutionResult",
    "INRecurrenceFrequency", "INRentalCar", "INRentalCarReservation", "INReservation",
    "INReservationAction", "INReservationActionType", "INReservationStatus",
    "INRestaurantReservation", "INSeat", "INSendMessageAttachment",
    "INSendMessageRecipientResolutionResult", "INSendMessageRecipientUnsupportedReason",
    "INShareFocusStatusIntent", "INShareFocusStatusIntentResponse", "INShareFocusStatusIntentResponseCode",
    "INSpeedResolutionResult", "INStartCallCallRecordToCallBackResolutionResult",
    "INStartCallCallRecordToCallBackUnsupportedReason", "INSticker", "INStickerType",
    "INStringResolutionResult", "INTemperatureResolutionResult", "INTicketedEvent",
    "INTicketedEventCategory", "INTicketedEventReservation", "INTimeIntervalResolutionResult",
    "INTrainReservation", "INTrainTrip", "INURLResolutionResult", "INUnsendMessagesIntent",
    "INUnsendMessagesIntentResponse", "INUnsendMessagesIntentResponseCode",
    "INVolumeResolutionResult", "IntentsVersionNumber", "IntentsVersionString",
]

exempt_v1 = [
    "INAddTasksIntent",  # _Deprecated
    "INCallRecord",      # _Deprecated (but main interface is VERIFIED)
    "INMediaSearch",     # _Deprecated
    "INPerson",          # _Deprecated
    "INPlayMediaIntent", # _Deprecated
    "INRequestRideIntent", # _Deprecated
    "INRideDriver",      # _Deprecated
    "INRideOption",      # _Deprecated (inline in header)
    "INSaveProfileInCarIntent", # _Deprecated
    "INSearchCallHistoryIntent", # _Deprecated
    "INSearchForMessagesIntent", # _Deprecated
    "INSearchForNotebookItemsIntent", # _Deprecated
    "INSendMessageIntentResponse", # _Deprecated
    "INSendMessageIntent", # _Deprecated (but main interface is VERIFIED)
    "INSetClimateSettingsInCarIntent", # _Deprecated
    "INSetDefrosterSettingsInCarIntent", # _Deprecated
    "INSetProfileInCarIntent", # _Deprecated
    "INSetSeatSettingsInCarIntent", # _Deprecated
    "INSetTaskAttributeIntent", # _Deprecated
    "INStartAudioCallIntent", # _Deprecated (but main interface might be VERIFIED)
    "INStartCallIntent", # _Deprecated (but main interface is VERIFIED)
    # Restaurant/unavailable
    "INBookRestaurantReservationIntentCode",
    "INGetAvailableRestaurantReservationBookingDefaultsIntentResponseCode",
    "INGetAvailableRestaurantReservationBookingsIntentCode",
    "INGetRestaurantGuestIntentResponseCode",
    "INGetUserCurrentRestaurantReservationBookingsIntentResponseCode",
    "INRestaurantReservationBooking",
    "INRestaurantReservationUserBookingStatus",
    # Share ext
    "NSExtensionContext (ShareExtension)",
]

print(f"Verified (v1): {len(verified_v1)}")
print(f"Exempt (v1): {len(exempt_v1)}")
print(f"SDK_PUBLIC_SYMBOLS = {len(verified_v1) + len(exempt_v1)}")
print()
print(f"Coverage = {len(verified_v1)} / ({len(verified_v1)} + {len(exempt_v1)}) = {100*len(verified_v1)/(len(verified_v1)+len(exempt_v1)):.1f}%")

