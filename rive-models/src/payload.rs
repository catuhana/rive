use iso8601_timestamp::Timestamp;
use serde::Serialize;

use crate::{
    bot::FieldsBot,
    channel::{ChannelType, FieldsChannel},
    embed::SendableEmbed,
    emoji::EmojiParent,
    member::FieldsMember,
    message::{Interactions, Masquerade, MessageSort, Reply},
    mfa::MFAData,
    permission::{Override, Permission},
    report::{ReportStatus, ReportedContent},
    server::{Category, FieldsRole, FieldsServer, SystemMessageChannels},
    user::{FieldsUser, PartialUserProfile, UserStatus},
};

#[allow(dead_code)]
fn if_false(t: &bool) -> bool {
    !t
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct SendMessagePayload {
    /// Message content to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Attachments to include in message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    /// Messages to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Reply>>,
    /// Embeds to include in message
    ///
    /// Text embed content contributes to the content length cap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
    /// Masquerade to apply to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,
    /// Information about how this message should be interacted with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Interactions>,
}

/// User data
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditUserPayload {
    /// New user status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<PartialUserProfile>,
    /// Attachment ID for avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Fields to remove from user object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsUser>>,
}

/// Change username data
#[derive(Serialize, Debug, Clone)]
pub struct ChangeUsernamePayload {
    /// New username
    pub username: String,
    /// Current username password
    pub password: String,
}

/// Send friend request data
#[derive(Serialize, Debug, Clone)]
pub struct SendFriendRequestPayload {
    /// Friend's usernane
    pub username: String,
}

/// Edit channel data
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditChannelPayload {
    /// Channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Channel description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Group owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Icon attachment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Whether this channel is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Fields to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsChannel>>,
}

/// Set role permission payload data
#[derive(Serialize, Debug, Clone, Default)]
pub struct SetRolePermissionPayload {
    /// Representation of a single permission override
    pub permissions: Override,
}

/// Set role permission payload data
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SetDefaultPermissionPayload {
    Value {
        /// Permission values to set for members in a [Channel::Group]
        permissions: Permission,
    },
    Field {
        /// Allow / deny values to set for members in this [Channels::TextChannel] or [Channels::VoiceChannel]
        permissions: Override,
    },
}

/// Query parameters
#[derive(Serialize, Debug, Clone, Default)]
pub struct FetchMessagesPayload {
    /// Maximum number of messages to fetch
    ///
    /// For fetching nearby messages, this is `(limit + 1)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Message sort direction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MessageSort>,
    /// Message id to search around
    ///
    /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
    /// It will also take half of limit rounded as the limits to each side.
    /// It also fetches the message ID specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearby: Option<String>,
    /// Whether to include user (and member, if server channel) objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}

/// Search Parameters
#[derive(Serialize, Debug, Clone, Default)]
pub struct SearchForMessagesPayload {
    /// Full-text search query
    ///
    /// See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information.
    pub query: String,

    /// Maximum number of messages to fetch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Message sort direction
    ///
    /// By default, it will be sorted by relevance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MessageSort>,
    /// Whether to include user (and member, if server channel) objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}

/// Message details
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditMessagePayload {
    /// New message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Embeds to include in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
}

/// Search parameters
#[derive(Serialize, Debug, Clone)]
pub struct BulkDeleteMessagesPayload {
    /// Message IDs
    pub ids: Vec<String>,
}

/// Reactions remove options
#[derive(Serialize, Debug, Clone, Default)]
pub struct RemoveReactionToMessagePayload {
    /// Remove a specific user's reaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Remove all reactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all: Option<bool>,
}

/// Group create data
#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateGroupPayload {
    /// Group name
    name: String,
    /// Group description
    description: Option<String>,
    /// Array of user IDs to add to the group
    ///
    /// Must be friends with these users.
    users: Vec<String>,
    /// Whether this group is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
}

/// Bot create data
#[derive(Serialize, Debug, Clone)]
pub struct CreateBotPayload {
    /// Bot username
    name: String,
}

/// Bot invite data
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InviteBotPayload {
    /// Invite to a server
    Server {
        /// Server Id
        server: String,
    },
    /// Invite to a group
    Group {
        /// Group Id
        group: String,
    },
}

/// Bot edit data
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditBotPayload {
    /// Bot username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the bot can be added by anyone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Whether analytics should be gathered for this bot
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// Interactions URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<String>,
    /// Fields to remove from bot object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsBot>>,
}

/// Create server data
#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateServerPayload {
    /// Server name
    pub name: String,
    /// Server description
    pub description: Option<String>,
    /// Whether this server is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

/// Edit server data
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditServerPayload {
    /// Server name
    pub name: Option<String>,
    /// Server description
    pub description: Option<String>,

    /// Attachment Id for icon
    pub icon: Option<String>,
    /// Attachment Id for banner
    pub banner: Option<String>,

    /// Category structure for server
    pub categories: Option<Vec<Category>>,
    /// System message configuration
    pub system_messages: Option<SystemMessageChannels>,

    // Whether this server is age-restricted
    pub nsfw: Option<bool>,
    /// Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg)
    pub discoverable: Option<bool>,
    /// Whether analytics should be collected for this server
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    pub analytics: Option<bool>,

    /// Fields to remove from server object
    pub remove: Option<Vec<FieldsServer>>,
}

/// Create channel data
#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateChannelPayload {
    /// Channel type
    #[serde(rename = "type", default = "ChannelType::default")]
    pub channel_type: ChannelType,
    /// Channel name
    pub name: String,
    /// Channel description
    pub description: Option<String>,
    /// Whether this channel is age restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

/// Create emoji data
#[derive(Serialize, Debug, Clone)]
pub struct CreateEmojiPayload {
    /// Server name
    pub name: String,
    /// Parent information
    pub parent: EmojiParent,
    /// Whether the emoji is mature
    pub nsfw: bool,
}

/// Fetch settings data
#[derive(Serialize, Debug, Clone)]
pub struct FetchSettingsPayload {
    /// Keys to fetch
    pub keys: Vec<String>,
}

/// Web push subscription data
#[derive(Serialize, Debug, Clone)]
pub struct PushSubscribePayload {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

/// Create role data
#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateRolePayload {
    /// Role name
    pub name: String,
    /// Ranking position
    ///
    /// Smaller values take priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// Edit role data
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditRolePayload {
    /// Role name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Role colour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    /// Whether this role should be displayed separately
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    /// Ranking position
    ///
    /// Smaller values take priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// Fields to remove from role object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsRole>>,
}

/// Server role permission value
#[derive(Serialize, Debug, Clone, Default)]
pub struct SetServerRolePermissionPayload {
    /// Allow / deny values for the role in this server.
    pub permissions: Override,
}

/// Default server role permission value
#[derive(Serialize, Debug, Clone, Default)]
pub struct SetDefaultRolePermissionPayload {
    /// Allow / deny values for the role in this server.
    pub permissions: Override,
}

/// Members query options
#[derive(Serialize, Debug, Clone, Default)]
pub struct FetchMembersPayload {
    /// Whether to exclude offline users
    #[serde(skip_serializing_if = "if_false")]
    pub exclude_offline: bool,
}

/// Member edit data
#[derive(Serialize, Debug, Clone, Default)]
pub struct EditMemberPayload {
    /// Member nickname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Attachment Id to set for avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Array of role ids
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Timestamp this member is timed out until
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timestamp>,
    /// Fields to remove from channel object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<FieldsMember>>,
}

/// Ban information
#[derive(Serialize, Debug, Clone, Default)]
pub struct BanUserPayload {
    /// Ban reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// New account data
#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateAccountPayload {
    /// Valid email address
    pub email: String,
    /// Password
    pub password: String,
    /// Invite code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite: Option<String>,
    /// Captcha verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
}

/// Resend information
#[derive(Serialize, Debug, Clone, Default)]
pub struct ResendVerificationPayload {
    /// Email associated with the account
    pub email: String,
    /// Captcha verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
}

/// Account deletion data
#[derive(Serialize, Debug, Clone)]
pub struct ConfirmAccountDeletionPayload {
    /// Deletion token
    pub token: String,
}

/// Change password data
#[derive(Serialize, Debug, Clone)]
pub struct ChangePasswordPayload {
    /// New password
    pub password: String,
    /// Current password
    pub current_password: String,
}

/// Change email data
#[derive(Serialize, Debug, Clone)]
pub struct ChangeEmailPayload {
    /// Valid email address
    pub email: String,
    /// Current password
    pub current_password: String,
}

/// Reset password information
#[derive(Serialize, Debug, Clone)]
pub struct SendPasswordResetPayload {
    /// Email associated with the account
    pub email: String,
    /// Captcha verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<String>,
}

/// Password reset data
#[derive(Serialize, Debug, Clone)]
pub struct PasswordResetPayload {
    /// Reset token
    pub token: String,
    /// New password
    pub password: String,
    /// Whether to logout all sessions
    #[serde(default)]
    pub remove_sessions: bool,
}

/// New user data
#[derive(Serialize, Debug, Clone)]
pub struct CompleteOnboardingPayload {
    /// New username which will be used to identify the user on the platform
    pub username: String,
}

/// Edit report data
#[derive(Serialize, Debug, Clone)]
pub struct EditReportPayload {
    /// New report status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReportStatus>,
    /// Report notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Report data
#[derive(Serialize, Debug, Clone)]
pub struct ReportContentPayload {
    /// Content being reported
    pub content: ReportedContent,
    /// Additional report description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<String>,
}

/// New strike information
#[derive(Serialize, Debug, Clone)]
pub struct CreateStrikePayload {
    /// ID of reported user
    pub user_id: String,

    /// Attached reason
    pub reason: String,
}

/// Strike information edit data
#[derive(Serialize, Debug, Clone)]
pub struct EditAccountStrikePayload {
    /// New attached reason
    pub reason: String,
}

/// Login data
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum LoginPayload {
    Email {
        /// Email
        email: String,
        /// Password
        password: String,
        /// Friendly name used for the session
        #[serde(skip_serializing_if = "Option::is_none")]
        friendly_name: Option<String>,
    },
    MFA {
        /// Unvalidated or authorised MFA ticket
        ///
        /// Used to resolve the correct account
        mfa_ticket: String,
        /// Valid MFA response
        ///
        /// This will take precedence over the `password` field where applicable
        #[serde(skip_serializing_if = "Option::is_none")]
        mfa_response: Option<MFAData>,
        /// Friendly name used for the session
        #[serde(skip_serializing_if = "Option::is_none")]
        friendly_name: Option<String>,
    },
}

/// Sessions clear data
#[derive(Serialize, Debug, Clone, Default)]
pub struct DeleteAllSessionsPayload {
    #[serde(skip_serializing_if = "if_false")]
    pub revoke_self: bool,
}

/// Session edit data
#[derive(Serialize, Debug, Clone)]
pub struct EditSessionPayload {
    /// Session friendly name
    pub friendly_name: String,
}

/// MFA ticket create data
pub type CreateMFATicketPayload = MFAData;

/// TOTP secret generate data
pub type EnableTOTP2FAPayload = MFAData;
