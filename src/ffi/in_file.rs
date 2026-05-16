use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_file_create_with_data(
        bytes: *const u8,
        count: usize,
        filename: *const c_char,
        type_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_file_create_with_file_url(
        file_url: *const c_char,
        filename: *const c_char,
        type_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_file_copy_data_json(file: *mut c_void) -> *mut c_char;
}
