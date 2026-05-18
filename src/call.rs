use core::ffi::c_void;
use std::ops::{BitOr, BitOrAssign};

use crate::error::IntentsError;
use crate::ffi;
use crate::in_person::Person;
use crate::private::{self, RawObject, RetainedObject};

macro_rules! simple_enum {
    ($name:ident { $($variant:ident = $raw:expr,)* }) => {
        #[doc = concat!("Mirrors `IN", stringify!($name), "`.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(#[doc = concat!("Corresponds to the `", stringify!($variant), "` case of `IN", stringify!($name), "`.")]
            $variant,)*
            #[doc = concat!("Stores an unknown raw value from `IN", stringify!($name), "`.")]
            Other(i64),
        }

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn from_raw(raw: i64) -> Self {
                match raw {
                    $($raw => Self::$variant,)*
                    other => Self::Other(other),
                }
            }

            #[allow(dead_code)]
            pub(crate) const fn raw_value(self) -> i64 {
                match self {
                    $(Self::$variant => $raw,)*
                    Self::Other(raw) => raw,
                }
            }
        }
    };
}

macro_rules! objc_wrapper {
    ($name:ident, $objc_class:literal, $context:literal) => {
        #[doc = concat!("Wraps `", $objc_class, "`.")]
        #[derive(Debug)]
        pub struct $name {
            raw: RetainedObject,
        }

        impl $name {
            #[doc = concat!("Objective-C class name for `", $objc_class, "`.")]
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

            #[doc = concat!("Returns the Objective-C class name for this `", $objc_class, "` instance.")]
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

simple_enum!(CallAudioRoute {
    Unknown = 0,
    SpeakerphoneAudioRoute = 1,
    BluetoothAudioRoute = 2,
});

simple_enum!(CallCapability {
    Unknown = 0,
    AudioCall = 1,
    VideoCall = 2,
});

simple_enum!(CallDestinationType {
    Unknown = 0,
    Normal = 1,
    Emergency = 2,
    Voicemail = 3,
    Redial = 4,
    CallBack = 5,
});

simple_enum!(CallRecordType {
    Unknown = 0,
    Outgoing = 1,
    Missed = 2,
    Received = 3,
    Latest = 4,
    Voicemail = 5,
    Ringing = 6,
    InProgress = 7,
    OnHold = 8,
});

/// Wraps `INCallCapabilityOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CallCapabilityOptions(u64);

impl CallCapabilityOptions {
    /// Bit flag corresponding to `AUDIO_CALL` on `INCallCapabilityOptions`.
    pub const AUDIO_CALL: Self = Self(1 << 0);
    /// Bit flag corresponding to `VIDEO_CALL` on `INCallCapabilityOptions`.
    pub const VIDEO_CALL: Self = Self(1 << 1);

    /// Returns an empty set of `INCallCapabilityOptions` flags.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the raw bit pattern for this `INCallCapabilityOptions` flag set.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Builds an `INCallCapabilityOptions` flag set from raw bits.
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns whether this `INCallCapabilityOptions` flag set contains the provided bits.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for CallCapabilityOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CallCapabilityOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Wraps `INCallRecordTypeOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CallRecordTypeOptions(u64);

impl CallRecordTypeOptions {
    /// Bit flag corresponding to `OUTGOING` on `INCallRecordTypeOptions`.
    pub const OUTGOING: Self = Self(1 << 0);
    /// Bit flag corresponding to `MISSED` on `INCallRecordTypeOptions`.
    pub const MISSED: Self = Self(1 << 1);
    /// Bit flag corresponding to `RECEIVED` on `INCallRecordTypeOptions`.
    pub const RECEIVED: Self = Self(1 << 2);
    /// Bit flag corresponding to `LATEST` on `INCallRecordTypeOptions`.
    pub const LATEST: Self = Self(1 << 3);
    /// Bit flag corresponding to `VOICEMAIL` on `INCallRecordTypeOptions`.
    pub const VOICEMAIL: Self = Self(1 << 4);
    /// Bit flag corresponding to `RINGING` on `INCallRecordTypeOptions`.
    pub const RINGING: Self = Self(1 << 5);
    /// Bit flag corresponding to `IN_PROGRESS` on `INCallRecordTypeOptions`.
    pub const IN_PROGRESS: Self = Self(1 << 6);
    /// Bit flag corresponding to `ON_HOLD` on `INCallRecordTypeOptions`.
    pub const ON_HOLD: Self = Self(1 << 7);

    /// Returns an empty set of `INCallRecordTypeOptions` flags.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the raw bit pattern for this `INCallRecordTypeOptions` flag set.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Builds an `INCallRecordTypeOptions` flag set from raw bits.
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns whether this `INCallRecordTypeOptions` flag set contains the provided bits.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for CallRecordTypeOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for CallRecordTypeOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

objc_wrapper!(CallGroup, "INCallGroup", "call group");
objc_wrapper!(CallRecord, "INCallRecord", "call record");
objc_wrapper!(CallRecordFilter, "INCallRecordFilter", "call record filter");

impl CallGroup {
    /// Creates a `INCallGroup` wrapper.
    pub fn new(group_name: Option<&str>, group_id: Option<&str>) -> Result<Self, IntentsError> {
        let group_name = group_name
            .map(|value| private::cstring(value, "call group name"))
            .transpose()?;
        let group_id = group_id
            .map(|value| private::cstring(value, "call group id"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_call_group_create(
                group_name
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                group_id
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating call group") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the corresponding value from `INCallGroup`.
    pub fn group_name(&self) -> Option<String> {
        private::string_property(self, "groupName")
    }

    /// Returns the corresponding value from `INCallGroup`.
    pub fn group_id(&self) -> Option<String> {
        private::string_property(self, "groupId")
    }
}

impl CallRecord {
    /// Creates a `INCallRecord` wrapper.
    pub fn new(
        identifier: &str,
        call_record_type: CallRecordType,
        call_capability: CallCapability,
    ) -> Result<Self, IntentsError> {
        let identifier = private::cstring(identifier, "call record identifier")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_call_record_create(
                identifier.as_ptr(),
                call_record_type.raw_value(),
                call_capability.raw_value(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating call record") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the corresponding value from `INCallRecord`.
    pub fn identifier(&self) -> Option<String> {
        private::string_property(self, "identifier")
    }

    /// Returns the corresponding value from `INCallRecord`.
    pub fn call_record_type(&self) -> CallRecordType {
        private::integer_property(self, "callRecordType")
            .map_or(CallRecordType::Unknown, CallRecordType::from_raw)
    }

    /// Returns the corresponding value from `INCallRecord`.
    pub fn call_capability(&self) -> CallCapability {
        private::integer_property(self, "callCapability")
            .map_or(CallCapability::Unknown, CallCapability::from_raw)
    }

    /// Returns the number of corresponding values exposed by `INCallRecord`.
    pub fn participants_count(&self) -> usize {
        private::array_count_property(self, "participants").unwrap_or_default()
    }

    /// Returns the corresponding value from `INCallRecord`.
    pub fn call_duration(&self) -> Option<f64> {
        private::double_property(self, "callDuration")
    }

    /// Returns the corresponding value from `INCallRecord`.
    pub fn unseen(&self) -> Option<bool> {
        private::bool_property(self, "unseen")
    }

    /// Returns the corresponding value from `INCallRecord`.
    pub fn number_of_calls(&self) -> Option<i64> {
        private::integer_property(self, "numberOfCalls")
    }

    /// Returns the corresponding boolean value from `INCallRecord`.
    pub fn is_caller_id_blocked(&self) -> Option<bool> {
        private::bool_property(self, "isCallerIdBlocked")
    }
}

impl CallRecordFilter {
    /// Creates a `INCallRecordFilter` wrapper.
    pub fn new(
        participants: &[&Person],
        call_types: CallRecordTypeOptions,
        call_capability: CallCapability,
    ) -> Result<Self, IntentsError> {
        let participants = participants
            .iter()
            .map(|person| person.as_ptr())
            .collect::<Vec<_>>();
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_call_record_filter_create(
                if participants.is_empty() {
                    std::ptr::null()
                } else {
                    participants.as_ptr()
                },
                participants.len(),
                call_types.bits(),
                call_capability.raw_value(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating call record filter") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    /// Returns the number of corresponding values exposed by `INCallRecordFilter`.
    pub fn participants_count(&self) -> usize {
        private::array_count_property(self, "participants").unwrap_or_default()
    }

    /// Returns the corresponding value from `INCallRecordFilter`.
    pub fn call_types(&self) -> CallRecordTypeOptions {
        private::integer_property(self, "callTypes")
            .and_then(|value| u64::try_from(value).ok())
            .map_or_else(
                CallRecordTypeOptions::empty,
                CallRecordTypeOptions::from_bits_truncate,
            )
    }

    /// Returns the corresponding value from `INCallRecordFilter`.
    pub fn call_capability(&self) -> CallCapability {
        private::integer_property(self, "callCapability")
            .map_or(CallCapability::Unknown, CallCapability::from_raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::in_person::{Person, PersonHandle, PersonHandleType};

    #[test]
    fn call_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let group = CallGroup::new(Some("Friends"), Some("group-1"))?;
        let record = CallRecord::new(
            "call-1",
            CallRecordType::Outgoing,
            CallCapability::AudioCall,
        )?;
        let handle = PersonHandle::new(Some("friend@example.com"), PersonHandleType::EmailAddress)?;
        let person = Person::new(&handle, Some("Friend"))?;
        let filter = CallRecordFilter::new(
            &[&person],
            CallRecordTypeOptions::OUTGOING,
            CallCapability::AudioCall,
        )?;

        assert_eq!(group.group_name().as_deref(), Some("Friends"));
        assert_eq!(record.call_record_type(), CallRecordType::Outgoing);
        assert_eq!(filter.participants_count(), 1);
        assert!(filter
            .call_types()
            .contains(CallRecordTypeOptions::OUTGOING));
        Ok(())
    }
}
