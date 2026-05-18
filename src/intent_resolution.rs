use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INIntentResolutionResult`.
#[derive(Debug)]
pub struct IntentResolutionResult {
    raw: RetainedObject,
}

impl IntentResolutionResult {
    /// Wraps the corresponding method on `INIntentResolutionResult`.
    pub fn needs_value() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_resolution_result_needs_value() };
        unsafe { Self::from_owned(ptr) }
    }

    /// Wraps the corresponding method on `INIntentResolutionResult`.
    pub fn not_required() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_resolution_result_not_required() };
        unsafe { Self::from_owned(ptr) }
    }

    /// Wraps the corresponding method on `INIntentResolutionResult`.
    pub fn unsupported() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_resolution_result_unsupported() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent resolution result") }?,
        })
    }

    #[allow(dead_code)]
    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Returns the Objective-C class name for this `INIntentResolutionResult` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }
}

impl RawObject for IntentResolutionResult {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
