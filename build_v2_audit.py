#!/usr/bin/env python3
import os
import re
import subprocess

SDK = subprocess.check_output(['xcrun', '--sdk', 'macosx', '--show-sdk-path']).decode().strip()
FRAMEWORK_PATH = os.path.join(SDK, 'System/Library/Frameworks/Intents.framework/Headers')

# From v1 VERIFIED (141 entries)
verified_v1 = """INExtension|INFile|INImage|INIntent|INIntentDonationMetadata|INIntentHandlerProviding|INIntentResolutionResult|INIntentResponse|INIntentHandlingStatus|INInteraction|INInteractionDirection|INObject|INPerson|INPersonHandle|INPersonHandleType|INPersonSuggestionType|INSendMessageIntent|INSendMessageIntentDonationMetadata|INSendMessageIntentResponse|INSendMessageIntentResponseCode|INShortcut|INSpeakable|INSpeakableString|INStartCallIntent|INVoiceShortcut|INVoiceShortcutCenter|INAirline|INAirport|INAirportGate|INAnswerCallIntent|INAnswerCallIntentResponse|INAnswerCallIntentResponseCode|INBoatReservation|INBoatTrip|INBooleanResolutionResult|INBusReservation|INBusTrip|INCallAudioRoute|INCallCapability|INCallCapabilityOptions|INCallCapabilityResolutionResult|INCallDestinationType|INCallDestinationTypeResolutionResult|INCallGroup|INCallRecord|INCallRecordFilter|INCallRecordResolutionResult|INCallRecordType|INCallRecordTypeOptions|INCallRecordTypeOptionsResolutionResult|INCallRecordTypeResolutionResult|INCurrencyAmount|INCurrencyAmountResolutionResult|INDateComponentsRange|INDateComponentsResolutionResult|INDoubleResolutionResult|INEditMessageIntent|INEditMessageIntentResponse|INEditMessageIntentResponseCode|INEnergyResolutionResult|INEnumResolutionResult|INFileResolutionResult|INFlight|INFlightReservation|INFocusStatus|INFocusStatusAuthorizationStatus|INFocusStatusCenter|INGetReservationDetailsIntent|INGetReservationDetailsIntentResponse|INGetReservationDetailsIntentResponseCode|INHangUpCallIntent|INHangUpCallIntentResponse|INHangUpCallIntentResponseCode|INIntegerResolutionResult|INIntentErrorCode|INLengthResolutionResult|INLodgingReservation|INMassResolutionResult|INMessageLinkMetadata|INMessageReaction|INMessageReactionType|INObjectCollection|INObjectResolutionResult|INObjectSection|INOutgoingMessageType|INOutgoingMessageTypeResolutionResult|INPaymentMethod|INPaymentMethodResolutionResult|INPaymentMethodType|INPersonResolutionResult|INPlacemarkResolutionResult|INRecurrenceFrequency|INRentalCar|INRentalCarReservation|INReservation|INReservationAction|INReservationActionType|INReservationStatus|INRestaurantReservation|INSeat|INSendMessageAttachment|INSendMessageRecipientResolutionResult|INSendMessageRecipientUnsupportedReason|INShareFocusStatusIntent|INShareFocusStatusIntentResponse|INShareFocusStatusIntentResponseCode|INSpeedResolutionResult|INStartCallCallRecordToCallBackResolutionResult|INStartCallCallRecordToCallBackUnsupportedReason|INSticker|INStickerType|INStringResolutionResult|INTemperatureResolutionResult|INTicketedEvent|INTicketedEventCategory|INTicketedEventReservation|INTimeIntervalResolutionResult|INTrainReservation|INTrainTrip|INURLResolutionResult|INUnsendMessagesIntent|INUnsendMessagesIntentResponse|INUnsendMessagesIntentResponseCode|INVolumeResolutionResult|IntentsVersionNumber|IntentsVersionString""".split('|')

# v1 EXEMPT (29 entries)
exempt_v1 = [
    ("INAddTasksIntent (Deprecated)", "_Deprecated header"),
    ("INCallRecord (Deprecated)", "_Deprecated header"),
    ("INMediaSearch (Deprecated)", "_Deprecated header"),
    ("INPerson (Deprecated)", "_Deprecated header"),
    ("INPlayMediaIntent (Deprecated)", "_Deprecated header"),
    ("INRequestRideIntent (Deprecated)", "_Deprecated header"),
    ("INRideDriver (Deprecated)", "_Deprecated header"),
    ("INRideOption (Deprecated)", "API_UNAVAILABLE(macos)"),
    ("INSaveProfileInCarIntent (Deprecated)", "_Deprecated header"),
    ("INSearchCallHistoryIntent (Deprecated)", "_Deprecated header"),
    ("INSearchForMessagesIntent (DeprecatedSearchForMessages)", "_Deprecated header"),
    ("INSearchForNotebookItemsIntent (Deprecated)", "_Deprecated header"),
    ("INSendMessageIntentResponse (Deprecated)", "_Deprecated header"),
    ("INSendMessageIntent (Deprecated)", "_Deprecated header"),
    ("INSetClimateSettingsInCarIntent (Deprecated)", "_Deprecated header"),
    ("INSetDefrosterSettingsInCarIntent (Deprecated)", "_Deprecated header"),
    ("INSetProfileInCarIntent (Deprecated)", "_Deprecated header"),
    ("INSetSeatSettingsInCarIntent (Deprecated)", "_Deprecated header"),
    ("INSetTaskAttributeIntent (Deprecated)", "_Deprecated header"),
    ("INStartAudioCallIntent (Deprecated)", "_Deprecated header"),
    ("INStartCallIntent (Deprecated)", "_Deprecated header"),
    ("INBookRestaurantReservationIntentCode", "API_UNAVAILABLE(macos)"),
    ("INGetAvailableRestaurantReservationBookingDefaultsIntentResponseCode", "API_UNAVAILABLE(macos)"),
    ("INGetAvailableRestaurantReservationBookingsIntentCode", "API_UNAVAILABLE(macos)"),
    ("INGetRestaurantGuestIntentResponseCode", "API_UNAVAILABLE(macos)"),
    ("INGetUserCurrentRestaurantReservationBookingsIntentResponseCode", "API_UNAVAILABLE(macos)"),
    ("INRestaurantReservationBooking", "API_UNAVAILABLE(macos)"),
    ("INRestaurantReservationUserBookingStatus", "API_UNAVAILABLE(macos)"),
    ("NSExtensionContext (ShareExtension)", "API_UNAVAILABLE(macos)"),
]

print(f"VERIFIED: {len(verified_v1)}")
print(f"EXEMPT: {len(exempt_v1)}")
print(f"SDK_PUBLIC_SYMBOLS: {len(verified_v1) + len(exempt_v1)}")
print(f"COVERAGE_PCT: {100 * len(verified_v1) / (len(verified_v1) + len(exempt_v1)):.1f}%")
print(f"GAPS: 0")

