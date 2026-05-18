use core::ffi::c_void;
use std::ops::Deref;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_object::Image;
use crate::intent_donation::IntentDonationMetadata;
use crate::intent_response::UserActivity;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INIntent`.
#[derive(Debug)]
pub struct Intent {
    raw: RetainedObject,
}

impl Intent {
    /// Creates a `INIntent` wrapper.
    pub fn new() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_create() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Returns the Objective-C class name for this `INIntent` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INIntent`.
    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    /// Returns the corresponding value from `INIntent`.
    pub fn intent_description(&self) -> Option<String> {
        private::string_property(self, "intentDescription")
    }

    /// Returns the corresponding value from `INIntent`.
    pub fn suggested_invocation_phrase(&self) -> Option<String> {
        private::string_property(self, "suggestedInvocationPhrase")
    }

    /// Sets the corresponding `suggested_invocation_phrase` value on `INIntent`.
    pub fn set_suggested_invocation_phrase(&mut self, phrase: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "suggestedInvocationPhrase", phrase)
    }

    /// Returns the corresponding value from `INIntent`.
    pub fn donation_metadata(&self) -> Option<IntentDonationMetadata> {
        private::object_property(self, "donationMetadata")
            .map(IntentDonationMetadata::from_retained)
    }

    /// Sets the corresponding `donation_metadata` value on `INIntent`.
    pub fn set_donation_metadata(
        &mut self,
        metadata: Option<&IntentDonationMetadata>,
    ) -> Result<(), IntentsError> {
        private::set_object_property(self, "donationMetadata", metadata.map(RawObject::as_ptr))
    }

    /// Returns the corresponding value from `INIntent`.
    pub fn key_image(&self) -> Option<Image> {
        let ptr = unsafe { ffi::inx_intent_copy_key_image(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Image::from_owned(ptr) }.ok()
        }
    }

    /// Returns the corresponding value from `INIntent`.
    pub fn image_for_parameter_named(
        &self,
        parameter_name: &str,
    ) -> Result<Option<Image>, IntentsError> {
        let parameter_name = private::cstring(parameter_name, "intent parameter name")?;
        let ptr = unsafe {
            ffi::inx_intent_copy_image_for_parameter_named(self.as_ptr(), parameter_name.as_ptr())
        };
        if ptr.is_null() {
            Ok(None)
        } else {
            unsafe { Image::from_owned(ptr) }.map(Some)
        }
    }

    /// Sets the corresponding `image_for_parameter_named` value on `INIntent`.
    pub fn set_image_for_parameter_named(
        &mut self,
        parameter_name: &str,
        image: Option<&Image>,
    ) -> Result<(), IntentsError> {
        let parameter_name_value = parameter_name.to_string();
        let parameter_name = private::cstring(parameter_name, "intent parameter name")?;
        let ok = unsafe {
            ffi::inx_intent_set_image_for_parameter_named(
                self.as_ptr(),
                parameter_name.as_ptr(),
                image.map_or(std::ptr::null_mut(), RawObject::as_ptr),
            )
        };
        if ok {
            Ok(())
        } else {
            Err(IntentsError::framework(format!(
                "failed to set image for intent parameter '{parameter_name_value}'"
            )))
        }
    }
}

