use core::ffi::c_void;

use super::INXErrorCallback;

extern "C" {
    pub fn inx_relevant_shortcut_store_default() -> *mut c_void;
    pub fn inx_relevant_shortcut_store_set(
        store: *mut c_void,
        shortcuts: *const *mut c_void,
        count: usize,
        callback: INXErrorCallback,
        refcon: *mut c_void,
    );
}
