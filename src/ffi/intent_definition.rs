use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_intent_create() -> *mut c_void;
    pub fn inx_user_activity_create(
        activity_type: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_speakable_string_create(
        vocabulary_identifier: *const c_char,
        spoken_phrase: *const c_char,
        pronunciation_hint: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_image_create_named(
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_image_create_with_data(
        bytes: *const u8,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_image_create_with_url(
        url: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_shortcut_create_with_intent(
        intent: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_shortcut_create_with_user_activity(
        user_activity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_intent_copy_key_image(intent: *mut c_void) -> *mut c_void;
    pub fn inx_intent_copy_image_for_parameter_named(
        intent: *mut c_void,
        parameter_name: *const c_char,
    ) -> *mut c_void;
    pub fn inx_intent_set_image_for_parameter_named(
        intent: *mut c_void,
        parameter_name: *const c_char,
        image: *mut c_void,
    ) -> bool;
}
