use core::ffi::c_void;
use std::ops::Deref;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

/// Wraps `INIntentDonationMetadata`.
#[derive(Debug)]
pub struct IntentDonationMetadata {
    raw: RetainedObject,
}

impl IntentDonationMetadata {
    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "intent donation metadata") }?,
        })
    }

    pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
        Self { raw }
    }

    /// Returns the Objective-C class name for this `INIntentDonationMetadata` instance.
    pub fn class_name(&self) -> String {
        private::class_name(self)
    }
}

impl RawObject for IntentDonationMetadata {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// Wraps `INSendMessageIntentDonationMetadata`.
#[derive(Debug)]
pub struct SendMessageIntentDonationMetadata(IntentDonationMetadata);

impl SendMessageIntentDonationMetadata {
    /// Objective-C class name for `INSendMessageIntentDonationMetadata`.
    pub const OBJC_CLASS: &'static str = "INSendMessageIntentDonationMetadata";

    /// Creates a `INSendMessageIntentDonationMetadata` wrapper.
    pub fn new() -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_send_message_intent_donation_metadata_create(&mut error) };
        if ptr.is_null() {
            Err(unsafe {
                private::take_error(error, "creating send-message intent donation metadata")
            })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self(unsafe { IntentDonationMetadata::from_owned(ptr) }?))
    }

    /// Returns the Objective-C class name for this `INSendMessageIntentDonationMetadata` instance.
    pub fn class_name(&self) -> String {
        self.0.class_name()
    }

    /// Returns the corresponding value from `INSendMessageIntentDonationMetadata`.
    pub fn mentions_current_user(&self) -> bool {
        private::bool_property(self, "mentionsCurrentUser").unwrap_or_default()
    }

    /// Sets the corresponding `mentions_current_user` value on `INSendMessageIntentDonationMetadata`.
    pub fn set_mentions_current_user(&mut self, value: bool) -> Result<(), IntentsError> {
        private::set_integer_property(self, "mentionsCurrentUser", i64::from(value))
    }

    /// Returns the corresponding boolean value from `INSendMessageIntentDonationMetadata`.
    pub fn is_reply_to_current_user(&self) -> bool {
        private::bool_property(self, "replyToCurrentUser").unwrap_or_default()
    }

    /// Sets the corresponding `reply_to_current_user` value on `INSendMessageIntentDonationMetadata`.
    pub fn set_reply_to_current_user(&mut self, value: bool) -> Result<(), IntentsError> {
        private::set_integer_property(self, "replyToCurrentUser", i64::from(value))
    }

    /// Returns the corresponding value from `INSendMessageIntentDonationMetadata`.
    pub fn notify_recipient_anyway(&self) -> bool {
        private::bool_property(self, "notifyRecipientAnyway").unwrap_or_default()
    }

    /// Sets the corresponding `notify_recipient_anyway` value on `INSendMessageIntentDonationMetadata`.
    pub fn set_notify_recipient_anyway(&mut self, value: bool) -> Result<(), IntentsError> {
        private::set_integer_property(self, "notifyRecipientAnyway", i64::from(value))
    }

    /// Returns the number of corresponding values exposed by `INSendMessageIntentDonationMetadata`.
    pub fn recipient_count(&self) -> usize {
        private::integer_property(self, "recipientCount")
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or_default()
    }

    /// Sets the corresponding `recipient_count` value on `INSendMessageIntentDonationMetadata`.
    pub fn set_recipient_count(&mut self, value: usize) -> Result<(), IntentsError> {
        private::set_integer_property(
            self,
            "recipientCount",
            i64::try_from(value).map_err(|_| {
                IntentsError::invalid_argument("recipient count does not fit in i64")
            })?,
        )
    }
}

impl TryFrom<IntentDonationMetadata> for SendMessageIntentDonationMetadata {
    type Error = IntentsError;

    fn try_from(metadata: IntentDonationMetadata) -> Result<Self, Self::Error> {
        let actual = metadata.class_name();
        if actual == Self::OBJC_CLASS {
            Ok(Self(metadata))
        } else {
            Err(IntentsError::UnexpectedClass {
                expected: Self::OBJC_CLASS,
                actual,
            })
        }
    }
}

impl From<SendMessageIntentDonationMetadata> for IntentDonationMetadata {
    fn from(metadata: SendMessageIntentDonationMetadata) -> Self {
        metadata.0
    }
}

impl Deref for SendMessageIntentDonationMetadata {
    type Target = IntentDonationMetadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RawObject for SendMessageIntentDonationMetadata {
    fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}
