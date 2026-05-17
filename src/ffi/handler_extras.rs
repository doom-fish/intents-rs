use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_handling_helper_create(
        kind: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_intent_handling_helper_simulate_handle(
        helper: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn inx_intent_handling_helper_simulate_confirm(
        helper: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn inx_intent_handling_helper_simulate_resolve(
        helper: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
}
