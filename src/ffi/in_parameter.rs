use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_parameter_create(
        intent_class_name: *const c_char,
        key_path: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_parameter_set_index(
        parameter: *mut c_void,
        index: isize,
        sub_key_path: *const c_char,
    ) -> bool;
    pub fn inx_parameter_get_index(
        parameter: *mut c_void,
        sub_key_path: *const c_char,
        out_present: *mut bool,
    ) -> isize;
}
