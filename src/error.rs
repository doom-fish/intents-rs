use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntentsError {
    InvalidArgument(String),
    Framework(String),
    UnexpectedClass {
        expected: &'static str,
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
