#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

pub mod call;
pub mod error;
pub mod error_code;
pub mod ffi;
pub mod focus;
pub mod handler_extras;
pub mod in_file;
pub mod in_interaction;
pub mod in_object;
pub mod in_parameter;
pub mod in_person;
pub mod in_relevant_shortcut;
pub mod in_relevant_shortcut_store;
pub mod in_vocabulary;
pub mod intent;
pub mod intent_definition;
pub mod intent_donation;
pub mod intent_extras;
pub mod intent_extension;
pub mod intent_handler;
pub mod intent_resolution;
pub mod intent_response;
pub mod interaction;
pub mod message;
pub mod preferences;
mod private;
pub mod relevant;
pub mod reservation;
pub mod resolution_extras;
pub mod response_extras;
pub mod support;
pub mod voice_shortcut;

#[cfg(feature = "async")]
pub mod async_api;

pub use call::{
    CallAudioRoute, CallCapability, CallCapabilityOptions, CallDestinationType, CallGroup,
    CallRecord, CallRecordFilter, CallRecordType, CallRecordTypeOptions,
};
pub use error::IntentsError;
pub use error_code::IntentErrorCode;
pub use focus::{FocusStatus, FocusStatusAuthorizationStatus, FocusStatusCenter};
pub use handler_extras::{
    AnswerCallIntentHandling, EditMessageIntentHandling, HangUpCallIntentHandling,
    SendMessageIntentHandling, ShareFocusStatusIntentHandling, UnsendMessagesIntentHandling,
};
pub use in_file::IntentFile;
pub use in_interaction::{DateInterval, IntentHandlingStatus, Interaction, InteractionDirection};
pub use in_object::{Image, IntentObject, Speakable, SpeakableString};
pub use in_parameter::IntentParameter;
pub use in_person::{Person, PersonHandle, PersonHandleType, PersonSuggestionType};
pub use in_relevant_shortcut::{
    DailyRoutineSituation, RelevanceProvider, RelevantShape, RelevantShortcut, RelevantShortcutRole,
};
pub use in_relevant_shortcut_store::RelevantShortcutStore;
pub use in_vocabulary::{IntentVocabulary, VocabularyStringType};
pub use intent_definition::{
    AddTasksIntent, Intent, PlayMediaIntent, SearchForMessagesIntent, SendMessageIntent,
    SetTimerIntent, Shortcut, StartCallIntent,
};
pub use intent_donation::{IntentDonationMetadata, SendMessageIntentDonationMetadata};
pub use intent_extras::{
    AnswerCallIntent, EditMessageIntent, GetReservationDetailsIntent, HangUpCallIntent,
    ShareFocusStatusIntent, UnsendMessagesIntent,
};
pub use intent_extension::IntentExtension;
pub use intent_handler::{IntentHandlerProvider, StartCallIntentHandling};
pub use intent_resolution::IntentResolutionResult;
pub use intent_response::{
    IntentResponse, SendMessageIntentResponse, SendMessageIntentResponseCode, UserActivity,
};
pub use message::{
    MessageLinkMetadata, MessageReaction, MessageReactionType, OutgoingMessageType,
    SendMessageAttachment, Sticker, StickerType,
};
pub use preferences::{Preferences, SiriAuthorizationStatus};
pub use reservation::{
    Airline, Airport, AirportGate, BoatReservation, BoatTrip, BusReservation, BusTrip,
    CurrencyAmount, DateComponentsRange, Flight, FlightReservation, LodgingReservation,
    PaymentMethod, PaymentMethodType, RecurrenceFrequency, RentalCar, RentalCarReservation,
    Reservation, ReservationAction, ReservationActionType, ReservationStatus,
    RestaurantReservation, Seat, TicketedEvent, TicketedEventCategory,
    TicketedEventReservation, TrainReservation, TrainTrip,
};
pub use resolution_extras::{
    BooleanResolutionResult, CallCapabilityResolutionResult,
    CallDestinationTypeResolutionResult, CallRecordResolutionResult,
    CallRecordTypeOptionsResolutionResult, CallRecordTypeResolutionResult,
    CurrencyAmountResolutionResult, DateComponentsResolutionResult, DoubleResolutionResult,
    EnergyResolutionResult, EnumResolutionResult, FileResolutionResult, IntegerResolutionResult,
    LengthResolutionResult, MassResolutionResult, ObjectResolutionResult,
    OutgoingMessageTypeResolutionResult, PaymentMethodResolutionResult, PersonResolutionResult,
    PlacemarkResolutionResult, SendMessageRecipientResolutionResult,
    SendMessageRecipientUnsupportedReason, SpeedResolutionResult,
    StartCallCallRecordToCallBackResolutionResult,
    StartCallCallRecordToCallBackUnsupportedReason, StringResolutionResult,
    TemperatureResolutionResult, TimeIntervalResolutionResult, URLResolutionResult,
    VolumeResolutionResult,
};
pub use response_extras::{
    AnswerCallIntentResponse, AnswerCallIntentResponseCode, EditMessageIntentResponse,
    EditMessageIntentResponseCode, GetReservationDetailsIntentResponse,
    GetReservationDetailsIntentResponseCode, HangUpCallIntentResponse,
    HangUpCallIntentResponseCode, ShareFocusStatusIntentResponse,
    ShareFocusStatusIntentResponseCode, UnsendMessagesIntentResponse,
    UnsendMessagesIntentResponseCode,
};
pub use support::{
    deferred_localized_intents_string, deferred_localized_intents_string_from_table,
    intents_version_number, intents_version_string, ObjectCollection, ObjectSection, Placemark,
};
pub use voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};

