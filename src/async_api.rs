//! Async API for the `Intents` framework.
//!
//! This module provides `Future`-based wrappers for the `Intents` framework
//! completion-handler APIs. It is gated behind the `async` Cargo feature and
//! requires no specific async runtime — it is **executor-agnostic** and works
//! with any runtime (Tokio, async-std, smol, pollster, etc.).
//!
//! ## APIs wrapped
//!
//! | Swift API | Rust future type | Rust entry point |
//! |---|---|---|
//! | `INInteraction.donate(completion:)` | [`InteractionDonateFuture`] | [`AsyncInteraction::donate`] |
//! | `INInteraction.delete(with:[String],completion:)` | [`InteractionDeleteFuture`] | [`AsyncInteraction::delete`] |
//! | `INInteraction.delete(with:String,completion:)` | [`InteractionDeleteFuture`] | [`AsyncInteraction::delete_by_group`] |
//! | `INInteraction.deleteAll(completion:)` | [`InteractionDeleteAllFuture`] | [`AsyncInteraction::delete_all`] |
//! | `INPreferences.requestSiriAuthorization(_:)` | [`SiriAuthorizationFuture`] | [`AsyncPreferences::request_siri_authorization`] |
//! | `INVoiceShortcutCenter.getAllVoiceShortcuts(completion:)` | [`AllVoiceShortcutsFuture`] | [`AsyncVoiceShortcutCenter::get_all`] |
//! | `INVoiceShortcutCenter.getVoiceShortcut(with:completion:)` | [`VoiceShortcutFuture`] | [`AsyncVoiceShortcutCenter::get`] |
//!
//! ## APIs deferred to Tier 2 (Stream pattern)
//!
//! * `INSpeechRecognitionRequest.start(handler:)` — not available on macOS
//!   (Speech framework, iOS/watchOS only); defer to Tier 2 if macOS support
//!   is ever added.
//! * `IntentHandler.handle(intent:completion:)` — protocol-based: the system
//!   calls *into* your handler rather than you awaiting a result. An
//!   async-trait bridge belongs in Tier 2.
//!
//! ## Example
//!
//! ```rust,no_run
//! use intents::async_api::{AsyncInteraction, AsyncPreferences};
//! use intents::{Intent, IntentResponse, Interaction};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! pollster::block_on(async {
//!     // Donate an interaction
//!     let intent = Intent::new()?;
//!     let interaction = Interaction::new(&intent, None)?;
//!     AsyncInteraction::donate(&interaction).await?;
//!
//!     // Delete all interactions
//!     AsyncInteraction::delete_all().await?;
//!
//!     Ok::<_, Box<dyn std::error::Error>>(())
//! })
//! # }
//! ```

#![allow(clippy::missing_errors_doc)]

use std::ffi::{c_char, c_void, CString};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};

use crate::error::IntentsError;
use crate::ffi;
use crate::in_interaction::Interaction;
use crate::preferences::SiriAuthorizationStatus;
use crate::private::RawObject;
use crate::voice_shortcut::{VoiceShortcut, VoiceShortcutCenter};

// ============================================================================
// Shared error callback for void-result async operations
// ============================================================================

/// Callback for operations that return `()` on success.
unsafe extern "C" fn void_completion_cb(ctx: *mut c_void, error: *const c_char) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error.cast::<i8>()) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

// ============================================================================
// INInteraction.donate(completion:)
// ============================================================================

/// Future returned by [`AsyncInteraction::donate`].
///
/// Resolves to `Ok(())` when the donation is acknowledged by Siri,
/// or `Err(IntentsError)` on failure.
pub struct InteractionDonateFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for InteractionDonateFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InteractionDonateFuture")
            .finish_non_exhaustive()
    }
}

impl Future for InteractionDonateFuture {
    type Output = Result<(), IntentsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(IntentsError::framework))
    }
}

