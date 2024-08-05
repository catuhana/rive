use std::collections::HashMap;

use serde::Deserialize;

use crate::id::{marker::AttachmentMarker, Id};

const fn default_as_true() -> bool {
    true
}

/// Uploaded attachment data
#[derive(Deserialize, Debug, Clone)]
pub struct UploadData {
    /// Attachment ID
    pub id: Id<AttachmentMarker>,
}

/// Restricted content type
#[derive(Deserialize, Debug, Clone)]
pub enum ContentType {
    Image,
    Video,
    Audio,
}

/// Tag information
#[derive(Deserialize, Debug, Clone)]
pub struct Tag {
    /// Max file size
    pub max_size: usize,

    /// Whether to use ULID as an ID type
    #[serde(default)]
    pub use_ulid: bool,

    /// Whether is the tag is enabled
    #[serde(default = "default_as_true")]
    pub enabled: bool,

    /// List of required fields to be served
    #[serde(default)]
    pub serve_if_field_present: Vec<String>,

    /// Restricted tag's content type
    pub restrict_content_type: Option<ContentType>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// Autumn version
    pub autumn: String,

    /// Tag data per name
    pub tags: HashMap<String, Tag>,

    /// JPEG quality in percents
    pub jpeg_quality: u8,
}
