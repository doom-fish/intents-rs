use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::private::{self, RawObject, RetainedObject};

macro_rules! intent_handling_helper {
    ($name:ident, $kind:literal) => {
        #[derive(Debug)]
        pub struct $name {
            raw: RetainedObject,
        }

        impl $name {
            pub fn new() -> Result<Self, IntentsError> {
                let kind = private::cstring($kind, "intent handling helper kind")?;
                let mut error = std::ptr::null_mut();
                let ptr =
                    unsafe { ffi::inx_intent_handling_helper_create(kind.as_ptr(), &mut error) };
                if ptr.is_null() {
                    Err(unsafe {
                        private::take_error(
                            error,
                            concat!("creating ", $kind, " intent handling helper"),
                        )
                    })
                } else {
                    unsafe { Self::from_owned(ptr) }
                }
            }

            pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
                Ok(Self {
                    raw: unsafe {
                        RetainedObject::from_owned(ptr, concat!($kind, " intent handling helper"))
                    }?,
                })
            }

            pub fn class_name(&self) -> String {
                private::class_name(self)
            }

            pub fn simulate_handle(&mut self) -> Result<(), IntentsError> {
                let mut error = std::ptr::null_mut();
                let ok = unsafe {
                    ffi::inx_intent_handling_helper_simulate_handle(self.as_ptr(), &mut error)
                };
                if ok {
                    Ok(())
                } else {
                    Err(unsafe {
                        private::take_error(error, concat!("simulating ", $kind, " handle"))
                    })
                }
            }

            pub fn simulate_confirm(&mut self) -> Result<(), IntentsError> {
                let mut error = std::ptr::null_mut();
                let ok = unsafe {
                    ffi::inx_intent_handling_helper_simulate_confirm(self.as_ptr(), &mut error)
                };
                if ok {
                    Ok(())
                } else {
                    Err(unsafe {
                        private::take_error(error, concat!("simulating ", $kind, " confirm"))
                    })
                }
            }

            pub fn simulate_resolve(&mut self) -> Result<(), IntentsError> {
                let mut error = std::ptr::null_mut();
                let ok = unsafe {
                    ffi::inx_intent_handling_helper_simulate_resolve(self.as_ptr(), &mut error)
                };
                if ok {
                    Ok(())
                } else {
                    Err(unsafe {
                        private::take_error(error, concat!("simulating ", $kind, " resolve"))
                    })
                }
            }

            pub fn handle_call_count(&self) -> usize {
                private::integer_property(self, "handleCallCount")
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or_default()
            }

            pub fn confirm_call_count(&self) -> usize {
                private::integer_property(self, "confirmCallCount")
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or_default()
            }

            pub fn resolve_call_count(&self) -> usize {
                private::integer_property(self, "resolveCallCount")
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or_default()
            }

            pub fn last_intent_class_name(&self) -> Option<String> {
                private::string_property(self, "lastIntentClassName")
            }
        }

        impl RawObject for $name {
            fn as_ptr(&self) -> *mut c_void {
                self.raw.as_ptr()
            }
        }
    };
}

intent_handling_helper!(AnswerCallIntentHandling, "answer_call");
intent_handling_helper!(EditMessageIntentHandling, "edit_message");
intent_handling_helper!(HangUpCallIntentHandling, "hang_up_call");
intent_handling_helper!(SendMessageIntentHandling, "send_message");
intent_handling_helper!(ShareFocusStatusIntentHandling, "share_focus_status");
intent_handling_helper!(UnsendMessagesIntentHandling, "unsend_messages");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_extras_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let mut answer = AnswerCallIntentHandling::new()?;
        let mut edit = EditMessageIntentHandling::new()?;
        let mut send = SendMessageIntentHandling::new()?;

        answer.simulate_handle()?;
        answer.simulate_confirm()?;
        edit.simulate_handle()?;
        edit.simulate_confirm()?;
        edit.simulate_resolve()?;
        send.simulate_handle()?;
        send.simulate_confirm()?;
        send.simulate_resolve()?;

        assert_eq!(answer.handle_call_count(), 1);
        assert_eq!(answer.confirm_call_count(), 1);
        assert_eq!(edit.resolve_call_count(), 1);
        assert_eq!(send.handle_call_count(), 1);
        Ok(())
    }
}
