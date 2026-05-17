use core::ffi::c_void;
use std::ops::Deref;

use crate::error::IntentsError;
use crate::ffi;
use crate::intent_resolution::IntentResolutionResult;
use crate::private::{self, RawObject};

macro_rules! simple_enum {
    ($name:ident { $($variant:ident = $raw:expr,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)*
            Other(i64),
        }

        impl $name {
            #[allow(dead_code)]
            const fn from_raw(raw: i64) -> Self {
                match raw {
                    $($raw => Self::$variant,)*
                    other => Self::Other(other),
                }
            }

            #[allow(dead_code)]
            const fn raw_value(self) -> i64 {
                match self {
                    $(Self::$variant => $raw,)*
                    Self::Other(raw) => raw,
                }
            }
        }
    };
}

macro_rules! typed_resolution_result {
    ($name:ident, $objc_class:literal) => {
        #[derive(Debug)]
        pub struct $name(IntentResolutionResult);

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

            #[allow(dead_code)]
            pub(crate) fn new_blank() -> Result<Self, IntentsError> {
                let raw = private::create_blank_object(Self::OBJC_CLASS, "typed intent resolution result")?;
                Self::try_from(IntentResolutionResult::from_retained(raw))
            }

            pub fn class_name(&self) -> String {
                self.0.class_name()
            }
        }

        impl TryFrom<IntentResolutionResult> for $name {
            type Error = IntentsError;

            fn try_from(result: IntentResolutionResult) -> Result<Self, Self::Error> {
                let actual = result.class_name();
                if actual == Self::OBJC_CLASS {
                    Ok(Self(result))
                } else {
                    Err(IntentsError::UnexpectedClass {
                        expected: Self::OBJC_CLASS,
                        actual,
                    })
                }
            }
        }

        impl From<$name> for IntentResolutionResult {
            fn from(result: $name) -> Self {
                result.0
            }
        }

        impl Deref for $name {
            type Target = IntentResolutionResult;

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

simple_enum!(SendMessageRecipientUnsupportedReason {
    NoAccount = 1,
    Offline = 2,
    MessagingServiceNotEnabledForRecipient = 3,
    NoValidHandle = 4,
    RequestedHandleInvalid = 5,
    NoHandleForLabel = 6,
    RequiringInAppAuthentication = 7,
});

simple_enum!(StartCallCallRecordToCallBackUnsupportedReason {
    NoMatchingCall = 1,
});

typed_resolution_result!(BooleanResolutionResult, "INBooleanResolutionResult");
typed_resolution_result!(CallCapabilityResolutionResult, "INCallCapabilityResolutionResult");
typed_resolution_result!(CallDestinationTypeResolutionResult, "INCallDestinationTypeResolutionResult");
typed_resolution_result!(CallRecordResolutionResult, "INCallRecordResolutionResult");
typed_resolution_result!(CallRecordTypeOptionsResolutionResult, "INCallRecordTypeOptionsResolutionResult");
typed_resolution_result!(CallRecordTypeResolutionResult, "INCallRecordTypeResolutionResult");
typed_resolution_result!(CurrencyAmountResolutionResult, "INCurrencyAmountResolutionResult");
typed_resolution_result!(DateComponentsResolutionResult, "INDateComponentsResolutionResult");
typed_resolution_result!(DoubleResolutionResult, "INDoubleResolutionResult");
typed_resolution_result!(EnergyResolutionResult, "INEnergyResolutionResult");
typed_resolution_result!(EnumResolutionResult, "INEnumResolutionResult");
typed_resolution_result!(FileResolutionResult, "INFileResolutionResult");
typed_resolution_result!(IntegerResolutionResult, "INIntegerResolutionResult");
typed_resolution_result!(LengthResolutionResult, "INLengthResolutionResult");
typed_resolution_result!(MassResolutionResult, "INMassResolutionResult");
typed_resolution_result!(ObjectResolutionResult, "INObjectResolutionResult");
typed_resolution_result!(OutgoingMessageTypeResolutionResult, "INOutgoingMessageTypeResolutionResult");
typed_resolution_result!(PaymentMethodResolutionResult, "INPaymentMethodResolutionResult");
typed_resolution_result!(PersonResolutionResult, "INPersonResolutionResult");
typed_resolution_result!(PlacemarkResolutionResult, "INPlacemarkResolutionResult");
typed_resolution_result!(SendMessageRecipientResolutionResult, "INSendMessageRecipientResolutionResult");
typed_resolution_result!(SpeedResolutionResult, "INSpeedResolutionResult");
typed_resolution_result!(StartCallCallRecordToCallBackResolutionResult, "INStartCallCallRecordToCallBackResolutionResult");
typed_resolution_result!(StringResolutionResult, "INStringResolutionResult");
typed_resolution_result!(TemperatureResolutionResult, "INTemperatureResolutionResult");
typed_resolution_result!(TimeIntervalResolutionResult, "INTimeIntervalResolutionResult");
typed_resolution_result!(URLResolutionResult, "INURLResolutionResult");
typed_resolution_result!(VolumeResolutionResult, "INVolumeResolutionResult");

impl IntentResolutionResult {
    pub fn unsupported_with_reason(reason: i64) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_intent_resolution_result_unsupported_with_reason(reason, &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating unsupported custom intent resolution result") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn confirmation_required_with_item_for_reason(
        item: Option<&impl RawObject>,
        reason: i64,
    ) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_intent_resolution_result_confirmation_required_with_item_for_reason(
                item.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                reason,
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating confirmation-required custom intent resolution result") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }
}

impl SendMessageRecipientResolutionResult {
    pub fn unsupported_for_reason(reason: SendMessageRecipientUnsupportedReason) -> Result<Self, IntentsError> {
        let class_name = private::cstring(Self::OBJC_CLASS, "send-message recipient resolution result class name")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_typed_intent_resolution_result_unsupported_for_reason(
                class_name.as_ptr(),
                reason.raw_value(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating send-message recipient unsupported resolution result") })
        } else {
            Self::try_from(unsafe { IntentResolutionResult::from_owned(ptr) }?)
        }
    }
}

impl StartCallCallRecordToCallBackResolutionResult {
    pub fn unsupported_for_reason(reason: StartCallCallRecordToCallBackUnsupportedReason) -> Result<Self, IntentsError> {
        let class_name = private::cstring(Self::OBJC_CLASS, "start-call callback resolution result class name")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_typed_intent_resolution_result_unsupported_for_reason(
                class_name.as_ptr(),
                reason.raw_value(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating start-call callback unsupported resolution result") })
        } else {
            Self::try_from(unsafe { IntentResolutionResult::from_owned(ptr) }?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::in_object::IntentObject;

    #[test]
    fn resolution_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let string_result = StringResolutionResult::new_blank()?;
        let recipient = SendMessageRecipientResolutionResult::unsupported_for_reason(
            SendMessageRecipientUnsupportedReason::NoAccount,
        )?;
        let callback = StartCallCallRecordToCallBackResolutionResult::unsupported_for_reason(
            StartCallCallRecordToCallBackUnsupportedReason::NoMatchingCall,
        )?;
        let object = IntentObject::new(Some("id"), "Object")?;
        let custom = IntentResolutionResult::confirmation_required_with_item_for_reason(Some(&object), 42)?;

        assert_eq!(string_result.class_name(), StringResolutionResult::OBJC_CLASS);
        assert_eq!(recipient.class_name(), SendMessageRecipientResolutionResult::OBJC_CLASS);
        assert_eq!(callback.class_name(), StartCallCallRecordToCallBackResolutionResult::OBJC_CLASS);
        assert_eq!(custom.class_name(), "INIntentResolutionResult");
        Ok(())
    }
}
