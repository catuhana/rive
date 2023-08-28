use rive_models::{
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