// ============================================================================
// INInteraction.delete(with:completion:)
// ============================================================================

/// Future returned by [`AsyncInteraction::delete`] and
/// [`AsyncInteraction::delete_by_group`].
///
/// Resolves to `Ok(())` on success or `Err(IntentsError)` on failure.
pub struct InteractionDeleteFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for InteractionDeleteFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InteractionDeleteFuture")
            .finish_non_exhaustive()
    }
}

impl Future for InteractionDeleteFuture {
    type Output = Result<(), IntentsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(IntentsError::framework))
    }
}

// ============================================================================
// INInteraction.deleteAll(completion:)
// ============================================================================

/// Future returned by [`AsyncInteraction::delete_all`].
///
/// Resolves to `Ok(())` on success or `Err(IntentsError)` on failure.
pub struct InteractionDeleteAllFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for InteractionDeleteAllFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InteractionDeleteAllFuture")
            .finish_non_exhaustive()
    }
}

impl Future for InteractionDeleteAllFuture {
    type Output = Result<(), IntentsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(IntentsError::framework))
    }
}

// ============================================================================
// AsyncInteraction — async wrappers for INInteraction
// ============================================================================

/// Async wrappers for [`Interaction`] / `INInteraction` operations.
///
/// All methods are **executor-agnostic**: they return standard [`Future`]s and
/// work with any async runtime.
///
/// # macOS availability
///
/// All wrapped APIs are available on macOS 12.0+. Earlier macOS versions may
/// return errors via the returned `Err` variant.
#[derive(Debug, Clone, Copy)]
pub struct AsyncInteraction;

impl AsyncInteraction {
    /// Asynchronously donate an [`Interaction`] to Siri.
    ///
    /// Wraps `INInteraction.donate(completion:)`.
    pub fn donate(interaction: &Interaction) -> InteractionDonateFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe { ffi::inx_interaction_donate(interaction.as_ptr(), void_completion_cb, ctx) };
        InteractionDonateFuture { inner: future }
    }

    /// Asynchronously delete interactions by their identifiers.
    ///
    /// Wraps `INInteraction.delete(with:[String],completion:)`.
    ///
    /// # Errors
    ///
    /// Returns [`IntentsError::InvalidArgument`] if any identifier contains an
    /// interior NUL byte.
    pub fn delete(identifiers: &[&str]) -> Result<InteractionDeleteFuture, IntentsError> {
        let cstrings = identifiers
            .iter()
            .map(|id| {
                CString::new(*id).map_err(|e| {
                    IntentsError::invalid_argument(format!(
                        "identifier contains a NUL byte: {e}"
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let ptr = if ptrs.is_empty() {
            std::ptr::null()
        } else {
            ptrs.as_ptr()
        };
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::inx_interaction_delete_by_identifiers(ptr, ptrs.len(), void_completion_cb, ctx);
        }
        Ok(InteractionDeleteFuture { inner: future })
    }

    /// Asynchronously delete interactions belonging to a group identifier.
    ///
    /// Wraps `INInteraction.delete(with:String,completion:)`.
    ///
    /// # Errors
    ///
    /// Returns [`IntentsError::InvalidArgument`] if `group_id` contains an
    /// interior NUL byte.
    pub fn delete_by_group(group_id: &str) -> Result<InteractionDeleteFuture, IntentsError> {
        let cstring = CString::new(group_id).map_err(|e| {
            IntentsError::invalid_argument(format!("group_id contains a NUL byte: {e}"))
        })?;
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::inx_interaction_delete_by_group_identifier(
                cstring.as_ptr(),
                void_completion_cb,
                ctx,
            );
        }
        Ok(InteractionDeleteFuture { inner: future })
    }

    /// Asynchronously delete all donated interactions.
    ///
    /// Wraps `INInteraction.deleteAll(completion:)`.
    pub fn delete_all() -> InteractionDeleteAllFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe { ffi::inx_interaction_delete_all(void_completion_cb, ctx) };
        InteractionDeleteAllFuture { inner: future }
    }
}

// ============================================================================
// INPreferences.requestSiriAuthorization(_:)
// ============================================================================

/// Callback for Siri authorization, carrying a status code.
unsafe extern "C" fn siri_auth_cb(ctx: *mut c_void, status: i64, error: *const c_char) {
    if error.is_null() {
        let auth = SiriAuthorizationStatus::from_raw(status);
        unsafe { AsyncCompletion::complete_ok(ctx, auth) };
    } else {
        let msg = unsafe { error_from_cstr(error.cast::<i8>()) };
        unsafe { AsyncCompletion::<SiriAuthorizationStatus>::complete_err(ctx, msg) };
    }
}

/// Future returned by [`AsyncPreferences::request_siri_authorization`].
///
/// Resolves to the resulting [`SiriAuthorizationStatus`] on success, or
/// `Err(IntentsError)` if the system couldn't complete the request.
pub struct SiriAuthorizationFuture {
    inner: AsyncCompletionFuture<SiriAuthorizationStatus>,
}

impl std::fmt::Debug for SiriAuthorizationFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SiriAuthorizationFuture")
            .finish_non_exhaustive()
    }
}

