use core::ffi::c_void;
use std::ops::Deref;

use crate::call::CallAudioRoute;
use crate::error::IntentsError;
use crate::ffi;
use crate::focus::FocusStatus;
use crate::in_object::SpeakableString;
use crate::intent_definition::{Intent, SendMessageIntent, StartCallIntent};
use crate::private::{self, RawObject};

macro_rules! typed_intent_extra {
    ($name:ident, $objc_class:literal) => {
        #[derive(Debug)]
        pub struct $name(Intent);

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

            #[allow(dead_code)]
            pub(crate) fn new_blank() -> Result<Self, IntentsError> {
                let raw = private::create_blank_object(Self::OBJC_CLASS, "typed intent")?;
                Self::try_from(Intent::from_retained(raw))
            }

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

typed_intent_extra!(AnswerCallIntent, "INAnswerCallIntent");
typed_intent_extra!(EditMessageIntent, "INEditMessageIntent");
typed_intent_extra!(GetReservationDetailsIntent, "INGetReservationDetailsIntent");
typed_intent_extra!(HangUpCallIntent, "INHangUpCallIntent");
typed_intent_extra!(ShareFocusStatusIntent, "INShareFocusStatusIntent");
typed_intent_extra!(UnsendMessagesIntent, "INUnsendMessagesIntent");

impl AnswerCallIntent {
    pub fn new(audio_route: CallAudioRoute, call_identifier: Option<&str>) -> Result<Self, IntentsError> {
        let call_identifier = call_identifier
            .map(|value| private::cstring(value, "answer-call call identifier"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_answer_call_intent_create(
                audio_route.raw_value(),
                call_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating answer-call intent") })
        } else {
            Self::try_from(unsafe { Intent::from_owned(ptr) }?)
        }
    }

    pub fn audio_route(&self) -> CallAudioRoute {
        private::integer_property(self, "audioRoute").map_or(CallAudioRoute::Unknown, CallAudioRoute::from_raw)
    }

    pub fn call_identifier(&self) -> Option<String> {
        private::string_property(self, "callIdentifier")
    }
}

impl EditMessageIntent {
    pub fn new(message_identifier: Option<&str>, edited_content: Option<&str>) -> Result<Self, IntentsError> {
        let message_identifier = message_identifier
            .map(|value| private::cstring(value, "edit-message identifier"))
            .transpose()?;
        let edited_content = edited_content
            .map(|value| private::cstring(value, "edit-message content"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_edit_message_intent_create(
                message_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                edited_content
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating edit-message intent") })
        } else {
            Self::try_from(unsafe { Intent::from_owned(ptr) }?)
        }
    }

    pub fn message_identifier(&self) -> Option<String> {
        private::string_property(self, "messageIdentifier")
    }

    pub fn edited_content(&self) -> Option<String> {
        private::string_property(self, "editedContent")
    }
}

impl GetReservationDetailsIntent {
    pub fn new(
        reservation_container_reference: Option<&SpeakableString>,
        reservation_item_references: &[&SpeakableString],
    ) -> Result<Self, IntentsError> {
        let item_refs = reservation_item_references
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_get_reservation_details_intent_create(
                reservation_container_reference.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                if item_refs.is_empty() {
                    std::ptr::null()
                } else {
                    item_refs.as_ptr()
                },
                item_refs.len(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating get-reservation-details intent") })
        } else {
            Self::try_from(unsafe { Intent::from_owned(ptr) }?)
        }
    }

    pub fn reservation_container_reference_present(&self) -> bool {
        private::object_property(self, "reservationContainerReference").is_some()
    }

    pub fn reservation_item_references_count(&self) -> usize {
        private::array_count_property(self, "reservationItemReferences").unwrap_or_default()
    }
}

impl HangUpCallIntent {
    pub fn new(call_identifier: Option<&str>) -> Result<Self, IntentsError> {
        let call_identifier = call_identifier
            .map(|value| private::cstring(value, "hang-up call identifier"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_hang_up_call_intent_create(
                call_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating hang-up-call intent") })
        } else {
            Self::try_from(unsafe { Intent::from_owned(ptr) }?)
        }
    }

    pub fn call_identifier(&self) -> Option<String> {
        private::string_property(self, "callIdentifier")
    }
}

impl ShareFocusStatusIntent {
    pub fn new(focus_status: Option<&FocusStatus>) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_share_focus_status_intent_create(
                focus_status.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating share-focus-status intent") })
        } else {
            Self::try_from(unsafe { Intent::from_owned(ptr) }?)
        }
    }

    pub fn focus_status(&self) -> Option<FocusStatus> {
        private::object_property(self, "focusStatus").map(FocusStatus::from_retained)
    }
}

impl UnsendMessagesIntent {
    pub fn new(message_identifiers: &[&str]) -> Result<Self, IntentsError> {
        let values = message_identifiers
            .iter()
            .map(|value| private::cstring(value, "unsend-message identifier"))
            .collect::<Result<Vec<_>, _>>()?;
        let ptrs = values.iter().map(|value| value.as_ptr()).collect::<Vec<_>>();
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_unsend_messages_intent_create(
                if ptrs.is_empty() { std::ptr::null() } else { ptrs.as_ptr() },
                ptrs.len(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating unsend-messages intent") })
        } else {
            Self::try_from(unsafe { Intent::from_owned(ptr) }?)
        }
    }

    pub fn message_identifiers(&self) -> Result<Option<Vec<String>>, IntentsError> {
        private::string_array_property(self, "messageIdentifiers")
    }
}

impl SendMessageIntent {
    pub fn supports_user_notifications() -> Result<bool, IntentsError> {
        private::class_conforms_to_protocol(Self::OBJC_CLASS, "UNNotificationContentProviding")
    }
}

impl StartCallIntent {
    pub fn supports_user_notifications() -> Result<bool, IntentsError> {
        private::class_conforms_to_protocol(Self::OBJC_CLASS, "UNNotificationContentProviding")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::focus::FocusStatus;
    use crate::in_object::SpeakableString;

    #[test]
    fn intent_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let answer = AnswerCallIntent::new(CallAudioRoute::SpeakerphoneAudioRoute, Some("call-1"))?;
        let edit = EditMessageIntent::new(Some("message-1"), Some("updated"))?;
        let focus = FocusStatus::new(Some(true))?;
        let share = ShareFocusStatusIntent::new(Some(&focus))?;
        let reference = SpeakableString::new("ref", "Reference", None)?;
        let reservation = GetReservationDetailsIntent::new(Some(&reference), &[&reference])?;
        let unsend = UnsendMessagesIntent::new(&["message-1", "message-2"])?;
        let hang = HangUpCallIntent::new(Some("call-2"))?;

        assert_eq!(answer.call_identifier().as_deref(), Some("call-1"));
        assert_eq!(edit.edited_content().as_deref(), Some("updated"));
        assert!(share.focus_status().is_some());
        assert_eq!(reservation.reservation_item_references_count(), 1);
        assert_eq!(unsend.message_identifiers()?.unwrap_or_default().len(), 2);
        assert_eq!(hang.call_identifier().as_deref(), Some("call-2"));
        Ok(())
    }
}
