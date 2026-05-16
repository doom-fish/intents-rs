#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

pub mod error;
pub mod ffi;
pub mod intent;
pub mod interaction;
pub mod preferences;
mod private;
pub mod relevant;
pub mod voice_shortcut;

pub use error::IntentsError;
pub use intent::{
    AddTasksIntent, Intent, IntentResponse, PlayMediaIntent, SearchForMessagesIntent,
    SendMessageIntent, SetTimerIntent, Shortcut, StartCallIntent,
};
pub use interaction::{DateInterval, IntentHandlingStatus, Interaction, InteractionDirection};
pub use preferences::{Preferences, SiriAuthorizationStatus};
pub use relevant::{
    DailyRoutineSituation, RelevanceProvider, RelevantShape, RelevantShortcut, RelevantShortcutRole,
};
pub use voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};

pub mod prelude {
    pub use crate::error::IntentsError;
    pub use crate::intent::{
        AddTasksIntent, Intent, IntentResponse, PlayMediaIntent, SearchForMessagesIntent,
        SendMessageIntent, SetTimerIntent, Shortcut, StartCallIntent,
    };
    pub use crate::interaction::{
        DateInterval, IntentHandlingStatus, Interaction, InteractionDirection,
    };
    pub use crate::preferences::{Preferences, SiriAuthorizationStatus};
    pub use crate::relevant::{
        DailyRoutineSituation, RelevanceProvider, RelevantShape, RelevantShortcut,
        RelevantShortcutRole,
    };
    pub use crate::voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};
}
