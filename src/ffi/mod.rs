#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type INXStatusCallback =
    unsafe extern "C" fn(refcon: *mut c_void, status: i64, error: *const c_char);
pub type INXObjectCallback =
    unsafe extern "C" fn(refcon: *mut c_void, object: *mut c_void, error: *const c_char);
pub type INXArrayCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    objects: *mut *mut c_void,
    count: usize,
    error: *const c_char,
);
pub type INXErrorCallback = unsafe extern "C" fn(refcon: *mut c_void, error: *const c_char);

extern "C" {
    pub fn inx_string_free(string: *mut c_char);
    pub fn inx_object_release(ptr: *mut c_void);

    pub fn inx_preferences_siri_authorization_status() -> i64;
    pub fn inx_preferences_request_siri_authorization(
        callback: INXStatusCallback,
        refcon: *mut c_void,
    );

    pub fn inx_object_class_name(ptr: *mut c_void) -> *mut c_char;
    pub fn inx_object_copy_string_property(ptr: *mut c_void, key: *const c_char) -> *mut c_char;
    pub fn inx_object_copy_object_property(ptr: *mut c_void, key: *const c_char) -> *mut c_void;
    pub fn inx_object_copy_string_array_property_json(
        ptr: *mut c_void,
        key: *const c_char,
    ) -> *mut c_char;
    pub fn inx_object_get_integer_property(
        ptr: *mut c_void,
        key: *const c_char,
        out_present: *mut bool,
    ) -> i64;
    pub fn inx_object_get_double_property(
        ptr: *mut c_void,
        key: *const c_char,
        out_present: *mut bool,
    ) -> f64;
    pub fn inx_object_get_bool_property(
        ptr: *mut c_void,
        key: *const c_char,
        out_present: *mut bool,
    ) -> bool;
    pub fn inx_object_get_array_count_property(
        ptr: *mut c_void,
        key: *const c_char,
        out_present: *mut bool,
    ) -> usize;
    pub fn inx_object_get_date_interval_property(
        ptr: *mut c_void,
        key: *const c_char,
        out_start: *mut f64,
        out_end: *mut f64,
        out_present: *mut bool,
    ) -> bool;

    pub fn inx_object_set_string_property(
        ptr: *mut c_void,
        key: *const c_char,
        value: *const c_char,
    ) -> bool;
    pub fn inx_object_set_integer_property(
        ptr: *mut c_void,
        key: *const c_char,
        value: i64,
    ) -> bool;
    pub fn inx_object_set_date_interval_property(
        ptr: *mut c_void,
        key: *const c_char,
        start: f64,
        end: f64,
    ) -> bool;
    pub fn inx_object_set_object_array_property(
        ptr: *mut c_void,
        key: *const c_char,
        values: *const *mut c_void,
        count: usize,
    ) -> bool;

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

    pub fn inx_shortcut_create_with_intent(
        intent: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;

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
