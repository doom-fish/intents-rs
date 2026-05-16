use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_in_object_create(
        identifier: *const c_char,
        display_string: *const c_char,
        pronunciation_hint: *const c_char,
        subtitle_string: *const c_char,
        display_image: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