impl Future for SiriAuthorizationFuture {
    type Output = Result<SiriAuthorizationStatus, IntentsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(IntentsError::framework))
    }
}

/// Async wrapper for `INPreferences` operations.
///
/// Wraps `INPreferences.requestSiriAuthorization(_:)` as a [`Future`].
#[derive(Debug, Clone, Copy)]
pub struct AsyncPreferences;

impl AsyncPreferences {
    /// Asynchronously request Siri & dictation authorization.
    ///
    /// Wraps `INPreferences.requestSiriAuthorization(_:)`. On macOS this
    /// will show the system authorization dialog the first time it is called.
    /// Subsequent calls resolve immediately with the cached status.
    ///
    /// # macOS availability
    ///
    /// `INPreferences` is available on macOS 12.0+. If the class is absent,
    /// the future resolves with `Err(IntentsError::Framework(...))`.
    pub fn request_siri_authorization() -> SiriAuthorizationFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe { ffi::inx_preferences_request_siri_authorization(siri_auth_cb, ctx) };
        SiriAuthorizationFuture { inner: future }
    }
}

// ============================================================================
// INVoiceShortcutCenter.getAllVoiceShortcuts(completion:)
// ============================================================================

/// Callback that decodes a C array of `INVoiceShortcut *` pointers.
unsafe extern "C" fn all_shortcuts_cb(
    ctx: *mut c_void,
    objects: *mut *mut c_void,
    count: usize,
    error: *const c_char,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error.cast::<i8>()) };
        unsafe { AsyncCompletion::<Vec<VoiceShortcut>>::complete_err(ctx, msg) };
        return;
    }
    let result: Result<Vec<VoiceShortcut>, String> = if count == 0 {
        Ok(Vec::new())
    } else {
        let slice = unsafe { std::slice::from_raw_parts(objects.cast_const(), count) };
        slice
            .iter()
            .copied()
            .map(|ptr| unsafe { VoiceShortcut::from_owned(ptr) }.map_err(|e| e.to_string()))
            .collect()
    };
    match result {
        Ok(shortcuts) => unsafe { AsyncCompletion::complete_ok(ctx, shortcuts) },
        Err(msg) => unsafe { AsyncCompletion::<Vec<VoiceShortcut>>::complete_err(ctx, msg) },
    }
}

/// Future returned by [`AsyncVoiceShortcutCenter::get_all`].
///
/// Resolves to a `Vec<VoiceShortcut>` (possibly empty) on success, or
/// `Err(IntentsError)` on failure.
pub struct AllVoiceShortcutsFuture {
    inner: AsyncCompletionFuture<Vec<VoiceShortcut>>,
}

