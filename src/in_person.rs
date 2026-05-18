use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_object::Image;
use crate::private::{self, RawObject, RetainedObject};

/// Mirrors `INPersonHandleType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PersonHandleType {
    /// Stores an unknown raw value from `INPersonHandleType`.
    Unknown,
    /// Corresponds to the `EmailAddress` case of `INPersonHandleType`.
    EmailAddress,
    /// Corresponds to the `PhoneNumber` case of `INPersonHandleType`.
    PhoneNumber,
    /// Stores an unknown raw value from `INPersonHandleType`.
    Other(i64),
}

impl PersonHandleType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Unknown,
            1 => Self::EmailAddress,
            2 => Self::PhoneNumber,
            other => Self::Other(other),
        }
    }

    const fn raw_value(self) -> i64 {
        match self {
            Self::Unknown => 0,
            Self::EmailAddress => 1,
            Self::PhoneNumber => 2,
            Self::Other(raw) => raw,
        }
    }
}

/// Mirrors `INPersonSuggestionType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PersonSuggestionType {
    /// Corresponds to the `None` case of `INPersonSuggestionType`.
    None,
    /// Corresponds to the `SocialProfile` case of `INPersonSuggestionType`.
    SocialProfile,
    /// Corresponds to the `InstantMessageAddress` case of `INPersonSuggestionType`.
    InstantMessageAddress,
    /// Stores an unknown raw value from `INPersonSuggestionType`.
    Other(i64),
}

impl PersonSuggestionType {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::SocialProfile,
            2 => Self::InstantMessageAddress,
            other => Self::Other(other),
        }
    }

    const fn raw_value(self) -> i64 {
        match self {
            Self::None => 0,
            Self::SocialProfile => 1,
            Self::InstantMessageAddress => 2,
            Self::Other(raw) => raw,
        }
    }
}

/// Wraps `INPersonHandle`.
#[derive(Debug)]
pub struct PersonHandle {
    raw: RetainedObject,
}

impl PersonHandle {
    /// Creates a `INPersonHandle` wrapper.
    pub fn new(value: Option<&str>, handle_type: PersonHandleType) -> Result<Self, IntentsError> {
        let value = value
            .map(|value| private::cstring(value, "person handle value"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_person_handle_create(
                value
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                handle_type.raw_value(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating person handle") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "person handle") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Returns the Objective-C class name for this `INPersonHandle` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INPersonHandle`.
    pub fn value(&self) -> Option<String> {
        private::string_property(self, "value")
    }

    /// Returns the corresponding value from `INPersonHandle`.
    pub fn handle_type(&self) -> PersonHandleType {
        private::integer_property(self, "type")
            .map_or(PersonHandleType::Unknown, PersonHandleType::from_raw)
    }
}

impl RawObject for PersonHandle {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INPerson`.
#[derive(Debug)]
pub struct Person {
    raw: RetainedObject,
}

impl Person {
    /// Creates a `INPerson` wrapper.
    pub fn new(
        person_handle: &PersonHandle,
        display_name: Option<&str>,
    ) -> Result<Self, IntentsError> {
        Self::with_details(
            person_handle,
            display_name,
            None,
            None,
            None,
            &[],
            PersonSuggestionType::None,
        )
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn with_details(
        person_handle: &PersonHandle,
        display_name: Option<&str>,
        image: Option<&Image>,
        contact_identifier: Option<&str>,
        custom_identifier: Option<&str>,
        aliases: &[&PersonHandle],
        suggestion_type: PersonSuggestionType,
    ) -> Result<Self, IntentsError> {
        let display_name = display_name
            .map(|value| private::cstring(value, "person display name"))
            .transpose()?;
        let contact_identifier = contact_identifier
            .map(|value| private::cstring(value, "person contact identifier"))
            .transpose()?;
        let custom_identifier = custom_identifier
            .map(|value| private::cstring(value, "person custom identifier"))
            .transpose()?;
        let aliases = aliases
            .iter()
            .map(|alias| RawObject::as_ptr(*alias))
            .collect::<Vec<_>>();
        let aliases_ptr = if aliases.is_empty() {
            std::ptr::null()
        } else {
            aliases.as_ptr()
        };

        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_person_create(
                person_handle.as_ptr(),
                display_name
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                image.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                contact_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                custom_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                aliases_ptr,
                aliases.len(),
                suggestion_type.raw_value(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating person") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "person") }?,
        })
    }

    /// Returns the Objective-C class name for this `INPerson` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn person_handle(&self) -> Option<PersonHandle> {
        private::object_property(self, "personHandle").map(PersonHandle::from_retained)
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn display_name(&self) -> Option<String> {
        private::string_property(self, "displayName")
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn image(&self) -> Option<Image> {
        private::object_property(self, "image").map(Image::from_retained)
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn contact_identifier(&self) -> Option<String> {
        private::string_property(self, "contactIdentifier")
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn custom_identifier(&self) -> Option<String> {
        private::string_property(self, "customIdentifier")
    }

    /// Returns the number of corresponding values exposed by `INPerson`.
    pub fn aliases_count(&self) -> usize {
        private::array_count_property(self, "aliases").unwrap_or_default()
    }

    /// Returns the corresponding value from `INPerson`.
    pub fn suggestion_type(&self) -> PersonSuggestionType {
        private::integer_property(self, "suggestionType")
            .map_or(PersonSuggestionType::None, PersonSuggestionType::from_raw)
    }

    /// Returns the corresponding boolean value from `INPerson`.
    pub fn is_me(&self) -> bool {
        private::bool_property(self, "isMe").unwrap_or_default()
    }
}

impl RawObject for Person {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
