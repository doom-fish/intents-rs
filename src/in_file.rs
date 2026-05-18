use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INFile`.
#[derive(Debug)]
pub struct IntentFile {
    raw: RetainedObject,
}

impl IntentFile {
    /// Creates a `INFile` wrapper using the corresponding initializer.
    pub fn from_data(
        data: &[u8],
        filename: &str,
        type_identifier: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let filename = private::cstring(filename, "file name")?;
        let type_identifier = type_identifier
            .map(|value| private::cstring(value, "file type identifier"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_file_create_with_data(
                data.as_ptr(),
                data.len(),
                filename.as_ptr(),
                type_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating intent file from data") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Creates a `INFile` wrapper using the corresponding initializer.
    pub fn from_file_url(
        file_url: &str,
        filename: Option<&str>,
        type_identifier: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let file_url = private::cstring(file_url, "file URL")?;
        let filename = filename
            .map(|value| private::cstring(value, "file name"))
            .transpose()?;
        let type_identifier = type_identifier
            .map(|value| private::cstring(value, "file type identifier"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_file_create_with_file_url(
                file_url.as_ptr(),
                filename
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                type_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating intent file from file URL") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent file") }?,
        })
    }

    /// Returns the Objective-C class name for this `INFile` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Wraps the corresponding method on `INFile`.
    pub fn data(&self) -> Result<Vec<u8>, IntentsError> {
        let ptr = unsafe { ffi::inx_file_copy_data_json(self.as_ptr()) };
        let Some(json) = (unsafe { private::take_string(ptr) }) else {
            return Err(IntentsError::framework("failed to copy INFile data"));
        };
        serde_json::from_str(&json).map_err(|error| {
            IntentsError::framework(format!("failed to parse INFile data JSON: {error}"))
        })
    }

    /// Returns the corresponding value from `INFile`.
    pub fn filename(&self) -> Option<String> {
        private::string_property(self, "filename")
    }

    /// Sets the corresponding `filename` value on `INFile`.
    pub fn set_filename(&mut self, filename: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "filename", filename)
    }

    /// Returns the corresponding value from `INFile`.
    pub fn type_identifier(&self) -> Option<String> {
        private::string_property(self, "typeIdentifier")
    }

    /// Returns the corresponding value from `INFile`.
    pub fn file_url(&self) -> Option<String> {
        private::string_property(self, "fileURL")
    }
}

impl RawObject for IntentFile {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
