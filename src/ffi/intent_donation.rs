use core::ffi::c_char;
use core::ffi::c_void;

extern "C" {
    pub fn inx_send_message_intent_donation_metadata_create(
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
