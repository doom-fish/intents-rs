use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

#[derive(Debug)]
pub struct UserActivity {
    raw: RetainedObject,
}

impl UserActivity {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "user activity") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    pub fn new(activity_type: &str) -> Result<Self, IntentsError> {
        let activity_type = private::cstring(activity_type, "user activity type")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_user_activity_create(activity_type.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating user activity") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn activity_type(&self) -> Option<String> {
        private::string_property(self, "activityType")
    }
}

impl RawObject for UserActivity {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

#[derive(Debug)]
pub struct IntentResponse {
    raw: RetainedObject,
}

impl IntentResponse {
    pub fn new() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_intent_response_create() };
        unsafe { Self::from_owned(ptr) }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent response") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn result_code(&self) -> Option<i64> {
        private::integer_property(self, "code")
    }

    pub fn user_activity(&self) -> Option<UserActivity> {
        private::object_property(self, "userActivity").map(UserActivity::from_retained)
    }

    pub fn user_activity_type(&self) -> Option<String> {
        self.user_activity().and_then(|activity| activity.activity_type())
    }

    pub fn set_user_activity(
        &mut self,
        user_activity: Option<&UserActivity>,
    ) -> Result<(), IntentsError> {
        private::set_object_property(self, "userActivity", user_activity.map(RawObject::as_ptr))
    }

    pub fn set_user_activity_type(&mut self, activity_type: &str) -> Result<(), IntentsError> {
        let activity = UserActivity::new(activity_type)?;
        self.set_user_activity(Some(&activity))
    }
}

impl RawObject for IntentResponse {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
