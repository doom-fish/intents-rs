use core::ffi::c_void;
use std::ops::Deref;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_interaction::Interaction;
use crate::intent_response::{IntentResponse, UserActivity};
use crate::private::{self, RawObject};

macro_rules! simple_enum {
    ($name:ident { $($variant:ident = $raw:expr,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)*
            Other(i64),
        }

        impl $name {
            #[allow(dead_code)]
            const fn from_raw(raw: i64) -> Self {
                match raw {
                    $($raw => Self::$variant,)*
                    other => Self::Other(other),
                }
            }

            #[allow(dead_code)]
            const fn raw_value(self) -> i64 {
                match self {
                    $(Self::$variant => $raw,)*
                    Self::Other(raw) => raw,
                }
            }
        }
    };
}

macro_rules! typed_response_extra {
    ($name:ident, $objc_class:literal, $code_ty:ident) => {
        #[derive(Debug)]
        pub struct $name(IntentResponse);

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

            #[allow(dead_code)]
            pub(crate) fn new_blank() -> Result<Self, IntentsError> {
                let raw = private::create_blank_object(Self::OBJC_CLASS, "typed response")?;
                Self::try_from(IntentResponse::from_retained(raw))
            }

            pub fn new(
                code: $code_ty,
                user_activity: Option<&UserActivity>,
            ) -> Result<Self, IntentsError> {
                let class_name =
                    private::cstring(Self::OBJC_CLASS, "intent response subclass class name")?;
                let mut error = std::ptr::null_mut();
                let ptr = unsafe {
                    ffi::inx_intent_response_subclass_create(
                        class_name.as_ptr(),
                        code.raw_value(),
                        user_activity.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                        &mut error,
                    )
                };
                if ptr.is_null() {
                    Err(unsafe { private::take_error(error, "creating intent response subclass") })
                } else {
                    Self::try_from(unsafe { IntentResponse::from_owned(ptr) }?)
                }
            }

            pub fn class_name(&self) -> String {
                self.0.class_name()
            }

            pub fn code(&self) -> $code_ty {
                self.0
                    .result_code()
                    .map_or($code_ty::Other(-1), $code_ty::from_raw)
            }
        }

        impl TryFrom<IntentResponse> for $name {
            type Error = IntentsError;

            fn try_from(response: IntentResponse) -> Result<Self, Self::Error> {
                let actual = response.class_name();
                if actual == Self::OBJC_CLASS {
                    Ok(Self(response))
                } else {
                    Err(IntentsError::UnexpectedClass {
                        expected: Self::OBJC_CLASS,
                        actual,
                    })
                }
            }
        }

        impl From<$name> for IntentResponse {
            fn from(response: $name) -> Self {
                response.0
            }
        }

        impl Deref for $name {
            type Target = IntentResponse;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl RawObject for $name {
            fn as_ptr(&self) -> *mut c_void {
                self.0.as_ptr()
            }
        }
    };
}

simple_enum!(AnswerCallIntentResponseCode {
    Unspecified = 0,
    Ready = 1,
    ContinueInApp = 2,
    InProgress = 3,
    Success = 4,
    Failure = 5,
    FailureRequiringAppLaunch = 6,
});

simple_enum!(EditMessageIntentResponseCode {
    Unspecified = 0,
    Ready = 1,
    InProgress = 2,
    Success = 3,
    Failure = 4,
    FailureRequiringAppLaunch = 5,
    FailureMessageNotFound = 6,
    FailurePastEditTimeLimit = 7,
    FailureMessageTypeUnsupported = 8,
    FailureUnsupportedOnService = 9,
    FailureMessageServiceNotAvailable = 10,
    FailureRequiringInAppAuthentication = 11,
});

simple_enum!(GetReservationDetailsIntentResponseCode {
    Unspecified = 0,
    Ready = 1,
    InProgress = 2,
    Success = 3,
    Failure = 4,
    FailureRequiringAppLaunch = 5,
});

simple_enum!(HangUpCallIntentResponseCode {
    Unspecified = 0,
    Ready = 1,
    InProgress = 2,
    Success = 3,
    Failure = 4,
    FailureRequiringAppLaunch = 5,
    FailureNoCallToHangUp = 6,
});

