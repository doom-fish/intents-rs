#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

pub mod error;
pub mod ffi;
pub mod in_interaction;
pub mod in_object;
pub mod in_parameter;
pub mod in_relevant_shortcut;
pub mod in_relevant_shortcut_store;
pub mod in_vocabulary;
pub mod intent;
pub mod intent_definition;
pub mod intent_donation;
pub mod intent_extension;
pub mod intent_handler;
pub mod intent_response;
pub mod interaction;
pub mod preferences;
mod private;
pub mod relevant;
pub mod voice_shortcut;

pub use error::IntentsError;
pub use in_interaction::{DateInterval, Interaction, InteractionDirection, IntentHandlingStatus};
pub use in_object::{Image, IntentObject, Speakable, SpeakableString};
pub use in_parameter::IntentParameter;
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
pub use intent_handler::IntentHandlerProvider;
pub use intent_response::{IntentResponse, UserActivity};
pub use preferences::{Preferences, SiriAuthorizationStatus};
pub use voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};

pub mod prelude {
    pub use crate::error::IntentsError;
    pub use crate::in_interaction::{
        DateInterval, Interaction, InteractionDirection, IntentHandlingStatus,
    };
    pub use crate::in_object::{Image, IntentObject, Speakable, SpeakableString};
    pub use crate::in_parameter::IntentParameter;
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
    pub use crate::intent_handler::IntentHandlerProvider;
    pub use crate::intent_response::{IntentResponse, UserActivity};
    pub use crate::preferences::{Preferences, SiriAuthorizationStatus};
    pub use crate::voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};
}
