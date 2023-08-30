mod iter;
mod patch;
mod reference;
mod remove;
mod stats;
mod update;
mod util;

pub use iter::InMemoryCacheIter;
pub use reference::Reference;
pub use stats::InMemoryCacheStats;

use dashmap::DashMap;
use rive_models::{
    channel::Channel,
    emoji::Emoji,
    member::{Member, MemberCompositeKey},
    message::Message,
    server::Server,
    user::User,
};
use update::CacheUpdate;

#[derive(Debug, Clone, Default)]
pub struct InMemoryCache {
    users: DashMap<String, User>,
    servers: DashMap<String, Server>,
    channels: DashMap<String, Channel>,
    messages: DashMap<String, Message>,
    emojis: DashMap<String, Emoji>,
    members: DashMap<MemberCompositeKey, Member>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&self) {
        self.users.clear();
        self.servers.clear();
        self.channels.clear();
        self.messages.clear();
        self.emojis.clear();
        self.members.clear();
    }

    pub const fn stats(&self) -> InMemoryCacheStats {
        InMemoryCacheStats::new(self)
    }

    pub const fn iter(&self) -> InMemoryCacheIter {
        InMemoryCacheIter::new(self)
    }

    pub fn user(&self, id: &str) -> Option<Reference<String, User>> {
        self.users.get(id).map(Reference::new)
    }

    pub fn server(&self, id: &str) -> Option<Reference<String, Server>> {
        self.servers.get(id).map(Reference::new)
    }

    pub fn channel(&self, id: &str) -> Option<Reference<String, Channel>> {
        self.channels.get(id).map(Reference::new)
    }

    pub fn message(&self, id: &str) -> Option<Reference<String, Message>> {
        self.messages.get(id).map(Reference::new)
    }

    pub fn emoji(&self, id: &str) -> Option<Reference<String, Emoji>> {
        self.emojis.get(id).map(Reference::new)
    }

    pub fn member(&self, id: &MemberCompositeKey) -> Option<Reference<MemberCompositeKey, Member>> {
        self.members.get(id).map(Reference::new)
    }

    pub fn update(&self, event: &impl CacheUpdate) {
        event.update(self);
    }
}
