use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{EmojiMarker, ServerMarker, UserMarker},
    Id,
};

/// Information about what owns this emoji
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { id: Id<ServerMarker> },
    Detached,
}

/// Representation of an Emoji on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Emoji {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Id<EmojiMarker>,
    /// What owns this emoji
    pub parent: EmojiParent,
    /// Uploader user id
    pub creator_id: Id<UserMarker>,
    /// Emoji name
    pub name: String,
    /// Whether the emoji is animated
    #[serde(default)]
    pub animated: bool,
    /// Whether the emoji is marked as nsfw
    #[serde(default)]
    pub nsfw: bool,
}
