use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_relevant_shortcut_create(
        shortcut: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_date_relevance_provider_create(
        start: f64,
        end: f64,
        has_end: bool,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_daily_routine_relevance_provider_create(
        situation: i64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
