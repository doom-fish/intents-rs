use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_focus_status_create(
        has_value: bool,
        is_focused: bool,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_focus_status_center_copy_default() -> *mut c_void;
    pub fn inx_focus_status_center_request_authorization(
        center: *mut c_void,
        out_status: *mut i64,
        out_error: *mut *mut c_char,
    ) -> bool;
}
