use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_file::IntentFile;
use crate::private::{self, RawObject, RetainedObject};

macro_rules! simple_enum {
    ($name:ident { $($variant:ident = $raw:expr,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)*
            Other(i64),
        }

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn from_raw(raw: i64) -> Self {
                match raw {
                    $($raw => Self::$variant,)*
                    other => Self::Other(other),
                }
            }

            #[allow(dead_code)]
            pub(crate) const fn raw_value(self) -> i64 {
                match self {
                    $(Self::$variant => $raw,)*
                    Self::Other(raw) => raw,
                }
            }
        }
    };
}

macro_rules! objc_wrapper {
    ($name:ident, $objc_class:literal, $context:literal) => {
        #[derive(Debug)]
        pub struct $name {
            raw: RetainedObject,
        }

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

            pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
                Ok(Self {
                    raw: unsafe { RetainedObject::from_owned(ptr, $context) }?,
                })
            }

            #[allow(dead_code)]
            pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
                Self { raw }
            }

            #[allow(dead_code)]
            pub(crate) fn new_blank() -> Result<Self, IntentsError> {
                Ok(Self::from_retained(private::create_blank_object(
                    Self::OBJC_CLASS,
                    $context,
                )?))
            }

            pub fn class_name(&self) -> String {
                private::class_name(self)
            }
        }

        impl RawObject for $name {
            fn as_ptr(&self) -> *mut c_void {
                self.raw.as_ptr()
            }
        }
    };
}

simple_enum!(MessageReactionType {
    Unknown = 0,
    Emoji = 1,
    Generic = 2,
});

simple_enum!(OutgoingMessageType {
    Unknown = 0,
    OutgoingMessageText = 1,
    OutgoingMessageAudio = 2,
});

simple_enum!(StickerType {
    Unknown = 0,
    Emoji = 1,
    Generic = 2,
});

objc_wrapper!(
    MessageLinkMetadata,
    "INMessageLinkMetadata",
    "message link metadata"
);
objc_wrapper!(MessageReaction, "INMessageReaction", "message reaction");
objc_wrapper!(
    SendMessageAttachment,
    "INSendMessageAttachment",
    "send-message attachment"
);
objc_wrapper!(Sticker, "INSticker", "sticker");

impl MessageLinkMetadata {
    pub fn new(
        site_name: Option<&str>,
        summary: Option<&str>,
        title: Option<&str>,
        open_graph_type: Option<&str>,
        link_url: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let site_name = site_name
            .map(|value| private::cstring(value, "message link metadata site name"))
            .transpose()?;
        let summary = summary
            .map(|value| private::cstring(value, "message link metadata summary"))
            .transpose()?;
        let title = title
            .map(|value| private::cstring(value, "message link metadata title"))
            .transpose()?;
        let open_graph_type = open_graph_type
            .map(|value| private::cstring(value, "message link metadata open graph type"))
            .transpose()?;
        let link_url = link_url
            .map(|value| private::cstring(value, "message link metadata URL"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_message_link_metadata_create(
                site_name
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                summary
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                title
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                open_graph_type
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                link_url
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating message link metadata") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn site_name(&self) -> Option<String> {
        private::string_property(self, "siteName")
    }

    pub fn summary(&self) -> Option<String> {
        private::string_property(self, "summary")
    }

    pub fn title(&self) -> Option<String> {
        private::string_property(self, "title")
    }

    pub fn open_graph_type(&self) -> Option<String> {
        private::string_property(self, "openGraphType")
    }

    pub fn link_url(&self) -> Option<String> {
        private::string_property(self, "linkURL")
    }
}

impl MessageReaction {
    pub fn new(
        reaction_type: MessageReactionType,
        reaction_description: Option<&str>,
        emoji: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let reaction_description = reaction_description
            .map(|value| private::cstring(value, "message reaction description"))
            .transpose()?;
        let emoji = emoji
            .map(|value| private::cstring(value, "message reaction emoji"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_message_reaction_create(
                reaction_type.raw_value(),
                reaction_description
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                emoji
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating message reaction") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn reaction_type(&self) -> MessageReactionType {
        private::integer_property(self, "reactionType")
            .map_or(MessageReactionType::Unknown, MessageReactionType::from_raw)
    }

    pub fn reaction_description(&self) -> Option<String> {
        private::string_property(self, "reactionDescription")
    }

    pub fn emoji(&self) -> Option<String> {
        private::string_property(self, "emoji")
    }
}

impl SendMessageAttachment {
    pub fn audio_message_file(file: &IntentFile) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_send_message_attachment_create_with_audio_file(file.as_ptr(), &mut error)
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating send-message attachment") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn audio_message_file_present(&self) -> bool {
        private::object_property(self, "audioMessageFile").is_some()
    }
}

impl Sticker {
    pub fn new(sticker_type: StickerType, emoji: Option<&str>) -> Result<Self, IntentsError> {
        let emoji = emoji
            .map(|value| private::cstring(value, "sticker emoji"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_sticker_create(
                sticker_type.raw_value(),
                emoji
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating sticker") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn sticker_type(&self) -> StickerType {
        private::integer_property(self, "type").map_or(StickerType::Unknown, StickerType::from_raw)
    }

    pub fn emoji(&self) -> Option<String> {
        private::string_property(self, "emoji")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let metadata = MessageLinkMetadata::new(
            Some("doom.fish"),
            Some("summary"),
            Some("title"),
            Some("article"),
            Some("https://doom.fish"),
        )?;
        let reaction = MessageReaction::new(MessageReactionType::Emoji, Some("Like"), Some("👍"))?;
        let sticker = Sticker::new(StickerType::Emoji, Some("😀"))?;

        assert_eq!(metadata.site_name().as_deref(), Some("doom.fish"));
        assert_eq!(reaction.reaction_type(), MessageReactionType::Emoji);
        assert_eq!(sticker.sticker_type(), StickerType::Emoji);
        Ok(())
    }
}
