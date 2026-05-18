use core::fmt;

/// Errors surfaced by the safe Intents.framework bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntentsError {
    /// Corresponds to the `InvalidArgument` case of `Intents.framework`.
    InvalidArgument(String),
    /// Corresponds to the `Framework` case of `Intents.framework`.
    Framework(String),
    /// Indicates that a wrapper expected one Intents.framework class but received another.
    UnexpectedClass {
        /// Expected Objective-C class name from Intents.framework.
        expected: &'static str,
        /// Actual Objective-C class name returned by Intents.framework.
        actual: String,
    },
}

impl IntentsError {
    pub(crate) fn invalid_argument(message: impl Into<String>) -> Self {
        Self::InvalidArgument(message.into())
    }

    pub(crate) fn framework(message: impl Into<String>) -> Self {
        Self::Framework(message.into())
    }
}

impl fmt::Display for IntentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Framework(message) => write!(f, "Intents.framework error: {message}"),
            Self::UnexpectedClass { expected, actual } => {
                write!(f, "expected Objective-C class {expected}, got {actual}")
            }
        }
    }
}

impl std::error::Error for IntentsError {}
