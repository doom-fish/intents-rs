use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_vocabulary_shared() -> *mut c_void;
    pub fn inx_vocabulary_set_strings(
        vocabulary: *mut c_void,
        values: *const *const c_char,
        count: usize,
        type_: i64,
    ) -> bool;
    pub fn inx_vocabulary_set_speakables(
        vocabulary: *mut c_void,
        values: *const *mut c_void,
        count: usize,
        type_: i64,
    ) -> bool;
    pub fn inx_vocabulary_remove_all(vocabulary: *mut c_void) -> bool;
}
