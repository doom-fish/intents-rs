use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::intent_definition::Intent;
use crate::private::{self, RawObject, RetainedObject};

#[derive(Debug)]
pub struct IntentHandlerProvider {
    raw: RetainedObject,
}

impl IntentHandlerProvider {
    pub fn new() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_handler_provider_create() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent handler provider") }?,
        })
    }

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn register_handler(
        &mut self,
        intent_class_name: &str,
        handler_name: &str,
    ) -> Result<(), IntentsError> {
        let intent_class_name = private::cstring(intent_class_name, "intent handler class name")?;
        let handler_name = private::cstring(handler_name, "intent handler name")?;
        let ok = unsafe {
            ffi::inx_intent_handler_provider_register(
                self.as_ptr(),
                intent_class_name.as_ptr(),
                handler_name.as_ptr(),
            )
        };
        if ok {
            Ok(())
        } else {
            Err(IntentsError::framework(
                "failed to register intent handler mapping".to_string(),
            ))
        }
    }

    pub fn handler_name_for_intent(&self, intent: &Intent) -> Result<Option<String>, IntentsError> {
        let ptr = unsafe {
            ffi::inx_intent_handler_provider_copy_handler_name_for_intent(
                self.as_ptr(),
                intent.as_ptr(),
            )
        };
        Ok(unsafe { private::take_string(ptr) })
    }
}

impl RawObject for IntentHandlerProvider {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

#[derive(Debug)]
pub struct StartCallIntentHandling {
    raw: RetainedObject,
}

impl StartCallIntentHandling {
    pub fn new() -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_start_call_intent_handling_create(&mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating start-call intent handling helper") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "start-call intent handling helper") }?,
        })
    }

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn simulate_handle(&mut self) -> Result<(), IntentsError> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::inx_start_call_intent_handling_simulate_handle(self.as_ptr(), &mut error)
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe { private::take_error(error, "simulating start-call handle") })
        }
    }

    pub fn simulate_confirm(&mut self) -> Result<(), IntentsError> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::inx_start_call_intent_handling_simulate_confirm(self.as_ptr(), &mut error)
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe { private::take_error(error, "simulating start-call confirmation") })
        }
    }

    pub fn handle_call_count(&self) -> usize {
        private::integer_property(self, "handleCallCount")
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or_default()
    }

    pub fn confirm_call_count(&self) -> usize {
        private::integer_property(self, "confirmCallCount")
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or_default()
    }

    pub fn last_intent_class_name(&self) -> Option<String> {
        private::string_property(self, "lastIntentClassName")
    }
}

impl RawObject for StartCallIntentHandling {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
