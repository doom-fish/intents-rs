use core::ffi::c_void;
use std::ops::Deref;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_object::Image;
use crate::intent_donation::IntentDonationMetadata;
use crate::intent_response::UserActivity;
use crate::private::{self, RawObject, RetainedObject};

#[derive(Debug)]
pub struct Intent {
    raw: RetainedObject,
}

impl Intent {
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

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    pub fn intent_description(&self) -> Option<String> {
        private::string_property(self, "intentDescription")
    }

    pub fn suggested_invocation_phrase(&self) -> Option<String> {
        private::string_property(self, "suggestedInvocationPhrase")
    }

    pub fn set_suggested_invocation_phrase(&mut self, phrase: &str) -> Result<(), IntentsError> {
        private::set_string_property(self, "suggestedInvocationPhrase", phrase)
    }

    pub fn donation_metadata(&self) -> Option<IntentDonationMetadata> {
        private::object_property(self, "donationMetadata").map(IntentDonationMetadata::from_retained)
    }

    pub fn set_donation_metadata(
        &mut self,
        metadata: Option<&IntentDonationMetadata>,
    ) -> Result<(), IntentsError> {
        private::set_object_property(self, "donationMetadata", metadata.map(RawObject::as_ptr))
    }

    pub fn key_image(&self) -> Option<Image> {
        let ptr = unsafe { ffi::inx_intent_copy_key_image(self.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Image::from_owned(ptr) }.ok()
        }
    }

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

    pub fn new(intent: &Intent) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_shortcut_create_with_intent(intent.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating shortcut") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

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

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn intent(&self) -> Option<Intent> {
        private::object_property(self, "intent").map(Intent::from_retained)
    }

    pub fn user_activity(&self) -> Option<UserActivity> {
        private::object_property(self, "userActivity").map(UserActivity::from_retained)
    }

    pub fn user_activity_type(&self) -> Option<String> {
        self.user_activity().and_then(|activity| activity.activity_type())
    }
}

impl RawObject for Shortcut {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

macro_rules! typed_intent {
    ($name:ident, $objc_class:literal) => {
        #[derive(Debug)]
        pub struct $name(Intent);

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

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
    pub fn recipients_count(&self) -> Option<usize> {
        private::array_count_property(self, "recipients")
    }

    pub fn content(&self) -> Option<String> {
        private::string_property(self, "content")
    }

    pub fn service_name(&self) -> Option<String> {
        private::string_property(self, "serviceName")
    }

    pub fn conversation_identifier(&self) -> Option<String> {
        private::string_property(self, "conversationIdentifier")
    }
}

impl SearchForMessagesIntent {
    pub fn search_terms(&self) -> Result<Option<Vec<String>>, IntentsError> {
        private::string_array_property(self, "searchTerms")
    }

    pub fn identifiers(&self) -> Result<Option<Vec<String>>, IntentsError> {
        private::string_array_property(self, "identifiers")
    }

    pub fn attributes(&self) -> Option<i64> {
        private::integer_property(self, "attributes")
    }
}

impl StartCallIntent {
    pub fn contacts_count(&self) -> Option<usize> {
        private::array_count_property(self, "contacts")
    }

    pub fn audio_route(&self) -> Option<i64> {
        private::integer_property(self, "audioRoute")
    }

    pub fn destination_type(&self) -> Option<i64> {
        private::integer_property(self, "destinationType")
    }

    pub fn call_capability(&self) -> Option<i64> {
        private::integer_property(self, "callCapability")
    }
}

impl PlayMediaIntent {
    pub fn media_items_count(&self) -> Option<usize> {
        private::array_count_property(self, "mediaItems")
    }

    pub fn play_shuffled(&self) -> Option<bool> {
        private::bool_property(self, "playShuffled")
    }

    pub fn playback_repeat_mode(&self) -> Option<i64> {
        private::integer_property(self, "playbackRepeatMode")
    }

    pub fn resume_playback(&self) -> Option<bool> {
        private::bool_property(self, "resumePlayback")
    }

    pub fn playback_speed(&self) -> Option<f64> {
        private::double_property(self, "playbackSpeed")
    }
}

impl AddTasksIntent {
    pub fn task_titles_count(&self) -> Option<usize> {
        private::array_count_property(self, "taskTitles")
    }

    pub fn priority(&self) -> Option<i64> {
        private::integer_property(self, "priority")
    }
}
