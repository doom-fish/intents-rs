use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_deferred_localized_intents_string_copy(
        format: *const c_char,
        table: *const c_char,
    ) -> *mut c_char;
    pub fn inx_placemark_create(
        name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_object_section_create(
        title: *const c_char,
        items: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_object_collection_create_with_items(
        items: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_object_collection_create_with_sections(
        sections: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
