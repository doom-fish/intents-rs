#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

/// Wrappers for `INCall*` types and related enums from Intents.framework.
pub mod call;
/// Error types used by the safe Intents.framework bindings.
pub mod error;
/// Rust mirrors of `INIntentErrorCode` values from Intents.framework.
pub mod error_code;
/// Low-level FFI bindings used to call into Intents.framework.
pub mod ffi;
/// Wrappers for `INFocusStatus*` types from Intents.framework.
pub mod focus;
/// Helpers for `IN*IntentHandling` protocols from Intents.framework.
pub mod handler_extras;
/// Wrappers for `INFile` from Intents.framework.
pub mod in_file;
/// Wrappers for `INInteraction` and related enums from Intents.framework.
pub mod in_interaction;
/// Wrappers for `INObject`, `INImage`, and `INSpeakable*` types.
pub mod in_object;
/// Wrappers for `INParameter` from Intents.framework.
pub mod in_parameter;
/// Wrappers for `INPerson*` types from Intents.framework.
pub mod in_person;
/// Wrappers for `INRelevantShortcut*` types from Intents.framework.
pub mod in_relevant_shortcut;
/// Wrappers for `INRelevantShortcutStore` from Intents.framework.
pub mod in_relevant_shortcut_store;
/// Wrappers for `INVocabulary` and related enums from Intents.framework.
pub mod in_vocabulary;
/// Re-exported `INIntent` wrapper types.
pub mod intent;
/// Wrappers for `INIntent`, `INShortcut`, and typed intent classes.
pub mod intent_definition;
/// Wrappers for `INIntentDonationMetadata` types.
pub mod intent_donation;
/// Wrappers for `INExtension` from Intents.framework.
pub mod intent_extension;
/// Wrappers for additional Siri intent families from Intents.framework.
pub mod intent_extras;
/// Wrappers for `INIntentHandlerProviding` and related helpers.
pub mod intent_handler;
/// Wrappers for `INIntentResolutionResult` from Intents.framework.
pub mod intent_resolution;
/// Wrappers for `INIntentResponse` and related response types.
pub mod intent_response;
/// Re-exported `INInteraction` wrapper types.
pub mod interaction;
/// Wrappers for `INMessage*` and related messaging types.
pub mod message;
/// Wrappers for `INPreferences` authorization APIs.
pub mod preferences;
mod private;
/// Re-exported `INRelevantShortcut*` wrapper types.
pub mod relevant;
/// Wrappers for reservation, travel, and payment types from Intents.framework.
pub mod reservation;
/// Wrappers for typed `INIntentResolutionResult` subclasses.
pub mod resolution_extras;
/// Wrappers for additional typed `INIntentResponse` subclasses.
pub mod response_extras;
/// Support wrappers and exported constants from Intents.framework.
pub mod support;
/// Wrappers for `INVoiceShortcut` and `INVoiceShortcutCenter`.
pub mod voice_shortcut;

/// Async wrappers over callback-based Intents.framework APIs.
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
pub use intent_extension::IntentExtension;
pub use intent_extras::{
    AnswerCallIntent, EditMessageIntent, GetReservationDetailsIntent, HangUpCallIntent,
    ShareFocusStatusIntent, UnsendMessagesIntent,
};
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
    RestaurantReservation, Seat, TicketedEvent, TicketedEventCategory, TicketedEventReservation,
    TrainReservation, TrainTrip,
};
pub use resolution_extras::{
    BooleanResolutionResult, CallCapabilityResolutionResult, CallDestinationTypeResolutionResult,
    CallRecordResolutionResult, CallRecordTypeOptionsResolutionResult,
    CallRecordTypeResolutionResult, CurrencyAmountResolutionResult, DateComponentsResolutionResult,
    DoubleResolutionResult, EnergyResolutionResult, EnumResolutionResult, FileResolutionResult,
    IntegerResolutionResult, LengthResolutionResult, MassResolutionResult, ObjectResolutionResult,
    OutgoingMessageTypeResolutionResult, PaymentMethodResolutionResult, PersonResolutionResult,
    PlacemarkResolutionResult, SendMessageRecipientResolutionResult,
    SendMessageRecipientUnsupportedReason, SpeedResolutionResult,
    StartCallCallRecordToCallBackResolutionResult, StartCallCallRecordToCallBackUnsupportedReason,
    StringResolutionResult, TemperatureResolutionResult, TimeIntervalResolutionResult,
    URLResolutionResult, VolumeResolutionResult,
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

/// Convenience re-exports for the main Intents.framework wrapper types.
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
    pub use crate::intent_extension::IntentExtension;
    pub use crate::intent_extras::{
        AnswerCallIntent, EditMessageIntent, GetReservationDetailsIntent, HangUpCallIntent,
        ShareFocusStatusIntent, UnsendMessagesIntent,
    };
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
        ObjectResolutionResult, OutgoingMessageTypeResolutionResult, PaymentMethodResolutionResult,
        PersonResolutionResult, PlacemarkResolutionResult, SendMessageRecipientResolutionResult,
        SendMessageRecipientUnsupportedReason, SpeedResolutionResult,
        StartCallCallRecordToCallBackResolutionResult,
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
