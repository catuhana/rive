use serde::{Deserialize, Serialize};

use crate::{
    channel::{Channel, FieldsChannel, PartialChannel},
    emoji::Emoji,
    member::{FieldsMember, Member, MemberCompositeKey, PartialMember},
    message::{AppendMessage, Message, PartialMessage},
    server::{FieldsRole, FieldsServer, PartialRole, PartialServer, Server},
    user::{FieldsUser, PartialUser, RelationshipStatus, User, UserSettings},
};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ErrorId {
    LabelMe,
    InternalError {
        at: String,
    },
    InvalidSenssion,
    OnboardingNotFinished,
    AlreadyAuthenticated,
    MalformedData {
        msg: String,
    },
    #[serde(other)]
    Unknown,
}

/// Event sent by server
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerEvent {
    /// Multiple events
    Bulk(Bulk),

    /// Error
    Error(Error),

    /// Successfully authenticated
    Authenticated,

    /// Basic data to cache
    Ready(Ready),

    /// Ping response
    Pong(Pong),

    /// New message
    Message(Message),

    /// Update existing message
    MessageUpdate(MessageUpdate),

    /// Append information to existing message
    MessageAppend(MessageAppend),

    /// Delete message
    MessageDelete(MessageDelete),

    /// New reaction to a message
    MessageReact(MessageReact),

    /// Remove user's reaction from message
    MessageUnreact(MessageUnreact),

    /// Remove a reaction from message
    MessageRemoveReaction(MessageRemoveReaction),

    /// Bulk delete messages
    BulkMessageDelete(BulkMessageDelete),

    /// New channel
    ChannelCreate(Channel),

    /// Update existing channel
    ChannelUpdate(ChannelUpdate),

    /// Delete channel
    ChannelDelete(ChannelDelete),

    /// User joins a group
    ChannelGroupJoin(ChannelGroupJoin),

    /// User leaves a group
    ChannelGroupLeave(ChannelGroupLeave),

    /// User started typing in a channel
    ChannelStartTyping(ChannelStartTyping),

    /// User stopped typing in a channel
    ChannelStopTyping(ChannelStartTyping),

    /// User acknowledged message in channel
    ChannelAck(ChannelAck),

    /// New server
    ServerCreate(ServerCreate),

    /// Update existing server
    ServerUpdate(ServerUpdate),

    /// Delete server
    ServerDelete(ServerDelete),

    /// Update existing server member
    ServerMemberUpdate(ServerMemberUpdate),

    /// User joins server
    ServerMemberJoin(ServerMemberJoin),

    /// User left server
    ServerMemberLeave(ServerMemberLeave),

    /// Server role created or updated
    ServerRoleUpdate(ServerRoleUpdate),

    /// Server role deleted
    ServerRoleDelete(ServerRoleDelete),

    /// Update existing user
    UserUpdate(UserUpdate),

    /// Relationship with another user changed
    UserRelationship(UserRelationship),

    /// Settings updated remotely
    UserSettingsUpdate(UserSettingsUpdate),

    /// User has been platform banned or deleted their account
    ///
    /// Clients should remove the following associated data:
    /// - Messages
    /// - DM Channels
    /// - Relationships
    /// - Server Memberships
    ///
    /// User flags are specified to explain why a wipe is occurring though not all reasons will necessarily ever appear.
    UserPlatformWipe(UserPlatformWipe),

    /// New emoji
    EmojiCreate(Emoji),

    /// Delete emoji
    EmojiDelete(EmojiDelete),

    /// Unknown event
    ///
    /// If you received this event, please open an issue!
    #[serde(other)]
    Unknown,
}

/// Event sent by client
#[derive(Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Authenticate { token: String },
    Ping { data: i32 },
    BeginTyping { channel: String },
    EndTyping { channel: String },
}

/// Bulk event data
#[derive(Deserialize, Debug, Clone)]
pub struct Bulk {
    /// List of events
    pub v: Vec<ServerEvent>,
}

/// Error event data
#[derive(Deserialize, Debug, Clone)]
pub struct Error {
    /// Error ID
    pub error: ErrorId,
}

/// Ready event data
#[derive(Deserialize, Debug, Clone)]
pub struct Ready {
    /// List of users
    pub users: Vec<User>,
    /// List of servers
    pub servers: Vec<Server>,
    /// List of channels
    pub channels: Vec<Channel>,
    /// List of server members
    pub members: Vec<Member>,
    /// List of emojis
    pub emojis: Option<Vec<Emoji>>,
}

