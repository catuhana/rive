use rive_models::user::{PartialUser, User};

#[inline(always)]
fn unwrap<T: Clone>(a: &Option<T>, b: T) -> T {
    a.as_ref().map(Clone::clone).unwrap_or(b)
}

#[inline(always)]
fn either<T: Clone>(a: &Option<T>, b: Option<T>) -> Option<T> {
    a.as_ref().map(Clone::clone).or(b)
}

// TODO: maybe move this to rive_models?
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
