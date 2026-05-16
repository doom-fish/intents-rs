use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

#[derive(Debug)]
pub struct IntentResolutionResult {
    raw: RetainedObject,
}

impl IntentResolutionResult {
    pub fn needs_value() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_resolution_result_needs_value() };
        unsafe { Self::from_owned(ptr) }
    }

    pub fn not_required() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_resolution_result_not_required() };
        unsafe { Self::from_owned(ptr) }
    }

    pub fn unsupported() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_resolution_result_unsupported() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent resolution result") }?,
        })
    }

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }
}

impl RawObject for IntentResolutionResult {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
