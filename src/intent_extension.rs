use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::intent_definition::Intent;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INExtension`.
#[derive(Debug)]
pub struct IntentExtension {
    raw: RetainedObject,
}

impl IntentExtension {
    /// Creates a `INExtension` wrapper.
    pub fn new() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_extension_create() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent extension") }?,
        })
    }

    /// Returns the Objective-C class name for this `INExtension` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INExtension`.
    pub fn handler_class_name_for_intent(&self, intent: &Intent) -> Option<String> {
        let ptr = unsafe {
            ffi::inx_intent_extension_copy_handler_class_name_for_intent(
                self.as_ptr(),
                intent.as_ptr(),
            )
        };
        unsafe { private::take_string(ptr) }
    }
}

impl RawObject for IntentExtension {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
