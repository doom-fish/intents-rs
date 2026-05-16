use core::ffi::c_void;

use super::INXStatusCallback;

extern "C" {
    pub fn inx_preferences_siri_authorization_status() -> i64;
    pub fn inx_preferences_request_siri_authorization(
        callback: INXStatusCallback,
        refcon: *mut c_void,
    );
}
