use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::attachment::Attachment;

/// User's relationship with another user (or themselves)
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RelationshipStatus {
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther,
}

/// Relationship entry indicating current status with other user
#[derive(Deserialize, Debug, Clone)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: RelationshipStatus,
}

/// Mutual servers and friends
#[derive(Deserialize, Debug, Clone)]
pub struct Mutuals {
    /// Array of mutual user IDs that both users are friends with
    pub users: Vec<String>,
    /// Array of mutual server IDs that both users are in
    pub servers: Vec<String>,
}

/// Presence status
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Presence {
    /// User is online
    Online,
    /// User is not currently available
    Idle,
    /// User is focusing / will only receive mentions
    Focus,
    /// User is busy / will not receive any notifications
    Busy,
    /// User appears to be offline
    Invisible,
}

/// User's active status
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserStatus {
    /// Custom status text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Current presence option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

/// User's profile
#[derive(Deserialize, Debug, Clone, Default)]
pub struct UserProfile {
    /// Text content on user's profile
    pub content: Option<String>,
    /// Background visible on user's profile
    pub background: Option<Attachment>,
}

/// Partial user's profile
///
/// This object not contains additional background attachment data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialUserProfile {
    /// Text to set as user profile description
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// Attachment Id for background
    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<String>,
}

/// Bot information for if the user is a bot
#[derive(Deserialize, Debug, Clone)]
pub struct BotInformation {
    /// Id of the owner of this bot
    pub owner: String,
}

/// Representiation of a User on Revolt.
#[derive(Deserialize, Debug, Clone)]
pub struct User {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Username
    pub username: String,
    /// Avatar attachment
    pub avatar: Option<Attachment>,
    /// Relationships with other users
    pub relations: Option<Vec<Relationship>>,

    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// User's current status
    pub status: Option<UserStatus>,
    /// User's profile page
    pub profile: Option<UserProfile>,

    /// Enum of user flags
    pub flags: Option<u64>,
    /// Whether this user is privileged
    #[serde(default)]
    pub privileged: bool,
    /// Bot information
    pub bot: Option<BotInformation>,

    /// Current session user's relationship with this user
    pub relationship: Option<RelationshipStatus>,
    /// Whether this user is currently online
    pub online: Option<bool>,
}

/// Partial representiation of a User on Revolt.
#[derive(Deserialize, Debug, Clone)]
pub struct PartialUser {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Option<String>,
    /// Username
    pub username: Option<String>,
    /// Avatar attachment
    pub avatar: Option<Attachment>,
    /// Relationships with other users
    pub relations: Option<Vec<Relationship>>,

    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// User's current status
    pub status: Option<UserStatus>,
    /// User's profile page
    pub profile: Option<UserProfile>,

    /// Enum of user flags
    pub flags: Option<u64>,
    /// Whether this user is privileged
    pub privileged: Option<bool>,
    /// Bot information
    pub bot: Option<BotInformation>,

    /// Current session user's relationship with this user
    pub relationship: Option<RelationshipStatus>,
    /// Whether this user is currently online
    pub online: Option<bool>,
}

/// Optional fields on user object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
}

/// HashMap of user settings
/// Each key is mapped to a tuple consisting of the
/// revision timestamp and serialised data (in JSON format)
pub type UserSettings = HashMap<String, (i64, String)>;