impl std::fmt::Debug for AllVoiceShortcutsFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AllVoiceShortcutsFuture")
            .finish_non_exhaustive()
    }
}

impl Future for AllVoiceShortcutsFuture {
    type Output = Result<Vec<VoiceShortcut>, IntentsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(IntentsError::framework))
    }
}

// ============================================================================
// INVoiceShortcutCenter.getVoiceShortcut(with:completion:)
// ============================================================================

/// Callback for a single optional `INVoiceShortcut *`.
unsafe extern "C" fn voice_shortcut_cb(
    ctx: *mut c_void,
    object: *mut c_void,
    error: *const c_char,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error.cast::<i8>()) };
        unsafe { AsyncCompletion::<Option<VoiceShortcut>>::complete_err(ctx, msg) };
        return;
    }
    if object.is_null() {
        unsafe { AsyncCompletion::complete_ok(ctx, None::<VoiceShortcut>) };
        return;
    }
    match unsafe { VoiceShortcut::from_owned(object) } {
        Ok(shortcut) => unsafe { AsyncCompletion::complete_ok(ctx, Some(shortcut)) },
        Err(e) => unsafe {
            AsyncCompletion::<Option<VoiceShortcut>>::complete_err(ctx, e.to_string());
        },
    }
}

/// Future returned by [`AsyncVoiceShortcutCenter::get`].
///
/// Resolves to `Ok(Some(shortcut))` when found, `Ok(None)` when no matching
/// shortcut exists, or `Err(IntentsError)` on failure.
pub struct VoiceShortcutFuture {
    inner: AsyncCompletionFuture<Option<VoiceShortcut>>,
}

impl std::fmt::Debug for VoiceShortcutFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VoiceShortcutFuture")
            .finish_non_exhaustive()
    }
}

impl Future for VoiceShortcutFuture {
    type Output = Result<Option<VoiceShortcut>, IntentsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(IntentsError::framework))
    }
}

// ============================================================================
// AsyncVoiceShortcutCenter — async wrappers for INVoiceShortcutCenter
// ============================================================================

/// Async wrappers for [`VoiceShortcutCenter`] / `INVoiceShortcutCenter`
/// operations.
///
/// # macOS availability
///
/// `INVoiceShortcutCenter` requires macOS 12.0+. Calls on earlier OS versions
/// resolve with `Err(IntentsError::Framework(...))`.
#[derive(Debug, Clone, Copy)]
pub struct AsyncVoiceShortcutCenter;

impl AsyncVoiceShortcutCenter {
    /// Asynchronously retrieve all voice shortcuts.
    ///
    /// Wraps `INVoiceShortcutCenter.getAllVoiceShortcuts(completion:)`.
    pub fn get_all(center: &VoiceShortcutCenter) -> AllVoiceShortcutsFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe { ffi::inx_voice_shortcut_center_get_all(center.as_ptr(), all_shortcuts_cb, ctx) };
        AllVoiceShortcutsFuture { inner: future }
    }

    /// Asynchronously look up a voice shortcut by UUID string.
    ///
    /// Wraps `INVoiceShortcutCenter.getVoiceShortcut(with:completion:)`.
    ///
    /// # Errors
    ///
    /// Returns [`IntentsError::InvalidArgument`] if `identifier` contains an
    /// interior NUL byte.
    pub fn get(
        center: &VoiceShortcutCenter,
        identifier: &str,
    ) -> Result<VoiceShortcutFuture, IntentsError> {
        let cstring = CString::new(identifier).map_err(|e| {
            IntentsError::invalid_argument(format!("identifier contains a NUL byte: {e}"))
        })?;
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::inx_voice_shortcut_center_get_by_identifier(
                center.as_ptr(),
                cstring.as_ptr(),
                voice_shortcut_cb,
                ctx,
            );
        }
        Ok(VoiceShortcutFuture { inner: future })
    }
}
