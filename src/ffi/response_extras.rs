use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_response_subclass_create(
        class_name: *const c_char,
        code: i64,
        user_activity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
