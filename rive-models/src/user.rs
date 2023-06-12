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

bitflags::bitflags! {
    /// User badge bitfield
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Badges: u64 {
        /// Revolt Developer
        const Developer = 1 << 0;
        /// Helped translate Revolt
        const Translator = 1 << 1;
        /// Monetarily supported Revolt
        const Supporter = 1 << 2;
        /// Responsibly disclosed a security issue
        const ResponsibleDisclosure = 1 << 3;
        /// Revolt Founder
        const Founder = 1 << 4;
        /// Platform moderator
        const PlatformModeration = 1 << 5;
        /// Active monetary supporter
        const ActiveSupporter = 1 << 6;
        /// 🦊🦝
        const Paw = 1 << 7;
        /// Joined as one of the first 1000 users in 2021
        const EarlyAdopter = 1 << 8;
        /// Amogus
        const ReservedRelevantJokeBadge1 = 1 << 9;
        /// Low resolution troll face
        const ReservedRelevantJokeBadge2 = 1 << 10;
    }
}
crate::impl_serde_bitflags!(Badges);

bitflags::bitflags! {
    /// User flag enum
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserFlags: u64 {
        /// User has been suspended from the platform
        const Suspended = 1;
        /// User has deleted their account
        const Deleted = 2;
        /// User was banned off the platform
        const Banned = 4;
        /// User was marked as spam and removed from platform
        const Spam = 8;
    }
}
crate::impl_serde_bitflags!(UserFlags);

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
    /// User discriminator (four numbers after the username)
    pub discriminator: String,
    /// User's display name
    pub display_name: Option<String>,
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
    pub flags: Option<UserFlags>,
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
    pub flags: Option<UserFlags>,
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
    DisplayName,
}

/// HashMap of user settings
/// Each key is mapped to a tuple consisting of the
/// revision timestamp and serialised data (in JSON format)
pub type UserSettings = HashMap<String, (i64, String)>;
