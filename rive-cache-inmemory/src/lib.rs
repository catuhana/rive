#![doc = include_str!("../README.md")]

mod builder;
mod config;
mod iter;
mod patch;
mod reference;
mod remove;
mod stats;
mod update;
mod util;

pub use builder::InMemoryCacheBuilder;
pub use config::Config;
pub use iter::{InMemoryCacheIter, ResourceIter};
pub use reference::IterReference;
pub use reference::Reference;
pub use stats::InMemoryCacheStats;

use dashmap::DashMap;
use rive_models::{
    channel::Channel,
    emoji::Emoji,
    id::{
        marker::{ChannelMarker, EmojiMarker, MessageMarker, ServerMarker, UserMarker},
        Id,
    },
    member::{Member, MemberCompositeKey},
    message::Message,
    server::Server,
    user::User,
};
use update::CacheUpdate;

/// An in-memory cache of Revolt data.
///
/// To use a cache instance in multiple tasks, consider wrapping it in an
/// [`std::sync::Arc`] or [`std::rc::Rc`].
#[derive(Debug, Default)]
pub struct InMemoryCache {
    config: Config,
    users: DashMap<Id<UserMarker>, User>,
    servers: DashMap<Id<ServerMarker>, Server>,
    channels: DashMap<Id<ChannelMarker>, Channel>,
    messages: DashMap<Id<MessageMarker>, Message>,
    emojis: DashMap<Id<EmojiMarker>, Emoji>,
    members: DashMap<MemberCompositeKey, Member>,
}

impl InMemoryCache {
    /// Create new [`InMemoryCache`] instance with default [`Config`].
    ///
    /// [`InMemoryCache`]: crate::InMemoryCache
    /// [`Config`]: crate::Config
    pub fn new() -> Self {
        Self::default()
    }

    /// Create new [`InMemoryCache`] instance with provided [`Config`].
    ///
    /// [`InMemoryCache`]: crate::InMemoryCache
    /// [`Config`]: crate::Config
    pub fn new_with_config(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Create a new builder to configure and construct an in-memory cache.
    pub const fn builder() -> InMemoryCacheBuilder {
        InMemoryCacheBuilder::new()
    }

    /// Clear the cache.
    pub fn clear(&self) {
        self.users.clear();
        self.servers.clear();
        self.channels.clear();
        self.messages.clear();
        self.emojis.clear();
        self.members.clear();
    }

    /// Create an interface for retrieving cache statistics.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use rive_cache_inmemory::InMemoryCache;
    ///
    /// let cache = InMemoryCache::new();
    ///
    /// // later in the code...
    /// let messages = cache.stats().messages();
    /// println!("messages count: {messages}");
    /// ```
    pub const fn stats(&self) -> InMemoryCacheStats {
        InMemoryCacheStats::new(self)
    }

    /// Create an interface for iterating over the various resources in the
    /// cache.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use rive_cache_inmemory::InMemoryCache;
    ///
    /// let cache = InMemoryCache::new();
    ///
    /// // later in the code...
    /// for user in cache.iter().users() {
    ///     println!("{}: {}#{}", user.id, user.username, user.discriminator);
    /// }
    /// ```
    pub const fn iter(&self) -> InMemoryCacheIter {
        InMemoryCacheIter::new(self)
    }

    /// Get a user by ID.
    pub fn user(&self, id: &Id<UserMarker>) -> Option<Reference<Id<UserMarker>, User>> {
        self.users.get(id).map(Reference::new)
    }

    /// Get a server by ID.
    pub fn server(&self, id: &Id<ServerMarker>) -> Option<Reference<Id<ServerMarker>, Server>> {
        self.servers.get(id).map(Reference::new)
    }

    /// Get a channel by ID.
    pub fn channel(&self, id: &Id<ChannelMarker>) -> Option<Reference<Id<ChannelMarker>, Channel>> {
        self.channels.get(id).map(Reference::new)
    }

    /// Get a message by ID.
    pub fn message(&self, id: &Id<MessageMarker>) -> Option<Reference<Id<MessageMarker>, Message>> {
        self.messages.get(id).map(Reference::new)
    }

    /// Get an emoji by ID.
    pub fn emoji(&self, id: &Id<EmojiMarker>) -> Option<Reference<Id<EmojiMarker>, Emoji>> {
        self.emojis.get(id).map(Reference::new)
    }

    /// Get a member by [`MemberCompositeKey`].
    ///
    /// [`MemberCompositeKey`]: rive_models::member::MemberCompositeKey
    pub fn member(&self, id: &MemberCompositeKey) -> Option<Reference<MemberCompositeKey, Member>> {
        self.members.get(id).map(Reference::new)
    }

    /// Update the cache with an incoming event.
    pub fn update(&self, event: &impl CacheUpdate) {
        event.update(self);
    }
}
