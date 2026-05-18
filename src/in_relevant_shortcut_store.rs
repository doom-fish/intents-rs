use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::sync::mpsc;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_relevant_shortcut::RelevantShortcut;
use crate::private::{RawObject, RetainedObject};

/// Wraps `INRelevantShortcutStore`.
#[derive(Debug)]
pub struct RelevantShortcutStore {
    raw: RetainedObject,
}

impl RelevantShortcutStore {
    /// Returns the shared `INRelevantShortcutStore` instance.
    pub fn default_store() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_relevant_shortcut_store_default() };
        if ptr.is_null() {
            Err(IntentsError::framework(
                "INRelevantShortcutStore is unavailable on this system".to_string(),
            ))
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "relevant shortcut store") }?,
        })
    }

    /// Sets the corresponding `relevant_shortcuts` value on `INRelevantShortcutStore`.
    pub fn set_relevant_shortcuts(
        &self,
        shortcuts: &[&RelevantShortcut],
    ) -> Result<(), IntentsError> {
        let pointers = shortcuts
            .iter()
            .map(|shortcut| shortcut.as_ptr())
            .collect::<Vec<_>>();
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe {
            ffi::inx_relevant_shortcut_store_set(
                self.as_ptr(),
                if pointers.is_empty() {
                    std::ptr::null()
                } else {
                    pointers.as_ptr()
                },
                pointers.len(),
                callback,
                context,
            );
        }
        receiver.recv().map_err(|error| {
            IntentsError::framework(format!(
                "relevant shortcut store callback channel dropped: {error}"
            ))
        })?
    }
}

impl RawObject for RelevantShortcutStore {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
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
