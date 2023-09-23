use serde::{Deserialize, Serialize};

use crate::{
    channel::{Channel, FieldsChannel, PartialChannel},
    emoji::Emoji,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, MessageMarker, RoleMarker, ServerMarker, SessionMarker,
            UserMarker, WebhookMarker,
        },
        Id,
    },
    member::{FieldsMember, Member, MemberCompositeKey, PartialMember},
    message::{AppendMessage, Message, PartialMessage},
    report::Report,
    server::{FieldsRole, FieldsServer, PartialRole, PartialServer, Server},
    user::{FieldsUser, PartialUser, RelationshipStatus, User, UserSettings},
    webhook::{FieldsWebhook, PartialWebhook, Webhook},
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
    Bulk(BulkEvent),

    /// Error
    Error(ErrorEvent),

    /// Successfully authenticated
    Authenticated,

    /// Basic data to cache
    Ready(ReadyEvent),

    /// Ping response
    Pong(PongEvent),

    /// New message
    Message(Message),

    /// Update existing message
    MessageUpdate(MessageUpdateEvent),

    /// Append information to existing message
    MessageAppend(MessageAppendEvent),

    /// Delete message
    MessageDelete(MessageDeleteEvent),

    /// New reaction to a message
    MessageReact(MessageReactEvent),

    /// Remove user's reaction from message
    MessageUnreact(MessageUnreactEvent),

    /// Remove a reaction from message
    MessageRemoveReaction(MessageRemoveReactionEvent),

    /// Bulk delete messages
    BulkMessageDelete(BulkMessageDeleteEvent),

    /// New channel
    ChannelCreate(Channel),

    /// Update existing channel
    ChannelUpdate(ChannelUpdateEvent),

    /// Delete channel
    ChannelDelete(ChannelDeleteEvent),

    /// User joins a group
    ChannelGroupJoin(ChannelGroupJoinEvent),

    /// User leaves a group
    ChannelGroupLeave(ChannelGroupLeaveEvent),

    /// User started typing in a channel
    ChannelStartTyping(ChannelStartTypingEvent),

    /// User stopped typing in a channel
    ChannelStopTyping(ChannelStopTypingEvent),

    /// User acknowledged message in channel
    ChannelAck(ChannelAckEvent),

    /// New server
    ServerCreate(ServerCreateEvent),

    /// Update existing server
    ServerUpdate(ServerUpdateEvent),

    /// Delete server
    ServerDelete(ServerDeleteEvent),

    /// Update existing server member
    ServerMemberUpdate(ServerMemberUpdateEvent),

    /// User joins server
    ServerMemberJoin(ServerMemberJoinEvent),

    /// User left server
    ServerMemberLeave(ServerMemberLeaveEvent),

    /// Server role created or updated
    ServerRoleUpdate(ServerRoleUpdateEvent),

    /// Server role deleted
    ServerRoleDelete(ServerRoleDeleteEvent),

    /// Update existing user
    UserUpdate(UserUpdateEvent),

    /// Relationship with another user changed
    UserRelationship(UserRelationshipEvent),

    /// Settings updated remotely
    UserSettingsUpdate(UserSettingsUpdateEvent),

    /// User has been platform banned or deleted their account
    ///
    /// Clients should remove the following associated data:
    /// - Messages
    /// - DM Channels
    /// - Relationships
    /// - Server Memberships
    ///
    /// User flags are specified to explain why a wipe is occurring though not all reasons will necessarily ever appear.
    UserPlatformWipe(UserPlatformWipeEvent),

    /// New emoji
    EmojiCreate(Emoji),

    /// Delete emoji
    EmojiDelete(EmojiDeleteEvent),

    /// New webhook
    WebhookCreate(Webhook),

    /// Update existing webhook
    WebhookUpdate(WebhookUpdateEvent),

    /// Delete webhook
    WebhookDelete(WebhookDeleteEvent),

    /// New report
    ReportCreate(Report),

    /// Auth event
    Auth(AuthifierEvent),

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
    BeginTyping { channel: Id<ChannelMarker> },
    EndTyping { channel: Id<ChannelMarker> },
}

/// Authentication related events
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_type")]
pub enum AuthifierEvent {
    DeleteSession {
        user_id: Id<UserMarker>,
        session_id: Id<SessionMarker>,
    },
    DeleteAllSessions {
        user_id: Id<UserMarker>,
        exclude_session_id: Option<Id<SessionMarker>>,
    },
}

/// Bulk event data
#[derive(Deserialize, Debug, Clone)]
pub struct BulkEvent {
    /// List of events
    pub v: Vec<ServerEvent>,
}

/// Error event data
#[derive(Deserialize, Debug, Clone)]
pub struct ErrorEvent {
    /// Error ID
    pub error: ErrorId,
}

/// Ready event data
#[derive(Deserialize, Debug, Clone)]
pub struct ReadyEvent {
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
pub struct PongEvent {
    /// Client echo data
    pub data: u32,
}

/// Message update event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageUpdateEvent {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Channel ID
    pub channel: Id<ChannelMarker>,
    /// Changed message data
    pub data: PartialMessage,
}

/// Message append event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageAppendEvent {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Channel ID
    pub channel: Id<ChannelMarker>,
    /// Appended message information
    pub append: AppendMessage,
}

/// Message delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageDeleteEvent {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Channel ID
    pub channel: Id<ChannelMarker>,
}

