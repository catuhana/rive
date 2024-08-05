use serde::Deserialize;

use crate::id::{
    marker::{AttachmentMarker, MessageMarker, ObjectMarker, ServerMarker, UserMarker},
    Id,
};

/// Metadata associated with attachment
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(tag = "type")]
pub enum AttachmentMetadata {
    /// Attachment is just a generic uncategorised file
    #[default]
    File,

    /// Attachment contains textual data and should be displayed as such
    Text,

    /// Attachment is an image with specific dimensions
    Image { width: isize, height: isize },

    /// Attachment is a video with specific dimensions
    Video { width: isize, height: isize },

    /// Attachment is audio
    Audio,
}

/// Representation of an attachment on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Attachment {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Id<AttachmentMarker>,

    /// Tag/bucket this attachment was uploaded to
    pub tag: String,

    /// Original filename
    pub filename: String,

    /// Parsed metadata of this attachment
    pub metadata: AttachmentMetadata,

    /// Raw content type of this attachment
    pub content_type: String,

    /// Size of this attachment (in bytes)
    pub size: isize,

    /// Whether this attachment was deleted
    pub deleted: Option<bool>,

    /// Whether this attachment was reported
    pub reported: Option<bool>,

    // NOTE: These 3 fields will be deprecated in the next update
    pub message_id: Option<Id<MessageMarker>>,
    pub user_id: Option<Id<UserMarker>>,
    pub server_id: Option<Id<ServerMarker>>,

    /// ID of the object this attachment is associated with
    pub object_id: Option<Id<ObjectMarker>>,
}
