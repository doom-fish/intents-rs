use core::ffi::c_void;
use std::ops::Deref;

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
        self.user_activity()
            .and_then(|activity| activity.activity_type())
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SendMessageIntentResponseCode {
    Unspecified,
    Ready,
    InProgress,
    Success,
    Failure,
    FailureRequiringAppLaunch,
    FailureMessageServiceNotAvailable,
    FailureRequiringInAppAuthentication,
    Other(i64),
}

impl SendMessageIntentResponseCode {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Unspecified,
            1 => Self::Ready,
            2 => Self::InProgress,
            3 => Self::Success,
            4 => Self::Failure,
            5 => Self::FailureRequiringAppLaunch,
            6 => Self::FailureMessageServiceNotAvailable,
            7 => Self::FailureRequiringInAppAuthentication,
            other => Self::Other(other),
        }
    }

    const fn raw_value(self) -> i64 {
        match self {
            Self::Unspecified => 0,
            Self::Ready => 1,
            Self::InProgress => 2,
            Self::Success => 3,
            Self::Failure => 4,
            Self::FailureRequiringAppLaunch => 5,
            Self::FailureMessageServiceNotAvailable => 6,
            Self::FailureRequiringInAppAuthentication => 7,
            Self::Other(raw) => raw,
        }
    }
}

#[derive(Debug)]
pub struct SendMessageIntentResponse(IntentResponse);

impl SendMessageIntentResponse {
    pub const OBJC_CLASS: &'static str = "INSendMessageIntentResponse";

    pub fn new(
        code: SendMessageIntentResponseCode,
        user_activity: Option<&UserActivity>,
    ) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_send_message_intent_response_create(
                code.raw_value(),
                user_activity.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating send-message intent response") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self(unsafe { IntentResponse::from_owned(ptr) }?))
    }

    pub fn class_name(&self) -> String {
        self.0.class_name()
    }

    pub fn code(&self) -> SendMessageIntentResponseCode {
        self.0.result_code().map_or(
            SendMessageIntentResponseCode::Unspecified,
            SendMessageIntentResponseCode::from_raw,
        )
    }
}

impl TryFrom<IntentResponse> for SendMessageIntentResponse {
    type Error = IntentsError;

    fn try_from(response: IntentResponse) -> Result<Self, Self::Error> {
        let actual = response.class_name();
        if actual == Self::OBJC_CLASS {
            Ok(Self(response))
        } else {
            Err(IntentsError::UnexpectedClass {
                expected: Self::OBJC_CLASS,
                actual,
            })
        }
    }
}

impl From<SendMessageIntentResponse> for IntentResponse {
    fn from(response: SendMessageIntentResponse) -> Self {
        response.0
    }
}

impl Deref for SendMessageIntentResponse {
    type Target = IntentResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RawObject for SendMessageIntentResponse {
    fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}
