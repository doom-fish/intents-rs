use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_resolution_result_unsupported_with_reason(
        reason: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_intent_resolution_result_confirmation_required_with_item_for_reason(
        item: *mut c_void,
        reason: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_typed_intent_resolution_result_unsupported_for_reason(
        class_name: *const c_char,
        reason: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
