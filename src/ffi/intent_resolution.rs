use core::ffi::c_void;

extern "C" {
    pub fn inx_intent_resolution_result_needs_value() -> *mut c_void;
    pub fn inx_intent_resolution_result_not_required() -> *mut c_void;
    pub fn inx_intent_resolution_result_unsupported() -> *mut c_void;
}
