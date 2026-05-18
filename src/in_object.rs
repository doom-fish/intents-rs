use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INImage`.
#[derive(Debug)]
pub struct Image {
    raw: RetainedObject,
}

impl Image {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "image") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Creates a `INImage` wrapper using the corresponding initializer.
    pub fn named(name: &str) -> Result<Self, IntentsError> {
        let name = private::cstring(name, "image name")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_image_create_named(name.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating image by name") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Creates a `INImage` wrapper using the corresponding initializer.
    pub fn from_data(data: &[u8]) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_image_create_with_data(data.as_ptr(), data.len(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating image from data") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Creates a `INImage` wrapper using the corresponding initializer.
    pub fn from_url(url: &str) -> Result<Self, IntentsError> {
        let url = private::cstring(url, "image URL")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_image_create_with_url(url.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating image from URL") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the Objective-C class name for this `INImage` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }
}

impl RawObject for Image {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INSpeakableString`.
#[derive(Debug)]
pub struct SpeakableString {
    raw: RetainedObject,
}

impl SpeakableString {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "speakable string") }?,
        })
    }

    /// Creates a `INSpeakableString` wrapper.
    pub fn new(
        vocabulary_identifier: &str,
        spoken_phrase: &str,
        pronunciation_hint: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let vocabulary_identifier =
            private::cstring(vocabulary_identifier, "speakable vocabulary identifier")?;
        let spoken_phrase = private::cstring(spoken_phrase, "speakable spoken phrase")?;
        let pronunciation_hint = pronunciation_hint
            .map(|value| private::cstring(value, "speakable pronunciation hint"))
            .transpose()?;

        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_speakable_string_create(
                vocabulary_identifier.as_ptr(),
                spoken_phrase.as_ptr(),
                pronunciation_hint
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating speakable string") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the Objective-C class name for this `INSpeakableString` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INSpeakableString`.
    pub fn vocabulary_identifier(&self) -> Option<String> {
        private::string_property(self, "vocabularyIdentifier")
    }

    /// Returns the corresponding value from `INSpeakableString`.
    pub fn spoken_phrase(&self) -> Option<String> {
        private::string_property(self, "spokenPhrase")
    }

    /// Returns the corresponding value from `INSpeakableString`.
    pub fn pronunciation_hint(&self) -> Option<String> {
        private::string_property(self, "pronunciationHint")
    }
}

impl RawObject for SpeakableString {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Trait for values that can be passed to Intents.framework APIs expecting `INSpeakable`-compatible objects.
pub trait Speakable {
    /// Returns the raw Objective-C pointer for this wrapped Intents object.
    fn as_ptr(&self) -> *mut c_void;
}

impl Speakable for SpeakableString {
    fn as_ptr(&self) -> *mut c_void {
        RawObject::as_ptr(self)
    }
}

/// Wraps `INObject`.
#[derive(Debug)]
pub struct IntentObject {
    raw: RetainedObject,
}

impl IntentObject {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "INObject") }?,
        })
    }

    /// Creates a `INObject` wrapper.
    pub fn new(identifier: Option<&str>, display_string: &str) -> Result<Self, IntentsError> {
        Self::with_details(identifier, display_string, None, None, None)
    }

    /// Returns the corresponding value from `INObject`.
    pub fn with_details(
        identifier: Option<&str>,
        display_string: &str,
        pronunciation_hint: Option<&str>,
        subtitle_string: Option<&str>,
        display_image: Option<&Image>,
    ) -> Result<Self, IntentsError> {
        let identifier = identifier
            .map(|value| private::cstring(value, "INObject identifier"))
            .transpose()?;
        let display_string = private::cstring(display_string, "INObject display string")?;
        let pronunciation_hint = pronunciation_hint
            .map(|value| private::cstring(value, "INObject pronunciation hint"))
            .transpose()?;
        let subtitle_string = subtitle_string
            .map(|value| private::cstring(value, "INObject subtitle string"))
            .transpose()?;

        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_in_object_create(
                identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                display_string.as_ptr(),
                pronunciation_hint
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                subtitle_string
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                display_image.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating INObject") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the Objective-C class name for this `INObject` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INObject`.
    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    /// Returns the corresponding value from `INObject`.
    pub fn display_string(&self) -> Option<String> {
        private::string_property(self, "displayString")
    }

    /// Returns the corresponding value from `INObject`.
    pub fn pronunciation_hint(&self) -> Option<String> {
        private::string_property(self, "pronunciationHint")
    }

    /// Returns the corresponding value from `INObject`.
    pub fn subtitle_string(&self) -> Option<String> {
        private::string_property(self, "subtitleString")
    }

    /// Sets the corresponding `subtitle_string` value on `INObject`.
    pub fn set_subtitle_string(&mut self, subtitle_string: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "subtitleString", subtitle_string)
    }

    /// Returns the corresponding value from `INObject`.
    pub fn display_image(&self) -> Option<Image> {
        private::object_property(self, "displayImage").map(Image::from_retained)
    }

    /// Sets the corresponding `display_image` value on `INObject`.
    pub fn set_display_image(&mut self, display_image: Option<&Image>) -> Result<(), IntentsError> {
        private::set_object_property(self, "displayImage", display_image.map(RawObject::as_ptr))
    }

    /// Returns the number of corresponding values exposed by `INObject`.
    pub fn alternative_speakable_matches_count(&self) -> usize {
        private::array_count_property(self, "alternativeSpeakableMatches").unwrap_or_default()
    }

    /// Sets the corresponding `alternative_speakable_matches` value on `INObject`.
    pub fn set_alternative_speakable_matches(
        &mut self,
        matches: &[&SpeakableString],
    ) -> Result<(), IntentsError> {
        let values = matches
            .iter()
            .map(|value| RawObject::as_ptr(*value))
            .collect::<Vec<_>>();
        private::set_object_array_property(self, "alternativeSpeakableMatches", &values)
    }
}

impl RawObject for IntentObject {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Speakable for IntentObject {
    fn as_ptr(&self) -> *mut c_void {
        RawObject::as_ptr(self)
    }
}
