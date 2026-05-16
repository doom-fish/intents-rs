use core::ffi::c_void;

extern "C" {
    pub fn inx_intent_response_create() -> *mut c_void;
}
