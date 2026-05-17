use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_call_group_create(
        group_name: *const c_char,
        group_id: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_call_record_create(
        identifier: *const c_char,
        call_record_type: i64,
        call_capability: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_call_record_filter_create(
        participants: *const *mut c_void,
        count: usize,
        call_types: u64,
        call_capability: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
