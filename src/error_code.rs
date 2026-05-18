#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IntentErrorCode {
    InteractionOperationNotSupported,
    DonatingInteraction,
    DeletingAllInteractions,
    DeletingInteractionWithIdentifiers,
    DeletingInteractionWithGroupIdentifier,
    IntentSupportedByMultipleExtension,
    RestrictedIntentsNotSupportedByExtension,
    NoHandlerProvidedForIntent,
    InvalidIntentName,
    NoAppAvailable,
    RequestTimedOut,
    MissingInformation,
    InvalidUserVocabularyFileLocation,
    ExtensionLaunchingTimeout,
    ExtensionBringUpFailed,
    ImageGeneric,
    ImageNoServiceAvailable,
    ImageStorageFailed,
    ImageLoadingFailed,
    ImageRetrievalFailed,
    ImageProxyLoop,
    ImageProxyInvalid,
    ImageProxyTimeout,
    ImageServiceFailure,
    ImageScalingFailed,
    PermissionDenied,
    VoiceShortcutCreationFailed,
    VoiceShortcutGetFailed,
    VoiceShortcutDeleteFailed,
    EncodingGeneric,
    EncodingFailed,
    DecodingGeneric,
    UnableToCreateAppIntentRepresentation,
    NoAppIntent,
    Other(i64),
}

impl IntentErrorCode {
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            1900 => Self::InteractionOperationNotSupported,
            1901 => Self::DonatingInteraction,
            1902 => Self::DeletingAllInteractions,
            1903 => Self::DeletingInteractionWithIdentifiers,
            1904 => Self::DeletingInteractionWithGroupIdentifier,
            2001 => Self::IntentSupportedByMultipleExtension,
            2002 => Self::RestrictedIntentsNotSupportedByExtension,
            2003 => Self::NoHandlerProvidedForIntent,
            2004 => Self::InvalidIntentName,
            2005 => Self::NoAppAvailable,
            3001 => Self::RequestTimedOut,
            3002 => Self::MissingInformation,
            4000 => Self::InvalidUserVocabularyFileLocation,
            5000 => Self::ExtensionLaunchingTimeout,
            5001 => Self::ExtensionBringUpFailed,
            6000 => Self::ImageGeneric,
            6001 => Self::ImageNoServiceAvailable,
            6002 => Self::ImageStorageFailed,
            6003 => Self::ImageLoadingFailed,
            6004 => Self::ImageRetrievalFailed,
            6005 => Self::ImageProxyLoop,
            6006 => Self::ImageProxyInvalid,
            6007 => Self::ImageProxyTimeout,
            6008 => Self::ImageServiceFailure,
            6009 => Self::ImageScalingFailed,
            6010 => Self::PermissionDenied,
            7000 => Self::VoiceShortcutCreationFailed,
            7001 => Self::VoiceShortcutGetFailed,
            7002 => Self::VoiceShortcutDeleteFailed,
            8000 => Self::EncodingGeneric,
            8001 => Self::EncodingFailed,
            9000 => Self::DecodingGeneric,
            10000 => Self::UnableToCreateAppIntentRepresentation,
            10001 => Self::NoAppIntent,
            other => Self::Other(other),
        }
    }

    pub const fn raw_value(self) -> i64 {
        match self {
            Self::InteractionOperationNotSupported => 1900,
            Self::DonatingInteraction => 1901,
            Self::DeletingAllInteractions => 1902,
            Self::DeletingInteractionWithIdentifiers => 1903,
            Self::DeletingInteractionWithGroupIdentifier => 1904,
            Self::IntentSupportedByMultipleExtension => 2001,
            Self::RestrictedIntentsNotSupportedByExtension => 2002,
            Self::NoHandlerProvidedForIntent => 2003,
            Self::InvalidIntentName => 2004,
            Self::NoAppAvailable => 2005,
            Self::RequestTimedOut => 3001,
            Self::MissingInformation => 3002,
            Self::InvalidUserVocabularyFileLocation => 4000,
            Self::ExtensionLaunchingTimeout => 5000,
            Self::ExtensionBringUpFailed => 5001,
            Self::ImageGeneric => 6000,
            Self::ImageNoServiceAvailable => 6001,
            Self::ImageStorageFailed => 6002,
            Self::ImageLoadingFailed => 6003,
            Self::ImageRetrievalFailed => 6004,
            Self::ImageProxyLoop => 6005,
            Self::ImageProxyInvalid => 6006,
            Self::ImageProxyTimeout => 6007,
            Self::ImageServiceFailure => 6008,
            Self::ImageScalingFailed => 6009,
            Self::PermissionDenied => 6010,
            Self::VoiceShortcutCreationFailed => 7000,
            Self::VoiceShortcutGetFailed => 7001,
            Self::VoiceShortcutDeleteFailed => 7002,
            Self::EncodingGeneric => 8000,
            Self::EncodingFailed => 8001,
            Self::DecodingGeneric => 9000,
            Self::UnableToCreateAppIntentRepresentation => 10000,
            Self::NoAppIntent => 10001,
            Self::Other(raw) => raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntentErrorCode;

    #[test]
    fn intent_error_code_roundtrip() {
        assert_eq!(
            IntentErrorCode::from_raw(1901),
            IntentErrorCode::DonatingInteraction
        );
        assert_eq!(IntentErrorCode::NoAppIntent.raw_value(), 10001);
    }
}
