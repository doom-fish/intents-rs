#![allow(clippy::missing_errors_doc)]

use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use crate::error::IntentsError;
use crate::ffi;

#[derive(Debug)]
pub struct RetainedObject(NonNull<c_void>);

impl RetainedObject {
    /// # Safety
    ///
    /// `ptr` must be a valid, non-null Objective-C object pointer that the caller
    /// is transferring ownership of (i.e. the refcount has been +1 for this caller).
    /// Passing a null pointer or a pointer that is not a valid Objective-C object
    /// is undefined behaviour.
    pub unsafe fn from_owned(
        ptr: *mut c_void,
        context: &'static str,
    ) -> Result<Self, IntentsError> {
        NonNull::new(ptr)
            .map(Self)
            .ok_or_else(|| IntentsError::framework(format!("{context} returned a null object")))
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Drop for RetainedObject {
    fn drop(&mut self) {
        unsafe { ffi::inx_object_release(self.0.as_ptr()) };
    }
}

pub trait RawObject {
    fn as_ptr(&self) -> *mut c_void;
}

impl RawObject for RetainedObject {
    fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

pub fn cstring(value: &str, context: &str) -> Result<CString, IntentsError> {
    CString::new(value).map_err(|error| {
        IntentsError::invalid_argument(format!("{context} contains a NUL byte: {error}"))
    })
}

fn property_key(key: &str) -> CString {
    CString::new(key).expect("static Objective-C property keys never contain NUL bytes")
}

/// # Safety
///
/// `ptr` must be either null or a pointer to a valid, null-terminated C string
/// allocated by the Swift bridge (via `inx_string_free`-compatible allocator).
/// The pointer must not be used after this call returns; ownership is transferred
/// to this function.
pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::inx_string_free(ptr);
    Some(value)
}

/// # Safety
///
/// Same requirements as [`take_string`]: `ptr` must be either null or a valid
/// C string pointer transferred to this function.
pub unsafe fn take_error(ptr: *mut c_char, context: &'static str) -> IntentsError {
    IntentsError::framework(
        take_string(ptr).unwrap_or_else(|| format!("{context} failed with an unknown error")),
    )
}

pub fn class_name(object: &impl RawObject) -> String {
    let ptr = unsafe { ffi::inx_object_class_name(object.as_ptr()) };
    unsafe { take_string(ptr) }.unwrap_or_else(|| "<unknown>".to_string())
}

pub fn string_property(object: &impl RawObject, key: &str) -> Option<String> {
    let key = property_key(key);
    let ptr = unsafe { ffi::inx_object_copy_string_property(object.as_ptr(), key.as_ptr()) };
    unsafe { take_string(ptr) }
}

pub fn object_is_equal(lhs: &impl RawObject, rhs: &impl RawObject) -> bool {
    unsafe { ffi::inx_object_is_equal(lhs.as_ptr(), rhs.as_ptr()) }
}

pub fn object_property(object: &impl RawObject, key: &str) -> Option<RetainedObject> {
    let key = property_key(key);
    let ptr = unsafe { ffi::inx_object_copy_object_property(object.as_ptr(), key.as_ptr()) };
    if ptr.is_null() {
        return None;
    }
    unsafe { RetainedObject::from_owned(ptr, "object property") }.ok()
}

pub fn string_array_property(
    object: &impl RawObject,
    key: &str,
) -> Result<Option<Vec<String>>, IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let ptr =
        unsafe { ffi::inx_object_copy_string_array_property_json(object.as_ptr(), key.as_ptr()) };
    let Some(json) = (unsafe { take_string(ptr) }) else {
        return Ok(None);
    };

    let values = serde_json::from_str(&json).map_err(|error| {
        IntentsError::framework(format!(
            "failed to parse JSON array for property '{key_name}': {error}"
        ))
    })?;
    Ok(Some(values))
}

pub fn integer_property(object: &impl RawObject, key: &str) -> Option<i64> {
    let key = property_key(key);
    let mut present = false;
    let value = unsafe {
        ffi::inx_object_get_integer_property(object.as_ptr(), key.as_ptr(), &mut present)
    };
    present.then_some(value)
}

pub fn double_property(object: &impl RawObject, key: &str) -> Option<f64> {
    let key = property_key(key);
    let mut present = false;
    let value =
        unsafe { ffi::inx_object_get_double_property(object.as_ptr(), key.as_ptr(), &mut present) };
    present.then_some(value)
}

pub fn bool_property(object: &impl RawObject, key: &str) -> Option<bool> {
    let key = property_key(key);
    let mut present = false;
    let value =
        unsafe { ffi::inx_object_get_bool_property(object.as_ptr(), key.as_ptr(), &mut present) };
    present.then_some(value)
}

pub fn array_count_property(object: &impl RawObject, key: &str) -> Option<usize> {
    let key = property_key(key);
    let mut present = false;
    let value = unsafe {
        ffi::inx_object_get_array_count_property(object.as_ptr(), key.as_ptr(), &mut present)
    };
    present.then_some(value)
}

pub fn date_interval_property(object: &impl RawObject, key: &str) -> Option<(f64, f64)> {
    let key = property_key(key);
    let mut start = 0.0;
    let mut end = 0.0;
    let mut present = false;
    let ok = unsafe {
        ffi::inx_object_get_date_interval_property(
            object.as_ptr(),
            key.as_ptr(),
            &mut start,
            &mut end,
            &mut present,
        )
    };
    (ok && present).then_some((start, end))
}

pub fn set_string_property(
    object: &impl RawObject,
    key: &str,
    value: &str,
) -> Result<(), IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let value = cstring(value, &key_name)?;
    let ok = unsafe {
        ffi::inx_object_set_string_property(object.as_ptr(), key.as_ptr(), value.as_ptr())
    };
    if ok {
        Ok(())
    } else {
        Err(IntentsError::framework(format!(
            "failed to set Objective-C string property '{key_name}'"
        )))
    }
}

