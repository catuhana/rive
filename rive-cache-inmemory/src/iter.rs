use dashmap::iter::Iter;
use rive_models::{
    channel::Channel,
    emoji::Emoji,
    member::{Member, MemberCompositeKey},
    message::Message,
    server::Server,
    user::User,
};
use std::hash::Hash;

use crate::{reference::IterReference, InMemoryCache};

/// Immutable cache iterator.
pub struct ResourceIter<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> ResourceIter<'a, K, V> {
    pub(crate) const fn new(iter: Iter<'a, K, V>) -> Self {
        Self { iter }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for ResourceIter<'a, K, V> {
    type Item = IterReference<'a, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(IterReference::new)
    }
}

/// Interface to create iterators over various resources.
///
/// The created iterators will iterate over all entities of a resource.
///
/// The iteration order of all iterators are arbitrary.
///
/// Example:
///
/// ```no_run
/// use rive_cache_inmemory::InMemoryCache;
///
/// let cache = InMemoryCache::new();
///
/// // later in the code...
/// let count = cache
///     .iter()
///     .users()
///     .filter(|user| user.avatar.is_some())
///     .count();
///
/// println!("users with avatar: {count}");
/// ```
#[derive(Clone, Debug)]
pub struct InMemoryCacheIter<'a>(&'a InMemoryCache);

impl<'a> InMemoryCacheIter<'a> {
    /// Create a new instance of cache iterator interface.
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    /// Create an iterator over the users in the cache.
    pub fn users(&'a self) -> ResourceIter<String, User> {
        ResourceIter::new(self.0.users.iter())
    }

    /// Create an iterator over the servers in the cache.
    pub fn serevrs(&'a self) -> ResourceIter<String, Server> {
        ResourceIter::new(self.0.servers.iter())
    }

    /// Create an iterator over the channels in the cache.
    pub fn channels(&'a self) -> ResourceIter<String, Channel> {
        ResourceIter::new(self.0.channels.iter())
    }

    /// Create an iterator over the messages in the cache.
    pub fn messages(&'a self) -> ResourceIter<String, Message> {
        ResourceIter::new(self.0.messages.iter())
    }

    /// Create an iterator over the emojis in the cache.
    pub fn emojis(&'a self) -> ResourceIter<String, Emoji> {
        ResourceIter::new(self.0.emojis.iter())
    }

    /// Create an iterator over the servers members in the cache.
    pub fn members(&'a self) -> ResourceIter<MemberCompositeKey, Member> {
        ResourceIter::new(self.0.members.iter())
    }
}
