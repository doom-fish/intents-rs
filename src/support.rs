use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_object::IntentObject;
use crate::private::{self, RawObject, RetainedObject};

macro_rules! objc_wrapper {
    ($name:ident, $objc_class:literal, $context:literal) => {
        #[derive(Debug)]
        pub struct $name {
            raw: RetainedObject,
        }

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

            pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
                Ok(Self {
                    raw: unsafe { RetainedObject::from_owned(ptr, $context) }?,
                })
            }

            #[allow(dead_code)]
            pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
                Self { raw }
            }

            #[allow(dead_code)]
            pub(crate) fn new_blank() -> Result<Self, IntentsError> {
                Ok(Self::from_retained(private::create_blank_object(
                    Self::OBJC_CLASS,
                    $context,
                )?))
            }

            pub fn class_name(&self) -> String {
                private::class_name(self)
            }
        }

        impl RawObject for $name {
            fn as_ptr(&self) -> *mut c_void {
                self.raw.as_ptr()
            }
        }
    };
}

objc_wrapper!(Placemark, "CLPlacemark", "placemark");
objc_wrapper!(ObjectSection, "INObjectSection", "object section");
objc_wrapper!(ObjectCollection, "INObjectCollection", "object collection");

pub fn intents_version_number() -> f64 {
    unsafe { ffi::inx_intents_version_number() }
}

pub fn intents_version_string() -> String {
    let ptr = unsafe { ffi::inx_intents_version_string() };
    unsafe { private::take_string(ptr) }.unwrap_or_default()
}

pub fn deferred_localized_intents_string(format: &str) -> Result<String, IntentsError> {
    deferred_localized_intents_string_from_table(format, None)
}

pub fn deferred_localized_intents_string_from_table(
    format: &str,
    table: Option<&str>,
) -> Result<String, IntentsError> {
    let format = private::cstring(format, "localized intents format")?;
    let table = table
        .map(|value| private::cstring(value, "localized intents table"))
        .transpose()?;
    let ptr = unsafe {
        ffi::inx_deferred_localized_intents_string_copy(
            format.as_ptr(),
            table
                .as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr()),
        )
    };
    unsafe { private::take_string(ptr) }.ok_or_else(|| {
        IntentsError::framework("failed to create deferred localized intents string".to_string())
    })
}

impl Placemark {
    pub fn new(name: Option<&str>) -> Result<Self, IntentsError> {
        let name = name
            .map(|value| private::cstring(value, "placemark name"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_placemark_create(
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating placemark") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn name(&self) -> Option<String> {
        private::string_property(self, "name")
    }
}

impl ObjectSection {
    pub fn new(title: Option<&str>, items: &[&IntentObject]) -> Result<Self, IntentsError> {
        let title = title
            .map(|value| private::cstring(value, "object section title"))
            .transpose()?;
        let items = items.iter().map(|item| item.as_ptr()).collect::<Vec<_>>();
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_object_section_create(
                title
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                if items.is_empty() {
                    std::ptr::null()
                } else {
                    items.as_ptr()
                },
                items.len(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating object section") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn title(&self) -> Option<String> {
        private::string_property(self, "title")
    }

    pub fn items_count(&self) -> usize {
        private::array_count_property(self, "items").unwrap_or_default()
    }
}

impl ObjectCollection {
    pub fn new_with_items(items: &[&IntentObject]) -> Result<Self, IntentsError> {
        let items = items.iter().map(|item| item.as_ptr()).collect::<Vec<_>>();
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_object_collection_create_with_items(
                if items.is_empty() {
                    std::ptr::null()
                } else {
                    items.as_ptr()
                },
                items.len(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating object collection") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn new_with_sections(sections: &[&ObjectSection]) -> Result<Self, IntentsError> {
        let sections = sections
            .iter()
            .map(|section| section.as_ptr())
            .collect::<Vec<_>>();
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_object_collection_create_with_sections(
                if sections.is_empty() {
                    std::ptr::null()
                } else {
                    sections.as_ptr()
                },
                sections.len(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating object collection") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn sections_count(&self) -> usize {
        private::array_count_property(self, "sections").unwrap_or_default()
    }

    pub fn all_items_count(&self) -> usize {
        private::array_count_property(self, "allItems").unwrap_or_default()
    }

    pub fn uses_indexed_collation(&self) -> bool {
        private::bool_property(self, "usesIndexedCollation").unwrap_or_default()
    }

    pub fn set_uses_indexed_collation(&mut self, value: bool) -> Result<(), IntentsError> {
        private::set_bool_property(self, "usesIndexedCollation", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let placemark = Placemark::new(Some("Test Place"))?;
        let item = IntentObject::new(Some("one"), "One")?;
        let section = ObjectSection::new(Some("Section"), &[&item])?;
        let mut collection = ObjectCollection::new_with_sections(&[&section])?;
        collection.set_uses_indexed_collation(true)?;

        assert_eq!(placemark.class_name(), Placemark::OBJC_CLASS);
        assert_eq!(placemark.name().as_deref(), Some("Test Place"));
        assert_eq!(section.items_count(), 1);
        assert_eq!(collection.sections_count(), 1);
        assert!(collection.uses_indexed_collation());
        assert!(intents_version_number() > 0.0);
        assert!(!intents_version_string().is_empty());
        assert!(deferred_localized_intents_string("Hello").is_ok());
        Ok(())
    }
}
