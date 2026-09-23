use core::ffi::{c_char, c_void};
use std::ffi::CStr;

use doom_fish_utils::completion::SyncCompletion;
use doom_fish_utils::panic_safe::{catch_user_panic, catch_user_panic_result};

use crate::error::IntentsError;
use crate::ffi;
use crate::intent::Shortcut;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INVoiceShortcut`.
#[derive(Debug)]
pub struct VoiceShortcut {
    raw: RetainedObject,
}

impl VoiceShortcut {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "voice shortcut") }?,
        })
    }

    /// Returns the corresponding value from `INVoiceShortcut`.
    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    /// Returns the corresponding value from `INVoiceShortcut`.
    pub fn invocation_phrase(&self) -> Option<String> {
        private::string_property(self, "invocationPhrase")
    }

    /// Returns the corresponding value from `INVoiceShortcut`.
    pub fn shortcut(&self) -> Option<Shortcut> {
        private::object_property(self, "shortcut").map(Shortcut::from_retained)
    }
}

impl RawObject for VoiceShortcut {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INVoiceShortcutCenter`.
#[derive(Debug)]
pub struct VoiceShortcutCenter {
    raw: RetainedObject,
}

impl VoiceShortcutCenter {
    /// Returns the shared `INVoiceShortcutCenter` instance.
    pub fn shared() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_voice_shortcut_center_shared() };
        if ptr.is_null() {
            Err(IntentsError::framework(
                "INVoiceShortcutCenter is unavailable on this macOS version".to_string(),
            ))
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "voice shortcut center") }?,
        })
    }

    /// Wraps the corresponding method on `INVoiceShortcutCenter`.
    pub fn get_all_voice_shortcuts(&self) -> Result<Vec<VoiceShortcut>, IntentsError> {
        let (completion, context) = SyncCompletion::<Vec<usize>>::new();
        unsafe { ffi::inx_voice_shortcut_center_get_all(self.as_ptr(), array_callback, context) };
        completion
            .wait()
            .map_err(IntentsError::framework)?
            .into_iter()
            .map(|address| unsafe { VoiceShortcut::from_owned(address as *mut c_void) })
            .collect()
    }

    /// Returns the corresponding value from `INVoiceShortcutCenter`.
    pub fn get_voice_shortcut(
        &self,
        identifier: &str,
    ) -> Result<Option<VoiceShortcut>, IntentsError> {
        let identifier = private::cstring(identifier, "voice shortcut identifier")?;
        let (completion, context) = SyncCompletion::<usize>::new();
        unsafe {
            ffi::inx_voice_shortcut_center_get_by_identifier(
                self.as_ptr(),
                identifier.as_ptr(),
                object_callback,
                context,
            );
        }
        match completion.wait().map_err(IntentsError::framework)? {
            0 => Ok(None),
            address => unsafe { VoiceShortcut::from_owned(address as *mut c_void) }.map(Some),
        }
    }
}

impl RawObject for VoiceShortcutCenter {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

unsafe fn callback_error(error: *const c_char) -> Option<String> {
    (!error.is_null()).then(|| CStr::from_ptr(error).to_string_lossy().into_owned())
}

unsafe extern "C" fn array_callback(
    context: *mut c_void,
    objects: *mut *mut c_void,
    count: usize,
    error: *const c_char,
) {
    let outcome = catch_user_panic_result("intents voice shortcut array callback", || {
        if let Some(message) = unsafe { callback_error(error) } {
            return Err(message);
        }
        if count == 0 || objects.is_null() {
            return Ok(Vec::new());
        }
        let objects = unsafe { std::slice::from_raw_parts(objects.cast_const(), count) };
        Ok(objects.iter().map(|object| *object as usize).collect())
    })
    .unwrap_or_else(|| Err("voice shortcut callback panicked".to_owned()));
    catch_user_panic("intents voice shortcut array completion", || unsafe {
        SyncCompletion::complete_with_result(context, outcome);
    });
}

unsafe extern "C" fn object_callback(
    context: *mut c_void,
    object: *mut c_void,
    error: *const c_char,
) {
    let outcome = catch_user_panic_result("intents voice shortcut callback", || {
        unsafe { callback_error(error) }.map_or(Ok(object as usize), Err)
    })
    .unwrap_or_else(|| Err("voice shortcut callback panicked".to_owned()));
    catch_user_panic("intents voice shortcut completion", || unsafe {
        SyncCompletion::complete_with_result(context, outcome);
    });
}
