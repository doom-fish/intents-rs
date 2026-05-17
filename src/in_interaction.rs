use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::sync::mpsc;

use crate::error::IntentsError;
use crate::ffi;
use crate::intent_definition::Intent;
use crate::intent_response::IntentResponse;
use crate::private::{self, RawObject, RetainedObject};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateInterval {
    pub start: f64,
    pub end: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IntentHandlingStatus {
    Unspecified,
    Ready,
    InProgress,
    Success,
    Failure,
    DeferredToApplication,
    UserConfirmationRequired,
    Unknown(i64),
}

impl IntentHandlingStatus {
    pub(crate) const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Unspecified,
            1 => Self::Ready,
            2 => Self::InProgress,
            3 => Self::Success,
            4 => Self::Failure,
            5 => Self::DeferredToApplication,
            6 => Self::UserConfirmationRequired,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InteractionDirection {
    Unspecified,
    Outgoing,
    Incoming,
    Unknown(i64),
}

impl InteractionDirection {
    pub(crate) const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Unspecified,
            1 => Self::Outgoing,
            2 => Self::Incoming,
            other => Self::Unknown(other),
        }
    }

    const fn raw_value(self) -> i64 {
        match self {
            Self::Unspecified => 0,
            Self::Outgoing => 1,
            Self::Incoming => 2,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug)]
pub struct Interaction {
    raw: RetainedObject,
}

impl Interaction {
    pub fn new(intent: &Intent, response: Option<&IntentResponse>) -> Result<Self, IntentsError> {
        let response_ptr = response.map_or(std::ptr::null_mut(), RawObject::as_ptr);
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_interaction_create(intent.as_ptr(), response_ptr, &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating interaction") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "interaction") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    pub fn donate(&self) -> Result<(), IntentsError> {
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe { ffi::inx_interaction_donate(self.as_ptr(), callback, context) };
        recv_result(&receiver, "interaction callback channel dropped")
    }

    pub fn delete_all() -> Result<(), IntentsError> {
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe { ffi::inx_interaction_delete_all(callback, context) };
        recv_result(&receiver, "interaction delete-all callback channel dropped")
    }

    pub fn delete_by_identifiers(identifiers: &[&str]) -> Result<(), IntentsError> {
        let cstrings = identifiers
            .iter()
            .map(|identifier| private::cstring(identifier, "interaction identifier"))
            .collect::<Result<Vec<_>, _>>()?;
        let pointers = cstrings
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let values_ptr = if pointers.is_empty() {
            std::ptr::null()
        } else {
            pointers.as_ptr()
        };

        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe {
            ffi::inx_interaction_delete_by_identifiers(
                values_ptr,
                pointers.len(),
                callback,
                context,
            );
        }
        recv_result(
            &receiver,
            "interaction delete-by-identifiers callback channel dropped",
        )
    }

    pub fn delete_by_group_identifier(group_identifier: &str) -> Result<(), IntentsError> {
        let group_identifier = private::cstring(group_identifier, "interaction group identifier")?;
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe {
            ffi::inx_interaction_delete_by_group_identifier(
                group_identifier.as_ptr(),
                callback,
                context,
            );
        }
        recv_result(
            &receiver,
            "interaction delete-by-group-identifier callback channel dropped",
        )
    }

    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    pub fn set_identifier(&mut self, identifier: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "identifier", identifier)
    }

    pub fn group_identifier(&self) -> Option<String> {
        private::string_property(self, "groupIdentifier")
    }

    pub fn set_group_identifier(&mut self, group_identifier: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "groupIdentifier", group_identifier)
    }

    pub fn direction(&self) -> InteractionDirection {
        private::integer_property(self, "direction").map_or(
            InteractionDirection::Unspecified,
            InteractionDirection::from_raw,
        )
    }

    pub fn set_direction(&mut self, direction: InteractionDirection) -> Result<(), IntentsError> {
        private::set_integer_property(self, "direction", direction.raw_value())
    }

    pub fn date_interval(&self) -> Option<DateInterval> {
        private::date_interval_property(self, "dateInterval")
            .map(|(start, end)| DateInterval { start, end })
    }

    pub fn set_date_interval(&mut self, start: f64, end: f64) -> Result<(), IntentsError> {
        private::set_date_interval_property(self, "dateInterval", start, end)
    }

    pub fn intent_handling_status(&self) -> IntentHandlingStatus {
        private::integer_property(self, "intentHandlingStatus").map_or(
            IntentHandlingStatus::Unspecified,
            IntentHandlingStatus::from_raw,
        )
    }

    pub fn intent(&self) -> Option<Intent> {
        private::object_property(self, "intent").map(Intent::from_retained)
    }

    pub fn intent_response(&self) -> Option<IntentResponse> {
        private::object_property(self, "intentResponse").map(IntentResponse::from_retained)
    }
}

impl RawObject for Interaction {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

fn recv_result(
    receiver: &mpsc::Receiver<Result<(), IntentsError>>,
    context: &str,
) -> Result<(), IntentsError> {
    receiver
        .recv()
        .map_err(|error| IntentsError::framework(format!("{context}: {error}")))?
}

unsafe extern "C" fn callback(context: *mut c_void, error: *const c_char) {
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: context is a valid Box<mpsc::Sender<...>> obtained via Box::into_raw
        // in the calling function; ownership is transferred to this callback.
        let sender =
            unsafe { Box::from_raw(context.cast::<mpsc::Sender<Result<(), IntentsError>>>()) };
        let result = if error.is_null() {
            Ok(())
        } else {
            let message = unsafe { CStr::from_ptr(error) }
                .to_string_lossy()
                .into_owned();
            Err(IntentsError::framework(message))
        };
        let _ = sender.send(result);
    }))
    .is_err()
    {
        eprintln!(
            "intents: panic in callback caught at C ABI boundary; channel will return RecvError"
        );
    }
}
