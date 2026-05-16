use core::ffi::{c_char, c_void};

use super::{INXArrayCallback, INXObjectCallback};

extern "C" {
    pub fn inx_voice_shortcut_center_shared() -> *mut c_void;
    pub fn inx_voice_shortcut_center_get_all(
        center: *mut c_void,
        callback: INXArrayCallback,
        refcon: *mut c_void,
    );
    pub fn inx_voice_shortcut_center_get_by_identifier(
        center: *mut c_void,
        identifier: *const c_char,
        callback: INXObjectCallback,
        refcon: *mut c_void,
    );
}
