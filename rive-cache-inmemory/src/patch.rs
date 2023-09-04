use rive_models::{
    channel::{Channel, PartialChannel},
    member::{Member, PartialMember},
    message::{Message, PartialMessage},
    server::{PartialRole, PartialServer, Role, Server},
    user::{PartialUser, User},
};

/// Unwrap and clone [a] or return [b]
#[inline(always)]
fn unwrap<T: Clone>(a: &Option<T>, b: T) -> T {
    a.as_ref().map(Clone::clone).unwrap_or(b)
}

/// Return either [a] or [b]
#[inline(always)]
fn either<T: Clone>(a: &Option<T>, b: Option<T>) -> Option<T> {
    a.as_ref().map(Clone::clone).or(b)
}

/// A trait for updating resource fields.
// TODO: maybe move this to rive_models? or even make a rive_util crate with patch feature?
pub trait Patch<T> {
    fn patch(self, partial: &T) -> Self;
}

impl Patch<PartialUser> for User {
    fn patch(self, partial: &PartialUser) -> Self {
        User {
            id: unwrap(&partial.id, self.id),
            username: unwrap(&partial.username, self.username),
            discriminator: unwrap(&partial.discriminator, self.discriminator),
            display_name: either(&partial.display_name, self.display_name),
            avatar: either(&partial.avatar, self.avatar),
            relations: either(&partial.relations, self.relations),
            badges: either(&partial.badges, self.badges),
            status: either(&partial.status, self.status),
            profile: either(&partial.profile, self.profile),
            flags: either(&partial.flags, self.flags),
            privileged: unwrap(&partial.privileged, self.privileged),
            bot: either(&partial.bot, self.bot),
            relationship: either(&partial.relationship, self.relationship),
            online: either(&partial.online, self.online),
        }
    }
}

impl Patch<PartialServer> for Server {
    fn patch(self, partial: &PartialServer) -> Self {
        Server {
            id: self.id,
            owner: unwrap(&partial.owner, self.owner),
            name: unwrap(&partial.name, self.name),
            description: either(&partial.description, self.description),
            channels: unwrap(&partial.channels, self.channels),
            categories: either(&partial.categories, self.categories),
            system_messages: either(&partial.system_messages, self.system_messages),
            roles: unwrap(&partial.roles, self.roles),
            default_permissions: unwrap(&partial.default_permissions, self.default_permissions),
            icon: either(&partial.icon, self.icon),
            banner: either(&partial.banner, self.banner),
            flags: either(&partial.flags, self.flags),
            nsfw: unwrap(&partial.nsfw, self.nsfw),
            analytics: unwrap(&partial.analytics, self.analytics),
            discoverable: unwrap(&partial.discoverable, self.discoverable),
        }
    }
}

impl Patch<PartialChannel> for Channel {
    fn patch(self, partial: &PartialChannel) -> Channel {
        match self {
            Channel::SavedMessages { id, user } => Channel::SavedMessages { id, user },
            Channel::DirectMessage {
                id,
                active,
                recipients,
                last_message_id,
            } => Channel::DirectMessage {
                id,
                active: unwrap(&partial.active, active),
                recipients,
                last_message_id: either(&partial.last_message_id, last_message_id),
            },
            Channel::Group {
                id,
                name,
                owner,
                description,
                recipients,
                icon,
                last_message_id,
                permissions,
                nsfw,
            } => Channel::Group {
                id,
                name: unwrap(&partial.name, name),
                owner: unwrap(&partial.owner, owner),
                description: either(&partial.description, description),
                recipients,
                icon: either(&partial.icon, icon),
                last_message_id: either(&partial.last_message_id, last_message_id),
                permissions: either(&partial.permissions, permissions),
                nsfw: unwrap(&partial.nsfw, nsfw),
            },
            Channel::TextChannel {
                id,
                server,
                name,
                description,
                icon,
                last_message_id,
                default_permissions,
                role_permissions,
                nsfw,
            } => Channel::TextChannel {
                id,
                server,
                name: unwrap(&partial.name, name),
                description: either(&partial.description, description),
                icon: either(&partial.icon, icon),
                last_message_id: either(&partial.last_message_id, last_message_id),
                default_permissions: either(&partial.default_permissions, default_permissions),
                role_permissions: unwrap(&partial.role_permissions, role_permissions),
                nsfw: unwrap(&partial.nsfw, nsfw),
            },
            Channel::VoiceChannel {
                id,
                server,
                name,
                description,
                icon,
                default_permissions,
                role_permissions,
                nsfw,
            } => Channel::VoiceChannel {
                id,
                server,
                name: unwrap(&partial.name, name),
                description: either(&partial.description, description),
                icon: either(&partial.icon, icon),
                default_permissions: either(&partial.default_permissions, default_permissions),
                role_permissions: unwrap(&partial.role_permissions, role_permissions),
                nsfw: unwrap(&partial.nsfw, nsfw),
            },
        }
    }
}

impl Patch<PartialMessage> for Message {
    fn patch(self, partial: &PartialMessage) -> Self {
        Message {
            id: unwrap(&partial.id, self.id),
            nonce: either(&partial.nonce, self.nonce),
            channel: unwrap(&partial.channel, self.channel),
            author: unwrap(&partial.author, self.author),
            content: either(&partial.content, self.content),
            system: either(&partial.system, self.system),
            attachments: either(&partial.attachments, self.attachments),
            edited: either(&partial.edited, self.edited),
            embeds: either(&partial.embeds, self.embeds),
            mentions: either(&partial.mentions, self.mentions),
            replies: either(&partial.replies, self.replies),
            reactions: unwrap(&partial.reactions, self.reactions),
            interactions: unwrap(&partial.interactions, self.interactions),
            masquerade: either(&partial.masquerade, self.masquerade),
        }
    }
}

impl Patch<PartialMember> for Member {
    fn patch(self, partial: &PartialMember) -> Self {
        Member {
            id: unwrap(&partial.id, self.id),
            joined_at: unwrap(&partial.joined_at, self.joined_at),
            nickname: either(&partial.nickname, self.nickname),
            avatar: either(&partial.avatar, self.avatar),
            roles: unwrap(&partial.roles, self.roles),
            timeout: either(&partial.timeout, self.timeout),
        }
    }
}

impl Patch<PartialRole> for Role {
    fn patch(self, partial: &PartialRole) -> Self {
        Role {
            name: unwrap(&partial.name, self.name),
            permissions: unwrap(&partial.permissions, self.permissions),
            colour: either(&partial.colour, self.colour),
            hoist: unwrap(&partial.hoist, self.hoist),
            rank: unwrap(&partial.rank, self.rank),
        }
    }
}
