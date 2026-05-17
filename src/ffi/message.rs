use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_message_link_metadata_create(
        site_name: *const c_char,
        summary: *const c_char,
        title: *const c_char,
        open_graph_type: *const c_char,
        link_url: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_message_reaction_create(
        reaction_type: i64,
        reaction_description: *const c_char,
        emoji: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_send_message_attachment_create_with_audio_file(
        file: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_sticker_create(
        sticker_type: i64,
        emoji: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
