use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{
    attachment::Attachment,
    embed::Embed,
    id::{
        marker::{ChannelMarker, EmojiMarker, MessageMarker, UserMarker},
        Id,
    },
    member::Member,
    user::User,
};

/// Channel message
#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    /// Unique message ID
    #[serde(rename = "_id")]
    pub id: Id<MessageMarker>,

    /// Unique value generated by client sending this message
    pub nonce: Option<String>,

    /// ID of the channel this message was sent in
    pub channel: Id<ChannelMarker>,

    /// ID of the user that sent this message
    pub author: Id<UserMarker>,

    /// Message content
    pub content: Option<String>,

    /// System message
    pub system: Option<SystemMessage>,

    /// Array of attachments
    pub attachments: Option<Vec<Attachment>>,

    /// Time at which this message was last edited
    pub edited: Option<Timestamp>,

    /// Attached embeds to this message
    pub embeds: Option<Vec<Embed>>,

    /// Array of user ids mentioned in this message
    pub mentions: Option<Vec<Id<UserMarker>>>,

    /// Array of message ids this message is replying to
    pub replies: Option<Vec<Id<MessageMarker>>>,

    /// Hashmap of emoji IDs to array of user IDs
    #[serde(default)]
    pub reactions: HashMap<Id<EmojiMarker>, HashSet<Id<UserMarker>>>,

    /// Information about how this message should be interacted with
    #[serde(default)]
    pub interactions: Interactions,

    /// Name and / or avatar overrides for this message
    pub masquerade: Option<Masquerade>,
}

///Partial channel message
#[derive(Deserialize, Debug, Clone)]
pub struct PartialMessage {
    /// Unique message ID
    #[serde(rename = "_id")]
    pub id: Option<Id<MessageMarker>>,

    /// Unique value generated by client sending this message
    pub nonce: Option<String>,

    /// ID of the channel this message was sent in
    pub channel: Option<Id<ChannelMarker>>,

    /// ID of the user that sent this message
    pub author: Option<Id<UserMarker>>,

    /// Message content
    pub content: Option<String>,

    /// System message
    pub system: Option<SystemMessage>,

    /// Array of attachments
    pub attachments: Option<Vec<Attachment>>,

    /// Time at which this message was last edited
    pub edited: Option<Timestamp>,

    /// Attached embeds to this message
    pub embeds: Option<Vec<Embed>>,

    /// Array of user ids mentioned in this message
    pub mentions: Option<Vec<Id<UserMarker>>>,

    /// Array of message ids this message is replying to
    pub replies: Option<Vec<Id<MessageMarker>>>,

    /// Hashmap of emoji IDs to array of user IDs
    pub reactions: Option<HashMap<Id<EmojiMarker>, HashSet<Id<UserMarker>>>>,

    /// Information about how this message should be interacted with
    pub interactions: Option<Interactions>,

    /// Name and / or avatar overrides for this message
    pub masquerade: Option<Masquerade>,
}

/// Information to guide interactions on this message
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Interactions {
    /// Reactions which should always appear and be distinct
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reactions: Option<HashSet<String>>,
    /// Whether reactions should be restricted to the given list
    #[serde(default)]
    pub restrict_reactions: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Replace the avatar shown on this message (URL to image file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Replace the display role colour shown on this message
    ///
    /// Must have `ManageRole` permission to use
    ///
    /// This can be any valid CSS colour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

/// System message type
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemMessage {
    Text {
        content: String,
    },
    UserAdded {
        id: Id<UserMarker>,
        by: Id<UserMarker>,
    },
    UserRemove {
        id: Id<UserMarker>,
        by: Id<UserMarker>,
    },
    UserJoined {
        id: Id<UserMarker>,
    },
    UserLeft {
        id: Id<UserMarker>,
    },
    UserKicked {
        id: Id<UserMarker>,
    },
    UserBanned {
        id: Id<UserMarker>,
    },
    ChannelRenamed {
        name: String,
        by: Id<UserMarker>,
    },
    ChannelDescriptionChanged {
        by: Id<UserMarker>,
    },
    ChannelIconChanged {
        by: Id<UserMarker>,
    },
    ChannelOwnershipChanged {
        from: Id<UserMarker>,
        to: Id<UserMarker>,
    },
}

/// Sort used for retrieving messages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageSort {
    /// Sort by the most relevant messages
    Relevance,
    /// Sort by the newest messages first
    Latest,
    /// Sort by the oldest messages first
    Oldest,
}

/// Appended Information
#[derive(Deserialize, Debug, Clone)]
pub struct AppendMessage {
    /// Additional embeds to include in this message
    pub embeds: Option<Vec<Embed>>,
}

/// Response used when multiple messages are fetched
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BulkMessageResponse {
    JustMessages(
        /// List of messages
        Vec<Message>,
    ),
    MessagesAndUsers {
        /// List of messages
        messages: Vec<Message>,
        /// List of users
        users: Vec<User>,
        /// List of members
        members: Option<Vec<Member>>,
    },
}

/// Representation of a message reply before it is sent
#[derive(Serialize, Clone, Debug)]
pub struct Reply {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Whether this reply should mention the message's author
    pub mention: bool,
}
