use rive_models::{
    channel::{Channel, PartialChannel},
    server::{PartialServer, Server},
    user::{PartialUser, User},
};

#[inline(always)]
fn unwrap<T: Clone>(a: &Option<T>, b: T) -> T {
    a.as_ref().map(Clone::clone).unwrap_or(b)
}

#[inline(always)]
fn either<T: Clone>(a: &Option<T>, b: Option<T>) -> Option<T> {
    a.as_ref().map(Clone::clone).or(b)
}

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