simple_enum!(ShareFocusStatusIntentResponseCode {
    Unspecified = 0,
    Ready = 1,
    InProgress = 2,
    Success = 3,
    Failure = 4,
    FailureRequiringAppLaunch = 5,
});

simple_enum!(UnsendMessagesIntentResponseCode {
    Unspecified = 0,
    Ready = 1,
    InProgress = 2,
    Success = 3,
    Failure = 4,
    FailureRequiringAppLaunch = 5,
    FailureMessageNotFound = 6,
    FailurePastUnsendTimeLimit = 7,
    FailureMessageTypeUnsupported = 8,
    FailureUnsupportedOnService = 9,
    FailureMessageServiceNotAvailable = 10,
    FailureRequiringInAppAuthentication = 11,
});

typed_response_extra!(
    AnswerCallIntentResponse,
    "INAnswerCallIntentResponse",
    AnswerCallIntentResponseCode
);
typed_response_extra!(
    EditMessageIntentResponse,
    "INEditMessageIntentResponse",
    EditMessageIntentResponseCode
);
typed_response_extra!(
    GetReservationDetailsIntentResponse,
    "INGetReservationDetailsIntentResponse",
    GetReservationDetailsIntentResponseCode
);
typed_response_extra!(
    HangUpCallIntentResponse,
    "INHangUpCallIntentResponse",
    HangUpCallIntentResponseCode
);
typed_response_extra!(
    ShareFocusStatusIntentResponse,
    "INShareFocusStatusIntentResponse",
    ShareFocusStatusIntentResponseCode
);
typed_response_extra!(
    UnsendMessagesIntentResponse,
    "INUnsendMessagesIntentResponse",
    UnsendMessagesIntentResponseCode
);

impl AnswerCallIntentResponse {
    pub fn call_records_count(&self) -> usize {
        private::array_count_property(self, "callRecords").unwrap_or_default()
    }
}

impl GetReservationDetailsIntentResponse {
    pub fn reservations_count(&self) -> usize {
        private::array_count_property(self, "reservations").unwrap_or_default()
    }
}

impl UserActivity {
    pub fn interaction(&self) -> Option<Interaction> {
        private::object_property(self, "interaction").map(Interaction::from_retained)
    }

    pub fn suggested_invocation_phrase(&self) -> Option<String> {
        private::string_property(self, "suggestedInvocationPhrase")
    }

    pub fn set_suggested_invocation_phrase(&mut self, phrase: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "suggestedInvocationPhrase", phrase)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let mut activity = UserActivity::new("com.doomfish.intents-rs.response-extras")?;
        activity.set_suggested_invocation_phrase("Reply now")?;

        let answer =
            AnswerCallIntentResponse::new(AnswerCallIntentResponseCode::Ready, Some(&activity))?;
        let edit = EditMessageIntentResponse::new(
            EditMessageIntentResponseCode::Success,
            Some(&activity),
        )?;
        let reservation = GetReservationDetailsIntentResponse::new(
            GetReservationDetailsIntentResponseCode::Ready,
            Some(&activity),
        )?;
        let hang =
            HangUpCallIntentResponse::new(HangUpCallIntentResponseCode::Success, Some(&activity))?;
        let focus = ShareFocusStatusIntentResponse::new(
            ShareFocusStatusIntentResponseCode::Ready,
            Some(&activity),
        )?;
        let unsend = UnsendMessagesIntentResponse::new(
            UnsendMessagesIntentResponseCode::InProgress,
            Some(&activity),
        )?;

        assert_eq!(
            activity.suggested_invocation_phrase().as_deref(),
            Some("Reply now")
        );
        assert_eq!(answer.code(), AnswerCallIntentResponseCode::Ready);
        assert_eq!(edit.code(), EditMessageIntentResponseCode::Success);
        assert_eq!(
            reservation.code(),
            GetReservationDetailsIntentResponseCode::Ready
        );
        assert_eq!(hang.code(), HangUpCallIntentResponseCode::Success);
        assert_eq!(focus.code(), ShareFocusStatusIntentResponseCode::Ready);
        assert_eq!(unsend.code(), UnsendMessagesIntentResponseCode::InProgress);
        Ok(())
    }
}
