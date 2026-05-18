use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

/// Mirrors `INFocusStatusAuthorizationStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FocusStatusAuthorizationStatus {
    /// Corresponds to the `NotDetermined` case of `INFocusStatusAuthorizationStatus`.
    NotDetermined,
    /// Corresponds to the `Restricted` case of `INFocusStatusAuthorizationStatus`.
    Restricted,
    /// Corresponds to the `Denied` case of `INFocusStatusAuthorizationStatus`.
    Denied,
    /// Corresponds to the `Authorized` case of `INFocusStatusAuthorizationStatus`.
    Authorized,
    /// Stores an unknown raw value from `INFocusStatusAuthorizationStatus`.
    Other(i64),
}

impl FocusStatusAuthorizationStatus {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::NotDetermined,
            1 => Self::Restricted,
            2 => Self::Denied,
            3 => Self::Authorized,
            other => Self::Other(other),
        }
    }
}

/// Wraps `INFocusStatus`.
#[derive(Debug)]
pub struct FocusStatus {
    raw: RetainedObject,
}

impl FocusStatus {
    /// Objective-C class name for `INFocusStatus`.
    pub const OBJC_CLASS: &'static str = "INFocusStatus";

    /// Creates a `INFocusStatus` wrapper.
    pub fn new(is_focused: Option<bool>) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_focus_status_create(
                is_focused.is_some(),
                is_focused.unwrap_or_default(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating focus status") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "focus status") }?,
        })
    }

    #[allow(dead_code)]
    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Returns the Objective-C class name for this `INFocusStatus` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding boolean value from `INFocusStatus`.
    pub fn is_focused(&self) -> Option<bool> {
        private::bool_property(self, "isFocused")
    }
}

impl RawObject for FocusStatus {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INFocusStatusCenter`.
#[derive(Debug)]
pub struct FocusStatusCenter {
    raw: RetainedObject,
}

impl FocusStatusCenter {
    /// Objective-C class name for `INFocusStatusCenter`.
    pub const OBJC_CLASS: &'static str = "INFocusStatusCenter";

    /// Returns the shared `INFocusStatusCenter` instance.
    pub fn default_center() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_focus_status_center_copy_default() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "focus status center") }?,
        })
    }

    /// Returns the Objective-C class name for this `INFocusStatusCenter` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INFocusStatusCenter`.
    pub fn focus_status(&self) -> Option<FocusStatus> {
        private::object_property(self, "focusStatus").map(FocusStatus::from_retained)
    }

    /// Returns the corresponding value from `INFocusStatusCenter`.
    pub fn authorization_status(&self) -> FocusStatusAuthorizationStatus {
        private::integer_property(self, "authorizationStatus").map_or(
            FocusStatusAuthorizationStatus::NotDetermined,
            FocusStatusAuthorizationStatus::from_raw,
        )
    }

    /// Wraps the corresponding request API on `INFocusStatusCenter`.
    pub fn request_authorization(&self) -> Result<FocusStatusAuthorizationStatus, IntentsError> {
        let mut status = 0;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::inx_focus_status_center_request_authorization(
                self.as_ptr(),
                &mut status,
                &mut error,
            )
        };
        if ok {
            Ok(FocusStatusAuthorizationStatus::from_raw(status))
        } else {
            Err(unsafe { private::take_error(error, "requesting focus status authorization") })
        }
    }
}

impl RawObject for FocusStatusCenter {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let status = FocusStatus::new(Some(true))?;
        let center = FocusStatusCenter::default_center()?;

        assert_eq!(status.class_name(), FocusStatus::OBJC_CLASS);
        assert_eq!(status.is_focused(), Some(true));
        assert_eq!(center.class_name(), FocusStatusCenter::OBJC_CLASS);
        Ok(())
    }
}
