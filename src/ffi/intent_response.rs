use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_response_create() -> *mut c_void;
    pub fn inx_send_message_intent_response_create(
        code: i64,
        user_activity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
