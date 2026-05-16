use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_object::Speakable;
use crate::private::{self, RawObject, RetainedObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VocabularyStringType {
    ContactName,
    ContactGroupName,
    PhotoTag,
    PhotoAlbumName,
    WorkoutActivityName,
    CarProfileName,
    CarName,
    PaymentsOrganizationName,
    PaymentsAccountNickname,
    NotebookItemTitle,
    NotebookItemGroupName,
    MediaPlaylistTitle,
    MediaMusicArtistName,
    MediaAudiobookTitle,
    MediaAudiobookAuthorName,
    MediaShowTitle,
    Unknown(i64),
}

impl VocabularyStringType {
    const fn raw_value(self) -> i64 {
        match self {
            Self::ContactName => 1,
            Self::ContactGroupName => 2,
            Self::PhotoTag => 100,
            Self::PhotoAlbumName => 101,
            Self::WorkoutActivityName => 200,
            Self::CarProfileName => 300,
            Self::CarName => 301,
            Self::PaymentsOrganizationName => 400,
            Self::PaymentsAccountNickname => 401,
            Self::NotebookItemTitle => 500,
            Self::NotebookItemGroupName => 501,
            Self::MediaPlaylistTitle => 700,
            Self::MediaMusicArtistName => 701,
            Self::MediaAudiobookTitle => 702,
            Self::MediaAudiobookAuthorName => 703,
            Self::MediaShowTitle => 704,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Debug)]
pub struct IntentVocabulary {
    raw: RetainedObject,
}

impl IntentVocabulary {
    pub fn shared() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_vocabulary_shared() };
        if ptr.is_null() {
            Err(IntentsError::framework(
                "INVocabulary is unavailable on this system".to_string(),
            ))
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
        Ok(Self {
            raw: unsafe { RetainedObject::from_owned(ptr, "INVocabulary") }?,
        })
    }

    pub fn class_name(&self) -> String {
        private::class_name(self)
    }

    pub fn set_vocabulary_strings(
        &self,
        values: &[&str],
        type_: VocabularyStringType,
    ) -> Result<(), IntentsError> {
        let cstrings = values
            .iter()
            .map(|value| private::cstring(value, "vocabulary string"))
            .collect::<Result<Vec<_>, _>>()?;
        let pointers = cstrings.iter().map(|value| value.as_ptr()).collect::<Vec<_>>();
        let ok = unsafe {
            ffi::inx_vocabulary_set_strings(
                self.as_ptr(),
                if pointers.is_empty() {
                    std::ptr::null()
                } else {
                    pointers.as_ptr()
                },
                pointers.len(),
                type_.raw_value(),
            )
        };
        if ok {
            Ok(())
        } else {
            Err(IntentsError::framework(
                "failed to set INVocabulary strings".to_string(),
            ))
        }
    }

    pub fn set_vocabulary_speakables<T: Speakable>(
        &self,
        values: &[&T],
        type_: VocabularyStringType,
    ) -> Result<(), IntentsError> {
        let pointers = values.iter().map(|value| value.as_ptr()).collect::<Vec<_>>();
        let ok = unsafe {
            ffi::inx_vocabulary_set_speakables(
                self.as_ptr(),
                if pointers.is_empty() {
                    std::ptr::null()
                } else {
                    pointers.as_ptr()
                },
                pointers.len(),
                type_.raw_value(),
            )
        };
        if ok {
            Ok(())
        } else {
            Err(IntentsError::framework(
                "failed to set INVocabulary speakables".to_string(),
            ))
        }
    }

    pub fn remove_all_vocabulary_strings(&self) -> Result<(), IntentsError> {
        let ok = unsafe { ffi::inx_vocabulary_remove_all(self.as_ptr()) };
        if ok {
            Ok(())
        } else {
            Err(IntentsError::framework(
                "failed to remove INVocabulary strings".to_string(),
            ))
        }
    }
}

impl RawObject for IntentVocabulary {
    fn as_ptr(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}
