use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_handler_provider_create() -> *mut c_void;
    pub fn inx_intent_handler_provider_register(
        provider: *mut c_void,
        intent_class_name: *const c_char,
        handler_name: *const c_char,
    ) -> bool;
    pub fn inx_intent_handler_provider_copy_handler_name_for_intent(
        provider: *mut c_void,
        intent: *mut c_void,
    ) -> *mut c_char;
    pub fn inx_start_call_intent_handling_create(out_error: *mut *mut c_char) -> *mut c_void;
    pub fn inx_start_call_intent_handling_simulate_handle(
        helper: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn inx_start_call_intent_handling_simulate_confirm(
        helper: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> bool;
}
