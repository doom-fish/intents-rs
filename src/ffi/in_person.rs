use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_person_handle_create(
        value: *const c_char,
        handle_type: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_person_create(
        person_handle: *mut c_void,
        display_name: *const c_char,
        image: *mut c_void,
        contact_identifier: *const c_char,
        custom_identifier: *const c_char,
        aliases: *const *mut c_void,
        alias_count: usize,
        suggestion_type: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
