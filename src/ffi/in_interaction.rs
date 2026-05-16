use core::ffi::{c_char, c_void};

use super::INXErrorCallback;

extern "C" {
    pub fn inx_interaction_create(
        intent: *mut c_void,
        response: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_interaction_donate(
        interaction: *mut c_void,
        callback: INXErrorCallback,
        refcon: *mut c_void,
    );
    pub fn inx_interaction_delete_all(callback: INXErrorCallback, refcon: *mut c_void);
    pub fn inx_interaction_delete_by_identifiers(
        identifiers: *const *const c_char,
        count: usize,
        callback: INXErrorCallback,
        refcon: *mut c_void,
    );
    pub fn inx_interaction_delete_by_group_identifier(
        group_identifier: *const c_char,
        callback: INXErrorCallback,
        refcon: *mut c_void,
    );
}