pub mod prelude {
    pub use crate::call::{
        CallAudioRoute, CallCapability, CallCapabilityOptions, CallDestinationType, CallGroup,
        CallRecord, CallRecordFilter, CallRecordType, CallRecordTypeOptions,
    };
    pub use crate::error::IntentsError;
    pub use crate::error_code::IntentErrorCode;
    pub use crate::focus::{FocusStatus, FocusStatusAuthorizationStatus, FocusStatusCenter};
    pub use crate::handler_extras::{
        AnswerCallIntentHandling, EditMessageIntentHandling, HangUpCallIntentHandling,
        SendMessageIntentHandling, ShareFocusStatusIntentHandling, UnsendMessagesIntentHandling,
    };
    pub use crate::in_file::IntentFile;
    pub use crate::in_interaction::{
        DateInterval, IntentHandlingStatus, Interaction, InteractionDirection,
    };
    pub use crate::in_object::{Image, IntentObject, Speakable, SpeakableString};
    pub use crate::in_parameter::IntentParameter;
    pub use crate::in_person::{Person, PersonHandle, PersonHandleType, PersonSuggestionType};
    pub use crate::in_relevant_shortcut::{
        DailyRoutineSituation, RelevanceProvider, RelevantShape, RelevantShortcut,
        RelevantShortcutRole,
    };
    pub use crate::in_relevant_shortcut_store::RelevantShortcutStore;
    pub use crate::in_vocabulary::{IntentVocabulary, VocabularyStringType};
    pub use crate::intent_definition::{
        AddTasksIntent, Intent, PlayMediaIntent, SearchForMessagesIntent, SendMessageIntent,
        SetTimerIntent, Shortcut, StartCallIntent,
    };
    pub use crate::intent_donation::{IntentDonationMetadata, SendMessageIntentDonationMetadata};
    pub use crate::intent_extras::{
        AnswerCallIntent, EditMessageIntent, GetReservationDetailsIntent, HangUpCallIntent,
        ShareFocusStatusIntent, UnsendMessagesIntent,
    };
    pub use crate::intent_extension::IntentExtension;
    pub use crate::intent_handler::{IntentHandlerProvider, StartCallIntentHandling};
    pub use crate::intent_resolution::IntentResolutionResult;
    pub use crate::intent_response::{
        IntentResponse, SendMessageIntentResponse, SendMessageIntentResponseCode, UserActivity,
    };
    pub use crate::message::{
        MessageLinkMetadata, MessageReaction, MessageReactionType, OutgoingMessageType,
        SendMessageAttachment, Sticker, StickerType,
    };
    pub use crate::preferences::{Preferences, SiriAuthorizationStatus};
    pub use crate::reservation::{
        Airline, Airport, AirportGate, BoatReservation, BoatTrip, BusReservation, BusTrip,
        CurrencyAmount, DateComponentsRange, Flight, FlightReservation, LodgingReservation,
        PaymentMethod, PaymentMethodType, RecurrenceFrequency, RentalCar, RentalCarReservation,
        Reservation, ReservationAction, ReservationActionType, ReservationStatus,
        RestaurantReservation, Seat, TicketedEvent, TicketedEventCategory,
        TicketedEventReservation, TrainReservation, TrainTrip,
    };
    pub use crate::resolution_extras::{
        BooleanResolutionResult, CallCapabilityResolutionResult,
        CallDestinationTypeResolutionResult, CallRecordResolutionResult,
        CallRecordTypeOptionsResolutionResult, CallRecordTypeResolutionResult,
        CurrencyAmountResolutionResult, DateComponentsResolutionResult, DoubleResolutionResult,
        EnergyResolutionResult, EnumResolutionResult, FileResolutionResult,
        IntegerResolutionResult, LengthResolutionResult, MassResolutionResult,
        ObjectResolutionResult, OutgoingMessageTypeResolutionResult,
        PaymentMethodResolutionResult, PersonResolutionResult, PlacemarkResolutionResult,
        SendMessageRecipientResolutionResult, SendMessageRecipientUnsupportedReason,
        SpeedResolutionResult, StartCallCallRecordToCallBackResolutionResult,
        StartCallCallRecordToCallBackUnsupportedReason, StringResolutionResult,
        TemperatureResolutionResult, TimeIntervalResolutionResult, URLResolutionResult,
        VolumeResolutionResult,
    };
    pub use crate::response_extras::{
        AnswerCallIntentResponse, AnswerCallIntentResponseCode, EditMessageIntentResponse,
        EditMessageIntentResponseCode, GetReservationDetailsIntentResponse,
        GetReservationDetailsIntentResponseCode, HangUpCallIntentResponse,
        HangUpCallIntentResponseCode, ShareFocusStatusIntentResponse,
        ShareFocusStatusIntentResponseCode, UnsendMessagesIntentResponse,
        UnsendMessagesIntentResponseCode,
    };
    pub use crate::support::{
        deferred_localized_intents_string, deferred_localized_intents_string_from_table,
        intents_version_number, intents_version_string, ObjectCollection, ObjectSection, Placemark,
    };
    pub use crate::voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};
}
