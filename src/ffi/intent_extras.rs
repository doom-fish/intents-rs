use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_answer_call_intent_create(
        audio_route: i64,
        call_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_edit_message_intent_create(
        message_identifier: *const c_char,
        edited_content: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_get_reservation_details_intent_create(
        reservation_container_reference: *mut c_void,
        reservation_item_references: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_hang_up_call_intent_create(
        call_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_share_focus_status_intent_create(
        focus_status: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_unsend_messages_intent_create(
        message_identifiers: *const *const c_char,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