/// Server pong event data
#[derive(Deserialize, Debug, Clone)]
pub struct Pong {
    /// Client echo data
    pub data: u32,
}

/// Message update event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageUpdate {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel: String,
    /// Changed message data
    pub data: PartialMessage,
}

/// Message append event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageAppend {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel: String,
    /// Appended message information
    pub append: AppendMessage,
}

/// Message delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageDelete {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel: String,
}

/// Message react event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageReact {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel_id: String,
    /// Reaction author ID
    pub user_id: String,
    /// Emoji ID
    pub emoji_id: String,
}

/// Message reaction remove event
#[derive(Deserialize, Debug, Clone)]
pub struct MessageUnreact {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel_id: String,
    /// Reaction author ID
    pub user_id: String,
    /// Emoji ID
    pub emoji_id: String,
}

/// Message remove reaction event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageRemoveReaction {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel_id: String,
    /// Emoji ID
    pub emoji_id: String,
}

/// Bulk message delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct BulkMessageDelete {
    /// Channel ID
    pub channel: String,
    /// List of messages IDs
    pub ids: Vec<String>,
}

/// Channel update event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelUpdate {
    pub id: String,
    pub data: PartialChannel,
    pub clear: Vec<FieldsChannel>,
}

/// Channel delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelDelete {
    /// Deleted channel ID
    pub id: String,
}

/// Group join event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelGroupJoin {
    /// Group ID
    pub id: String,
    /// User ID
    pub user: String,
}

/// Group leave event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelGroupLeave {
    /// Group ID
    pub id: String,
    /// User ID
    pub user: String,
}

/// Channel start typing event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelStartTyping {
    /// Channel ID
    pub id: String,
    /// Typing user ID
    pub user: String,
}

/// Channel stop typing event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelStopTyping {
    /// Channel ID
    pub id: String,
    /// Typing user ID
    pub user: String,
}

/// Channel acknowledge event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelAck {
    /// Channel ID
    pub id: String,
    /// User ID
    pub user: String,
    /// Message ID
    pub message_id: String,
}

/// New server data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerCreate {
    /// Server ID
    pub id: String,
    /// Server information
    pub server: Server,
    /// List of server channels
    pub channels: Vec<Channel>,
}

/// Server update data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerUpdate {
    /// Server ID
    pub id: String,
    /// Server changed data
    pub data: PartialServer,
    /// List of removed optional server fields
    pub clear: Vec<FieldsServer>,
}

/// Server delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerDelete {
    /// Deleted server ID
    pub id: String,
}

/// Server member update event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerMemberUpdate {
    /// Member ID
    pub id: MemberCompositeKey,
    /// Member changed data
    pub data: PartialMember,
    /// List of removed optional member fields
    pub clear: Vec<FieldsMember>,
}

/// Member join event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerMemberJoin {
    /// Server ID
    pub id: String,
    /// User ID
    pub user: String,
}

/// Server member leave event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerMemberLeave {
    /// Server ID
    pub id: String,
    /// Leaved user ID
    pub user: String,
}

/// Server role update event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerRoleUpdate {
    /// Server ID
    pub id: String,
    /// Role  ID
    pub role_id: String,
    /// Changed role data
    pub data: PartialRole,
    /// Removed role optional fields
    pub clear: Vec<FieldsRole>,
}

/// Server role delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerRoleDelete {
    /// Server ID
    pub id: String,
    /// Role ID
    pub role_id: String,
}

/// User update event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserUpdate {
    /// User ID
    pub id: String,
    /// Changed user data
    pub data: PartialUser,
    /// Removed user fields
    pub clear: Vec<FieldsUser>,
}

/// User relationship update event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserRelationship {
    pub id: String,
    /// User with whom relationship changed
    pub user: User,
    /// New relationship status
    pub status: RelationshipStatus,
}

/// Settings update event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserSettingsUpdate {
    pub id: String,
    /// Updated settings
    pub update: UserSettings,
}

/// User wipe event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserPlatformWipe {
    /// Deleted user ID
    pub user_id: String,
    /// User flags
    pub flags: i32,
}

/// Emoji delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct EmojiDelete {
    /// Deleted emoji ID
    pub id: String,
}
