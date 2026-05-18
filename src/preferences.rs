use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::sync::mpsc;

use crate::error::IntentsError;
use crate::ffi;

/// Mirrors `INSiriAuthorizationStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SiriAuthorizationStatus {
    /// Corresponds to the `NotDetermined` case of `INSiriAuthorizationStatus`.
    NotDetermined,
    /// Corresponds to the `Restricted` case of `INSiriAuthorizationStatus`.
    Restricted,
    /// Corresponds to the `Denied` case of `INSiriAuthorizationStatus`.
    Denied,
    /// Corresponds to the `Authorized` case of `INSiriAuthorizationStatus`.
    Authorized,
    /// Stores an unknown raw value from `INSiriAuthorizationStatus`.
    Unknown(i64),
}

impl SiriAuthorizationStatus {
    pub(crate) const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::NotDetermined,
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::Authorized,
            other => Self::Unknown(other),
        }
    }
}

/// Namespace wrapper for `INPreferences` APIs.
#[derive(Debug, Default, Clone, Copy)]
pub struct Preferences;

impl Preferences {
    /// Returns the corresponding value from `INPreferences`.
    pub fn siri_authorization_status() -> SiriAuthorizationStatus {
        let raw = unsafe { ffi::inx_preferences_siri_authorization_status() };
        SiriAuthorizationStatus::from_raw(raw)
    }

    /// Wraps the corresponding request API on `INPreferences`.
    pub fn request_siri_authorization() -> Result<SiriAuthorizationStatus, IntentsError> {
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe { ffi::inx_preferences_request_siri_authorization(request_callback, context) };
        receiver.recv().map_err(|error| {
            IntentsError::framework(format!("authorization callback channel dropped: {error}"))
        })?
    }
}

unsafe extern "C" fn request_callback(context: *mut c_void, status: i64, error: *const c_char) {
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: context is a valid Box<mpsc::Sender<...>> obtained via Box::into_raw
        // in the calling function; ownership is transferred to this callback.
        let sender = unsafe {
            Box::from_raw(
                context.cast::<mpsc::Sender<Result<SiriAuthorizationStatus, IntentsError>>>(),
            )
        };
        let result = if error.is_null() {
            Ok(SiriAuthorizationStatus::from_raw(status))
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
