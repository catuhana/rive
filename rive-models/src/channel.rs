use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    attachment::Attachment,
    id::{
        marker::{
            ChannelMarker, InviteMarker, MessageMarker, RoleMarker, ServerMarker, UserMarker,
        },
        Id,
    },
    permission::{OverrideField, Permission},
};

/// Representation of a channel on Revolt
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "channel_type")]
pub enum Channel {
    /// Personal "Saved Notes" channel which allows users to save messages
    SavedMessages {
        /// Unique Id
        #[serde(rename = "_id")]
        id: Id<ChannelMarker>,
        /// Id of the user this channel belongs to
        user: Id<UserMarker>,
    },

    /// Direct message channel between two users
    DirectMessage {
        /// Unique Id
        #[serde(rename = "_id")]
        id: Id<ChannelMarker>,

        /// Whether this direct message channel is currently open on both sides
        active: bool,
        /// 2-tuple of user ids participating in direct message
        recipients: Vec<Id<UserMarker>>,
        /// Id of the last message sent in this channel
        last_message_id: Option<Id<MessageMarker>>,
    },

    /// Group channel between 1 or more participants
    Group {
        /// Unique Id
        #[serde(rename = "_id")]
        id: Id<ChannelMarker>,

        /// Display name of the channel
        name: String,
        /// User id of the owner of the group
        owner: Id<UserMarker>,
        /// Channel description
        description: Option<String>,
        /// Array of user ids participating in channel
        recipients: Vec<Id<UserMarker>>,

        /// Custom icon attachment
        icon: Option<Attachment>,
        /// Id of the last message sent in this channel
        last_message_id: Option<Id<MessageMarker>>,

        /// Permissions assigned to members of this group
        /// (does not apply to the owner of the group)
        permissions: Option<Permission>,

        /// Whether this group is marked as not safe for work
        #[serde(default)]
        nsfw: bool,
    },

    /// Text channel belonging to a server
    TextChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: Id<ChannelMarker>,
        /// Id of the server this channel belongs to
        server: String,

        /// Display name of the channel
        name: String,
        /// Channel description
        description: Option<String>,

        /// Custom icon attachment
        icon: Option<Attachment>,
        /// Id of the last message sent in this channel
        last_message_id: Option<Id<MessageMarker>>,

        /// Default permissions assigned to users in this channel
        default_permissions: Option<OverrideField>,
        /// Permissions assigned based on role to this channel
        #[serde(default = "HashMap::<Id<RoleMarker>, OverrideField>::new")]
        role_permissions: HashMap<Id<RoleMarker>, OverrideField>,

        /// Whether this channel is marked as not safe for work
        #[serde(default)]
        nsfw: bool,
    },

    /// Voice channel belonging to a server
    VoiceChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: Id<ChannelMarker>,
        /// Id of the server this channel belongs to
        server: Id<ServerMarker>,

        /// Display name of the channel
        name: String,
        /// Channel description
        description: Option<String>,
        /// Custom icon attachment
        icon: Option<Attachment>,

        /// Default permissions assigned to users in this channel
        default_permissions: Option<OverrideField>,
        /// Permissions assigned based on role to this channel
        #[serde(default = "HashMap::<Id<RoleMarker>, OverrideField>::new")]
        role_permissions: HashMap<Id<RoleMarker>, OverrideField>,

        /// Whether this channel is marked as not safe for work
        #[serde(default)]
        nsfw: bool,
    },
}

/// Partial values of [Channel]
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PartialChannel {
    /// Display name of the channel
    pub name: Option<String>,
    /// User id of the owner of the group
    pub owner: Option<Id<UserMarker>>,
    /// Channel description
    pub description: Option<String>,
    /// Custom icon attachment
    pub icon: Option<Attachment>,
    /// Whether this channel is marked as not safe for work
    pub nsfw: Option<bool>,
    /// Whether this direct message channel is currently open on both sides
    pub active: Option<bool>,
    /// Permissions assigned to members of this channel
    pub permissions: Option<Permission>,
    /// Permissions assigned based on role to this channel
    pub role_permissions: Option<HashMap<Id<RoleMarker>, OverrideField>>,
    /// Default permissions assigned to users in this channel
    pub default_permissions: Option<OverrideField>,
    /// Id of the last message sent in this channel
    pub last_message_id: Option<Id<MessageMarker>>,
}

/// Channel type
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub enum ChannelType {
    #[default]
    Text,
    Voice,
}

/// Optional fields on channel object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsChannel {
    Description,
    Icon,
    DefaultPermissions,
}

/// Representation of an invite to a channel on Revolt
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PartialInvite {
    /// Invite to a specific server channel
    Server {
        /// Invite code
        #[serde(rename = "_id")]
        code: Id<InviteMarker>,
        /// Id of the server this invite points to
        server: Id<ServerMarker>,
        /// Id of user who created this invite
        creator: Id<UserMarker>,
        /// Id of the server channel this invite points to
        channel: Id<ChannelMarker>,
    },
    /// Invite to a group channel
    Group {
        /// Invite code
        #[serde(rename = "_id")]
        code: Id<InviteMarker>,
        /// Id of user who created this invite
        creator: Id<UserMarker>,
        /// Id of the group channel this invite points to
        channel: Id<ChannelMarker>,
    },
}

/// Composite primary key consisting of channel and user ID
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ChannelCompositeKey {
    /// Channel ID
    pub channel: Id<ChannelMarker>,
    /// User ID
    pub user: Id<UserMarker>,
}

/// Representation of the state of a channel from the perspective of a user
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelUnread {
    /// Composite key pointing to a user's view of a channel
    #[serde(rename = "_id")]
    pub id: ChannelCompositeKey,

    /// ID of the last message read in this channel by a user
    pub last_id: Option<Id<MessageMarker>>,
    /// Array of message ids that mention the user
    pub mentions: Option<Vec<Id<MessageMarker>>>,
}
