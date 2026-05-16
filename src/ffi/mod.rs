#![allow(missing_docs, non_camel_case_types)]

use std::ffi::{c_char, c_void};

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

mod core;
mod in_interaction;
mod in_object;
mod in_parameter;
mod in_relevant_shortcut;
mod in_relevant_shortcut_store;
mod in_vocabulary;
mod intent_definition;
mod intent_donation;
mod intent_extension;
mod intent_handler;
mod intent_response;
mod preferences;
mod voice_shortcut;

pub use core::*;
pub use in_interaction::*;
pub use in_object::*;
pub use in_parameter::*;
pub use in_relevant_shortcut::*;
pub use in_relevant_shortcut_store::*;
pub use in_vocabulary::*;
pub use intent_definition::*;
pub use intent_donation::*;
pub use intent_extension::*;
pub use intent_handler::*;
pub use intent_response::*;
pub use preferences::*;
pub use voice_shortcut::*;