/// Message react event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageReactEvent {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Channel ID
    pub channel_id: Id<ChannelMarker>,
    /// Reaction author ID
    pub user_id: Id<UserMarker>,
    /// Emoji ID
    pub emoji_id: Id<EmojiMarker>,
}

/// Message reaction remove event
#[derive(Deserialize, Debug, Clone)]
pub struct MessageUnreactEvent {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Channel ID
    pub channel_id: Id<ChannelMarker>,
    /// Reaction author ID
    pub user_id: Id<UserMarker>,
    /// Emoji ID
    pub emoji_id: Id<EmojiMarker>,
}

/// Message remove reaction event data
#[derive(Deserialize, Debug, Clone)]
pub struct MessageRemoveReactionEvent {
    /// Message ID
    pub id: Id<MessageMarker>,
    /// Channel ID
    pub channel_id: Id<ChannelMarker>,
    /// Emoji ID
    pub emoji_id: Id<EmojiMarker>,
}

/// Bulk message delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct BulkMessageDeleteEvent {
    /// Channel ID
    pub channel: Id<ChannelMarker>,
    /// List of messages IDs
    pub ids: Vec<Id<MessageMarker>>,
}

/// Channel update event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelUpdateEvent {
    pub id: Id<ChannelMarker>,
    pub data: PartialChannel,
    pub clear: Vec<FieldsChannel>,
}

/// Channel delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelDeleteEvent {
    /// Deleted channel ID
    pub id: Id<ChannelMarker>,
}

/// Group join event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelGroupJoinEvent {
    /// Group ID
    pub id: Id<ChannelMarker>,
    /// User ID
    pub user: Id<UserMarker>,
}

/// Group leave event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelGroupLeaveEvent {
    /// Group ID
    pub id: Id<ChannelMarker>,
    /// User ID
    pub user: Id<UserMarker>,
}

/// Channel start typing event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelStartTypingEvent {
    /// Channel ID
    pub id: Id<ChannelMarker>,
    /// Typing user ID
    pub user: Id<UserMarker>,
}

/// Channel stop typing event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelStopTypingEvent {
    /// Channel ID
    pub id: Id<ChannelMarker>,
    /// Typing user ID
    pub user: Id<UserMarker>,
}

/// Channel acknowledge event data
#[derive(Deserialize, Debug, Clone)]
pub struct ChannelAckEvent {
    /// Channel ID
    pub id: Id<ChannelMarker>,
    /// User ID
    pub user: Id<UserMarker>,
    /// Message ID
    pub message_id: Id<MessageMarker>,
}

/// New server data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerCreateEvent {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// Server information
    pub server: Server,
    /// List of server channels
    pub channels: Vec<Channel>,
}

/// Server update data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerUpdateEvent {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// Server changed data
    pub data: PartialServer,
    /// List of removed optional server fields
    pub clear: Vec<FieldsServer>,
}

/// Server delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerDeleteEvent {
    /// Deleted server ID
    pub id: Id<ServerMarker>,
}

/// Server member update event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerMemberUpdateEvent {
    /// Member ID
    pub id: MemberCompositeKey,
    /// Member changed data
    pub data: PartialMember,
    /// List of removed optional member fields
    pub clear: Vec<FieldsMember>,
}

/// Member join event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerMemberJoinEvent {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// User ID
    pub user: Id<UserMarker>,
}

/// Server member leave event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerMemberLeaveEvent {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// Leaved user ID
    pub user: Id<UserMarker>,
}

/// Server role update event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerRoleUpdateEvent {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// Role  ID
    pub role_id: Id<RoleMarker>,
    /// Changed role data
    pub data: PartialRole,
    /// Removed role optional fields
    pub clear: Vec<FieldsRole>,
}

/// Server role delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct ServerRoleDeleteEvent {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// Role ID
    pub role_id: Id<RoleMarker>,
}

/// User update event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserUpdateEvent {
    /// User ID
    pub id: Id<UserMarker>,
    /// Changed user data
    pub data: PartialUser,
    /// Removed user fields
    pub clear: Vec<FieldsUser>,
}

/// User relationship update event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserRelationshipEvent {
    pub id: Id<ServerMarker>,
    /// User with whom relationship changed
    pub user: User,
    /// New relationship status
    pub status: RelationshipStatus,
}

/// Settings update event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserSettingsUpdateEvent {
    pub id: String,
    /// Updated settings
    pub update: UserSettings,
}

/// User wipe event data
#[derive(Deserialize, Debug, Clone)]
pub struct UserPlatformWipeEvent {
    /// Deleted user ID
    pub user_id: Id<UserMarker>,
    /// User flags
    pub flags: i32,
}

/// Emoji delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct EmojiDeleteEvent {
    /// Deleted emoji ID
    pub id: Id<EmojiMarker>,
}

/// Webhook update event data
#[derive(Deserialize, Debug, Clone)]
pub struct WebhookUpdateEvent {
    /// Webhook ID
    pub id: Id<WebhookMarker>,
    /// Updated webhook data
    pub data: PartialWebhook,
    /// Fields removed from webhook
    pub remove: Vec<FieldsWebhook>,
}

/// Webhook delete event data
#[derive(Deserialize, Debug, Clone)]
pub struct WebhookDeleteEvent {
    pub id: Id<WebhookMarker>,
}