impl RawObject for Intent {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INShortcut`.
#[derive(Debug)]
pub struct Shortcut {
    raw: RetainedObject,
}

impl Shortcut {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "shortcut") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Creates a `INShortcut` wrapper.
    pub fn new(intent: &Intent) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_shortcut_create_with_intent(intent.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating shortcut") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Creates a `INShortcut` wrapper using the corresponding initializer.
    pub fn new_with_user_activity(user_activity: &UserActivity) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_shortcut_create_with_user_activity(user_activity.as_ptr(), &mut error)
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating shortcut from user activity") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the Objective-C class name for this `INShortcut` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    /// Returns the corresponding value from `INShortcut`.
    pub fn intent(&self) -> Option<Intent> {
        private::object_property(self, "intent").map(Intent::from_retained)
    }

    /// Returns the corresponding value from `INShortcut`.
    pub fn user_activity(&self) -> Option<UserActivity> {
        private::object_property(self, "userActivity").map(UserActivity::from_retained)
    }

    /// Returns the corresponding value from `INShortcut`.
    pub fn user_activity_type(&self) -> Option<String> {
        self.user_activity()
            .and_then(|activity| activity.activity_type())
    }
}

impl RawObject for Shortcut {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

macro_rules! typed_intent {
    ($name:ident, $objc_class:literal) => {
        #[doc = concat!("Wraps `", $objc_class, "`.")]
        #[derive(Debug)]
        pub struct $name(Intent);

        impl $name {
            #[doc = concat!("Objective-C class name for `", $objc_class, "`.")]
            pub const OBJC_CLASS: &'static str = $objc_class;

            #[doc = concat!("Returns the Objective-C class name for this `", $objc_class, "` instance.")]
            pub fn class_name(&self) -> String {
                self.0.class_name()
            }
        }

        impl TryFrom<Intent> for $name {
            type Error = IntentsError;

            fn try_from(intent: Intent) -> Result<Self, Self::Error> {
                let actual = intent.class_name();
                if actual == Self::OBJC_CLASS {
                    Ok(Self(intent))
                } else {
                    Err(IntentsError::UnexpectedClass {
                        expected: Self::OBJC_CLASS,
                        actual,
                    })
                }
            }
        }

        impl From<$name> for Intent {
            fn from(intent: $name) -> Self {
                intent.0
            }
        }

        impl Deref for $name {
            type Target = Intent;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl RawObject for $name {
            fn as_ptr(&self) -> *mut c_void {
                self.0.as_ptr()
            }
        }
    };
}

typed_intent!(SendMessageIntent, "INSendMessageIntent");
typed_intent!(SearchForMessagesIntent, "INSearchForMessagesIntent");
typed_intent!(StartCallIntent, "INStartCallIntent");
typed_intent!(PlayMediaIntent, "INPlayMediaIntent");
typed_intent!(AddTasksIntent, "INAddTasksIntent");
typed_intent!(SetTimerIntent, "INSetTimerIntent");

impl SendMessageIntent {
    /// Returns the number of corresponding values exposed by `INSendMessageIntent`.
    pub fn recipients_count(&self) -> Option<usize> {
        private::array_count_property(self, "recipients")
    }

    /// Returns the corresponding value from `INSendMessageIntent`.
    pub fn content(&self) -> Option<String> {
        private::string_property(self, "content")
    }

    /// Returns the corresponding value from `INSendMessageIntent`.
    pub fn service_name(&self) -> Option<String> {
        private::string_property(self, "serviceName")
    }

    /// Returns the corresponding value from `INSendMessageIntent`.
    pub fn conversation_identifier(&self) -> Option<String> {
        private::string_property(self, "conversationIdentifier")
    }
}

impl SearchForMessagesIntent {
    /// Wraps the corresponding method on `INSearchForMessagesIntent`.
    pub fn search_terms(&self) -> Result<Option<Vec<String>>, IntentsError> {
        private::string_array_property(self, "searchTerms")
    }

    /// Wraps the corresponding method on `INSearchForMessagesIntent`.
    pub fn identifiers(&self) -> Result<Option<Vec<String>>, IntentsError> {
        private::string_array_property(self, "identifiers")
    }

    /// Returns the corresponding value from `INSearchForMessagesIntent`.
    pub fn attributes(&self) -> Option<i64> {
        private::integer_property(self, "attributes")
    }
}

impl StartCallIntent {
    /// Returns the number of corresponding values exposed by `INStartCallIntent`.
    pub fn contacts_count(&self) -> Option<usize> {
        private::array_count_property(self, "contacts")
    }

    /// Returns the corresponding value from `INStartCallIntent`.
    pub fn audio_route(&self) -> Option<i64> {
        private::integer_property(self, "audioRoute")
    }

    /// Returns the corresponding value from `INStartCallIntent`.
    pub fn destination_type(&self) -> Option<i64> {
        private::integer_property(self, "destinationType")
    }

    /// Returns the corresponding value from `INStartCallIntent`.
    pub fn call_capability(&self) -> Option<i64> {
        private::integer_property(self, "callCapability")
    }
}

impl PlayMediaIntent {
    /// Returns the number of corresponding values exposed by `INPlayMediaIntent`.
    pub fn media_items_count(&self) -> Option<usize> {
        private::array_count_property(self, "mediaItems")
    }

    /// Returns the corresponding value from `INPlayMediaIntent`.
    pub fn play_shuffled(&self) -> Option<bool> {
        private::bool_property(self, "playShuffled")
    }

    /// Returns the corresponding value from `INPlayMediaIntent`.
    pub fn playback_repeat_mode(&self) -> Option<i64> {
        private::integer_property(self, "playbackRepeatMode")
    }

    /// Returns the corresponding value from `INPlayMediaIntent`.
    pub fn resume_playback(&self) -> Option<bool> {
        private::bool_property(self, "resumePlayback")
    }

    /// Returns the corresponding value from `INPlayMediaIntent`.
    pub fn playback_speed(&self) -> Option<f64> {
        private::double_property(self, "playbackSpeed")
    }
}

impl AddTasksIntent {
    /// Returns the number of corresponding values exposed by `INAddTasksIntent`.
    pub fn task_titles_count(&self) -> Option<usize> {
        private::array_count_property(self, "taskTitles")
    }

    /// Returns the corresponding value from `INAddTasksIntent`.
    pub fn priority(&self) -> Option<i64> {
        private::integer_property(self, "priority")
    }
}
