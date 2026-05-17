use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::sync::mpsc;

use crate::error::IntentsError;
use crate::ffi;
use crate::intent::Shortcut;
use crate::private::{self, RawObject, RetainedObject};

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

    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    pub fn invocation_phrase(&self) -> Option<String> {
        private::string_property(self, "invocationPhrase")
    }

    pub fn shortcut(&self) -> Option<Shortcut> {
        private::object_property(self, "shortcut").map(Shortcut::from_retained)
    }
}

impl RawObject for VoiceShortcut {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

#[derive(Debug)]
pub struct VoiceShortcutCenter {
    raw: RetainedObject,
}

impl VoiceShortcutCenter {
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

    pub fn get_all_voice_shortcuts(&self) -> Result<Vec<VoiceShortcut>, IntentsError> {
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe { ffi::inx_voice_shortcut_center_get_all(self.as_ptr(), array_callback, context) };
        receiver.recv().map_err(|error| {
            IntentsError::framework(format!("voice shortcut callback channel dropped: {error}"))
        })?
    }

    pub fn get_voice_shortcut(
        &self,
        identifier: &str,
    ) -> Result<Option<VoiceShortcut>, IntentsError> {
        let identifier = private::cstring(identifier, "voice shortcut identifier")?;
        let (sender, receiver) = mpsc::channel();
        let context = Box::into_raw(Box::new(sender)).cast::<c_void>();
        unsafe {
            ffi::inx_voice_shortcut_center_get_by_identifier(
                self.as_ptr(),
                identifier.as_ptr(),
                object_callback,
                context,
            );
        }
        receiver.recv().map_err(|error| {
            IntentsError::framework(format!("voice shortcut callback channel dropped: {error}"))
        })?
    }
}

impl RawObject for VoiceShortcutCenter {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

unsafe extern "C" fn array_callback(
    context: *mut c_void,
    objects: *mut *mut c_void,
    count: usize,
    error: *const c_char,
) {
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: context is a valid Box<mpsc::Sender<...>> obtained via Box::into_raw
        // in the calling function; ownership is transferred to this callback.
        let sender = unsafe {
            Box::from_raw(context.cast::<mpsc::Sender<Result<Vec<VoiceShortcut>, IntentsError>>>())
        };
        let result = if error.is_null() {
            let objects = if count == 0 {
                &[][..]
            } else {
                unsafe { std::slice::from_raw_parts(objects.cast_const(), count) }
            };
            objects
                .iter()
                .copied()
                .map(|ptr| unsafe { VoiceShortcut::from_owned(ptr) })
                .collect()
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

unsafe extern "C" fn object_callback(
    context: *mut c_void,
    object: *mut c_void,
    error: *const c_char,
) {
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // SAFETY: context is a valid Box<mpsc::Sender<...>> obtained via Box::into_raw
        // in the calling function; ownership is transferred to this callback.
        let sender = unsafe {
            Box::from_raw(
                context.cast::<mpsc::Sender<Result<Option<VoiceShortcut>, IntentsError>>>(),
            )
        };
        let result = if error.is_null() {
            if object.is_null() {
                Ok(None)
            } else {
                unsafe { VoiceShortcut::from_owned(object) }.map(Some)
            }
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
