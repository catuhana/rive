//! Utilities used by the bystander to process events.

use rive_models::{
    channel::Channel,
    emoji::Emoji,
    event::{
        AuthifierEvent, BulkEvent, BulkMessageDeleteEvent, ChannelAckEvent, ChannelDeleteEvent,
        ChannelGroupJoinEvent, ChannelGroupLeaveEvent, ChannelStartTypingEvent,
        ChannelStopTypingEvent, ChannelUpdateEvent, EmojiDeleteEvent, ErrorEvent,
        MessageAppendEvent, MessageDeleteEvent, MessageReactEvent, MessageRemoveReactionEvent,
        MessageUnreactEvent, MessageUpdateEvent, PongEvent, ReadyEvent, ServerCreateEvent,
        ServerDeleteEvent, ServerEvent, ServerMemberJoinEvent, ServerMemberLeaveEvent,
        ServerMemberUpdateEvent, ServerRoleDeleteEvent, ServerRoleUpdateEvent, ServerUpdateEvent,
        UserPlatformWipeEvent, UserRelationshipEvent, UserSettingsUpdateEvent, UserUpdateEvent,
        WebhookDeleteEvent, WebhookUpdateEvent,
    },
    message::Message,
    report::Report,
    webhook::Webhook,
};

mod private {
    pub trait Sealed {}
}

/// Trait for [`ServerEvent`] variants processing by the bystander.
///
/// This trait is sealed and cannot be implemented.
///
/// [`ServerEvent`]: rive_models::event::ServerEvent
pub trait StandbyEvent: private::Sealed {
    /// Return the inner event data or none depending on the match of the event.
    fn from_server_event(event: ServerEvent) -> Option<Self>
    where
        Self: Sized;
}

/// Macro for shorthand impl of the [`StandbyEvent`] for [`ServerEvent`]
/// variants.
macro_rules! impl_event {
    ($from:tt => $to:ty) => {
        impl private::Sealed for $to {}

        impl StandbyEvent for $to {
            fn from_server_event(event: ServerEvent) -> Option<Self> {
                match event {
                    ServerEvent::$from(e) => Some(e),
                    _ => None,
                }
            }
        }
    };

    ($from:tt => $to:ty, $($rest:tt)+) => {
        impl_event!($from => $to);
        impl_event!($($rest)+);
    };
}

impl_event!(
    Bulk => BulkEvent,
    Error => ErrorEvent,
    Ready => ReadyEvent,
    Pong => PongEvent,
    Message => Message,
    MessageUpdate => MessageUpdateEvent,
    MessageAppend => MessageAppendEvent,
    MessageDelete => MessageDeleteEvent,
    MessageReact => MessageReactEvent,
    MessageUnreact => MessageUnreactEvent,
    MessageRemoveReaction => MessageRemoveReactionEvent,
    BulkMessageDelete => BulkMessageDeleteEvent,
    ChannelCreate => Channel,
    ChannelUpdate => ChannelUpdateEvent,
    ChannelDelete => ChannelDeleteEvent,
    ChannelGroupJoin => ChannelGroupJoinEvent,
    ChannelGroupLeave => ChannelGroupLeaveEvent,
    ChannelStartTyping => ChannelStartTypingEvent,
    ChannelStopTyping => ChannelStopTypingEvent,
    ChannelAck => ChannelAckEvent,
    ServerCreate => ServerCreateEvent,
    ServerUpdate => ServerUpdateEvent,
    ServerDelete => ServerDeleteEvent,
    ServerMemberUpdate => ServerMemberUpdateEvent,
    ServerMemberJoin => ServerMemberJoinEvent,
    ServerMemberLeave => ServerMemberLeaveEvent,
    ServerRoleUpdate => ServerRoleUpdateEvent,
    ServerRoleDelete => ServerRoleDeleteEvent,
    UserUpdate => UserUpdateEvent,
    UserRelationship => UserRelationshipEvent,
    UserSettingsUpdate => UserSettingsUpdateEvent,
    UserPlatformWipe => UserPlatformWipeEvent,
    EmojiCreate => Emoji,
    EmojiDelete => EmojiDeleteEvent,
    WebhookCreate => Webhook,
    WebhookUpdate => WebhookUpdateEvent,
    WebhookDelete => WebhookDeleteEvent,
    ReportCreate => Report,
    Auth => AuthifierEvent
);
