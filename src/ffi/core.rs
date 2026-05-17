use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_string_free(string: *mut c_char);
    pub fn inx_object_release(ptr: *mut c_void);
    pub fn inx_object_class_name(ptr: *mut c_void) -> *mut c_char;
    pub fn inx_object_is_equal(lhs: *mut c_void, rhs: *mut c_void) -> bool;
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
    pub fn inx_object_create_blank(
        class_name: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_class_conforms_to_protocol(
        class_name: *const c_char,
        protocol_name: *const c_char,
    ) -> bool;
    pub fn inx_intents_version_number() -> f64;
    pub fn inx_intents_version_string() -> *mut c_char;
    pub fn inx_object_set_integer_property(
        ptr: *mut c_void,
        key: *const c_char,
        value: i64,
    ) -> bool;
    pub fn inx_object_set_bool_property(
        ptr: *mut c_void,
        key: *const c_char,
        value: bool,
    ) -> bool;
    pub fn inx_object_set_object_property(
        ptr: *mut c_void,
        key: *const c_char,
        value: *mut c_void,
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
}
