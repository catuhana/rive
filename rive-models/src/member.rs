use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    attachment::Attachment,
    id::{
        marker::{RoleMarker, ServerMarker, UserMarker},
        Id,
    },
    user::User,
};

/// Composite primary key consisting of server and user id
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MemberCompositeKey {
    /// Server Id
    pub server: Id<ServerMarker>,
    /// User Id
    pub user: Id<UserMarker>,
}

/// Representation of a member of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Member {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,

    /// Time at which this user joined the server
    pub joined_at: Timestamp,

    /// Member's nickname
    pub nickname: Option<String>,
    /// Avatar attachment
    pub avatar: Option<Attachment>,

    /// Member's roles
    #[serde(default)]
    pub roles: Vec<Id<RoleMarker>>,
    /// Timestamp this member is timed out until
    pub timeout: Option<Timestamp>,
}

/// Partial representation of a member of a server on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct PartialMember {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: Option<MemberCompositeKey>,

    /// Time at which this user joined the server
    pub joined_at: Option<Timestamp>,

    /// Member's nickname
    pub nickname: Option<String>,
    /// Avatar attachment
    pub avatar: Option<Attachment>,

    /// Member's roles
    pub roles: Option<Vec<Id<RoleMarker>>>,
    /// Timestamp this member is timed out until
    pub timeout: Option<Timestamp>,
}

/// Member List
///
/// Both lists are sorted by ID.
#[derive(Deserialize, Debug, Clone)]
pub struct MemberList {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}

/// Optional fields on server member object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsMember {
    Nickname,
    Avatar,
    Roles,
    Timeout,
}
