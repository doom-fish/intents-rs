use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_extension_create() -> *mut c_void;
    pub fn inx_intent_extension_copy_handler_class_name_for_intent(
        extension: *mut c_void,
        intent: *mut c_void,
    ) -> *mut c_char;
}
