use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    attachment::Attachment,
    id::{
        marker::{CategoryMarker, ChannelMarker, RoleMarker, ServerMarker, UserMarker},
        Id,
    },
    member::MemberCompositeKey,
    permission::{OverrideField, Permission},
};

/// Representation of a server role
#[derive(Deserialize, Debug, Clone)]
pub struct Role {
    /// Role name
    pub name: String,
    /// Permissions available to this role
    pub permissions: OverrideField,
    /// Colour used for this role
    ///
    /// This can be any valid CSS colour
    pub colour: Option<String>,
    /// Whether this role should be shown separately on the member sidebar
    #[serde(default)]
    pub hoist: bool,
    /// Ranking of this role
    #[serde(default)]
    pub rank: i64,
}

/// New role response
#[derive(Deserialize, Debug, Clone)]
pub struct NewRole {
    /// ID of the role
    pub id: Id<RoleMarker>,
    /// New role
    pub role: Role,
}

/// Partial representation of a server role
#[derive(Deserialize, Debug, Clone)]
pub struct PartialRole {
    /// Role name
    pub name: Option<String>,
    /// Permissions available to this role
    pub permissions: Option<OverrideField>,
    /// Colour used for this role
    ///
    /// This can be any valid CSS colour
    pub colour: Option<String>,
    /// Whether this role should be shown separately on the member sidebar
    pub hoist: Option<bool>,
    /// Ranking of this role
    pub rank: Option<i64>,
}

/// Channel category
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    /// Unique ID for this category
    pub id: Id<CategoryMarker>,
    /// Title for this category
    pub title: String,
    /// Channels in this category
    pub channels: Vec<Id<ChannelMarker>>,
}

/// System message channel assignments
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SystemMessageChannels {
    /// ID of channel to send user join messages in
    pub user_joined: Option<Id<ChannelMarker>>,
    /// ID of channel to send user left messages in
    pub user_left: Option<Id<ChannelMarker>>,
    /// ID of channel to send user kicked messages in
    pub user_kicked: Option<Id<ChannelMarker>>,
    /// ID of channel to send user banned messages in
    pub user_banned: Option<Id<ChannelMarker>>,
}

bitflags::bitflags! {
    /// Server flag enum
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ServerFlags: u64 {
        const Verified = 1;
        const Official = 2;
    }
}

impl<'de> Deserialize<'de> for ServerFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

/// Representation of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Id<ServerMarker>,
    /// User id of the owner
    pub owner: Id<UserMarker>,

    /// Name of the server
    pub name: String,
    /// Description for the server
    pub description: Option<String>,

    /// Channels within this server
    // ! FIXME: this may be redundant
    pub channels: Vec<Id<ChannelMarker>>,
    /// Categories for this server
    pub categories: Option<Vec<Category>>,
    /// Configuration for sending system event messages
    pub system_messages: Option<SystemMessageChannels>,

    /// Roles for this server
    #[serde(default = "HashMap::<Id<RoleMarker>, Role>::new")]
    pub roles: HashMap<Id<RoleMarker>, Role>,
    /// Default set of server and channel permissions
    pub default_permissions: Permission,

    /// Icon attachment
    pub icon: Option<Attachment>,
    /// Banner attachment
    pub banner: Option<Attachment>,

    /// Enum of server flags
    pub flags: Option<ServerFlags>,

    /// Whether this server is flagged as not safe for work
    #[serde(default)]
    pub nsfw: bool,
    /// Whether to enable analytics
    #[serde(default)]
    pub analytics: bool,
    /// Whether this server should be publicly discoverable
    #[serde(default)]
    pub discoverable: bool,
}

/// Partial representation of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct PartialServer {
    /// User id of the owner
    pub owner: Option<Id<UserMarker>>,

    /// Name of the server
    pub name: Option<String>,
    /// Description for the server
    pub description: Option<String>,

    /// Channels within this server
    // ! FIXME: this may be redundant
    pub channels: Option<Vec<Id<ChannelMarker>>>,
    /// Categories for this server
    pub categories: Option<Vec<Category>>,
    /// Configuration for sending system event messages
    pub system_messages: Option<SystemMessageChannels>,

    /// Roles for this server
    pub roles: Option<HashMap<Id<RoleMarker>, Role>>,
    /// Default set of server and channel permissions
    pub default_permissions: Option<Permission>,

    /// Icon attachment
    pub icon: Option<Attachment>,
    /// Banner attachment
    pub banner: Option<Attachment>,

    /// Enum of server flags
    pub flags: Option<ServerFlags>,

    /// Whether this server is flagged as not safe for work
    pub nsfw: Option<bool>,
    /// Whether to enable analytics
    pub analytics: Option<bool>,
    /// Whether this server should be publicly discoverable
    pub discoverable: Option<bool>,
}

/// Representation of a server ban
#[derive(Deserialize, Debug, Clone)]
pub struct ServerBan {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,
    /// Reason for ban creation
    pub reason: Option<String>,
}

/// Banned user
///
/// Just enough user information to list bans.
#[derive(Deserialize, Debug, Clone)]
pub struct BannedUser {
    /// Id of the banned user
    #[serde(rename = "_id")]
    pub id: Id<UserMarker>,
    /// Username of the banned user
    pub username: String,
    /// Avatar of the banned user
    pub avatar: Option<Attachment>,
}

/// Ban list
#[derive(Deserialize, Debug, Clone)]
pub struct BanList {
    /// Users objects
    pub users: Vec<BannedUser>,
    /// Ban objects
    pub bans: Vec<ServerBan>,
}

/// Optional fields on server object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsServer {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}

/// Optional fields on server object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsRole {
    Colour,
}