pub fn create_blank_object(
    class_name: &str,
    context: &'static str,
) -> Result<RetainedObject, IntentsError> {
    let class_name = cstring(class_name, context)?;
    let mut error = std::ptr::null_mut();
    let ptr = unsafe { ffi::inx_object_create_blank(class_name.as_ptr(), &mut error) };
    if ptr.is_null() {
        Err(unsafe { take_error(error, context) })
    } else {
        unsafe { RetainedObject::from_owned(ptr, context) }
    }
}

pub fn class_conforms_to_protocol(
    class_name: &str,
    protocol_name: &str,
) -> Result<bool, IntentsError> {
    let class_name = cstring(class_name, "Objective-C class name")?;
    let protocol_name = cstring(protocol_name, "Objective-C protocol name")?;
    Ok(unsafe { ffi::inx_class_conforms_to_protocol(class_name.as_ptr(), protocol_name.as_ptr()) })
}

pub fn set_integer_property(
    object: &impl RawObject,
    key: &str,
    value: i64,
) -> Result<(), IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let ok = unsafe { ffi::inx_object_set_integer_property(object.as_ptr(), key.as_ptr(), value) };
    if ok {
        Ok(())
    } else {
        Err(IntentsError::framework(format!(
            "failed to set Objective-C integer property '{key_name}'"
        )))
    }
}

pub fn set_bool_property(
    object: &impl RawObject,
    key: &str,
    value: bool,
) -> Result<(), IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let ok = unsafe { ffi::inx_object_set_bool_property(object.as_ptr(), key.as_ptr(), value) };
    if ok {
        Ok(())
    } else {
        Err(IntentsError::framework(format!(
            "failed to set Objective-C bool property '{key_name}'"
        )))
    }
}

pub fn set_object_property(
    object: &impl RawObject,
    key: &str,
    value: Option<*mut c_void>,
) -> Result<(), IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let ok = unsafe {
        ffi::inx_object_set_object_property(
            object.as_ptr(),
            key.as_ptr(),
            value.unwrap_or(std::ptr::null_mut()),
        )
    };
    if ok {
        Ok(())
    } else {
        Err(IntentsError::framework(format!(
            "failed to set Objective-C object property '{key_name}'"
        )))
    }
}

pub fn set_date_interval_property(
    object: &impl RawObject,
    key: &str,
    start: f64,
    end: f64,
) -> Result<(), IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let ok = unsafe {
        ffi::inx_object_set_date_interval_property(object.as_ptr(), key.as_ptr(), start, end)
    };
    if ok {
        Ok(())
    } else {
        Err(IntentsError::framework(format!(
            "failed to set Objective-C date interval property '{key_name}'"
        )))
    }
}

pub fn set_object_array_property(
    object: &impl RawObject,
    key: &str,
    values: &[*mut c_void],
) -> Result<(), IntentsError> {
    let key_name = key.to_string();
    let key = property_key(key);
    let values_ptr = if values.is_empty() {
        std::ptr::null()
    } else {
        values.as_ptr()
    };
    let ok = unsafe {
        ffi::inx_object_set_object_array_property(
            object.as_ptr(),
            key.as_ptr(),
            values_ptr,
            values.len(),
        )
    };
    if ok {
        Ok(())
    } else {
        Err(IntentsError::framework(format!(
            "failed to set Objective-C object-array property '{key_name}'"
        )))
    }
}
