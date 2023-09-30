//! Markers for various resource types, such as messages, channels or users.
//!
//! Markers themselves do not perform any logical action and are only used to
//! ensure that identifiers of the wrong types are not used.
//!
//! Markers are used in combination with [`Id`].
//!
//! [`Id`]: crate::id::Id

/// Marker for message IDs.
///
/// Types such as [`Message`] use this ID marker.
///
/// [`Message`]: crate::message::Message
#[derive(Debug)]
pub struct MessageMarker;

/// Marker for channel IDs.
///
/// Types such as [`Channel`] use this ID marker.
///
/// [`Channel`]: crate::channel::Channel
#[derive(Debug)]
pub struct ChannelMarker;

/// Marker for server IDs.
///
/// Types such as [`Server`] use this ID marker.
///
/// [`Server`]: crate::server::Server
#[derive(Debug)]
pub struct ServerMarker;

/// Marker for user IDs.
///
/// Types such as [`User`] use this ID marker.
///
/// [`User`]: crate::user::User
#[derive(Debug)]
pub struct UserMarker;

/// Marker for emoji IDs.
///
/// Types such as [`Emoji`] use this ID marker.
///
/// [`Emoji`]: crate::emoji::Emoji
#[derive(Debug)]
pub struct EmojiMarker;

/// Marker for account IDs.
///
/// Types such as [`AccountInfo`] use this ID marker.
///
/// [`AccountInfo`]: crate::account::AccountInfo
#[derive(Debug)]
pub struct AccountMarker;

/// Marker for attachment IDs.
///
/// Types such as [`Attachment`] use this ID marker.
///
/// [`Attachment`]: crate::attachment::Attachment
#[derive(Debug)]
pub struct AttachmentMarker;

/// Marker for attachment-associated object IDs.
///
/// Types such as [`Attachment`] use this ID marker.
///
/// [`Attachment`]: crate::attachment::Attachment
#[derive(Debug)]
pub struct ObjectMarker;

/// Marker for server role IDs.
///
/// Types such as [`Role`] use this ID marker.
///
/// [`Role`]: crate::server::Role
#[derive(Debug)]
pub struct RoleMarker;

/// Marker for invite IDs.
///
/// Types such as [`Invite`] use this ID marker.
///
/// [`Invite`]: crate::invite::Invite
#[derive(Debug)]
pub struct InviteMarker;

/// Marker for session IDs.
///
/// Types such as [`Session`] use this ID marker.
///
/// [`Session`]: crate::session::Session
#[derive(Debug)]
pub struct SessionMarker;

/// Marker for webhook IDs.
///
/// Types such as [`Webhook`] use this ID marker.
///
/// [`Webhook`]: crate::webhook::Webhook
#[derive(Debug)]
pub struct WebhookMarker;

/// Marker for multi-factor auth ticket IDs.
///
/// Types such as [`MFATicket`] use this ID marker.
///
/// [`MFATicket`]: crate::mfa::MFATicket
#[derive(Debug)]
pub struct MFATicketMarker;

/// Marker for report IDs.
///
/// Types such as [`Report`] use this ID marker.
///
/// [`Report`]: crate::report::Report
#[derive(Debug)]
pub struct ReportMarker;

/// Marker for category IDs.
///
/// Types such as [`Category`] use this ID marker.
///
/// [`Category`]: crate::server::Category
#[derive(Debug)]
pub struct CategoryMarker;

/// Marker for snapshot IDs.
///
/// Types such as [`Snapshot`] use this ID marker.
///
/// [`Snapshot`]: crate::snapshot::Snapshot
#[derive(Debug)]
pub struct SnapshotMarker;

/// Marker for account strike IDs.
///
/// Types such as [`AccountStrike`] use this ID marker.
///
/// [`AccountStrike`]: crate::strike::AccountStrike
#[derive(Debug)]
pub struct StrikeMarker;
