use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INParameter`.
#[derive(Debug)]
pub struct IntentParameter {
    raw: RetainedObject,
}

impl IntentParameter {
    /// Creates a `INParameter` wrapper.
    pub fn new(intent_class_name: &str, key_path: &str) -> Result<Self, IntentsError> {
        let intent_class_name = private::cstring(intent_class_name, "intent class name")?;
        let key_path = private::cstring(key_path, "intent parameter key path")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_parameter_create(intent_class_name.as_ptr(), key_path.as_ptr(), &mut error)
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating INParameter") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "INParameter") }?,
        })
    }

    /// Returns the Objective-C class name for this `INParameter` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INParameter`.
    pub fn parameter_class_name(&self) -> Option<String> {
        private::string_property(self, "parameterClass")
    }

    /// Returns the corresponding value from `INParameter`.
    pub fn parameter_key_path(&self) -> Option<String> {
        private::string_property(self, "parameterKeyPath")
    }

    /// Returns the corresponding boolean value from `INParameter`.
    pub fn is_equal_to_parameter(&self, other: &Self) -> bool {
        private::object_is_equal(self, other)
    }

    /// Sets the corresponding `index_for_sub_key_path` value on `INParameter`.
    pub fn set_index_for_sub_key_path(
        &mut self,
        index: usize,
        sub_key_path: &str,
    ) -> Result<(), IntentsError> {
        let sub_key_path = private::cstring(sub_key_path, "parameter sub key path")?;
        let index = isize::try_from(index)
            .map_err(|_| IntentsError::invalid_argument("parameter index does not fit in isize"))?;
        let ok =
            unsafe { ffi::inx_parameter_set_index(self.as_ptr(), index, sub_key_path.as_ptr()) };
        if ok {
            Ok(())
        } else {
            Err(IntentsError::framework(
                "failed to set INParameter sub-key-path index".to_string(),
            ))
        }
    }

    /// Returns the corresponding value from `INParameter`.
    pub fn index_for_sub_key_path(
        &self,
        sub_key_path: &str,
    ) -> Result<Option<usize>, IntentsError> {
        let sub_key_path = private::cstring(sub_key_path, "parameter sub key path")?;
        let mut present = false;
        let index = unsafe {
            ffi::inx_parameter_get_index(self.as_ptr(), sub_key_path.as_ptr(), &mut present)
        };
        if !present {
            return Ok(None);
        }
        usize::try_from(index).map(Some).map_err(|_| {
            IntentsError::framework("INParameter returned a negative index".to_string())
        })
    }
}

impl RawObject for IntentParameter {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
