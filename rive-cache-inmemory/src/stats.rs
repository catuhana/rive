use crate::InMemoryCache;

/// An interface for iterating over the various resources in the cache.
///
/// Example:
///
/// ```no_run
/// use rive_cache_inmemory::InMemoryCache;
///
/// let cache = InMemoryCache::new();
///
/// // later in the code...
/// let count = cache.stats().users();
/// println!("users cached: {count}");
/// ```
#[derive(Clone, Debug)]
pub struct InMemoryCacheStats<'a>(&'a InMemoryCache);

impl<'a> InMemoryCacheStats<'a> {
    /// Create a new stats interface instance.
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    /// Number of users in the cache.
    pub fn users(&'a self) -> usize {
        self.0.users.len()
    }

    /// Number of servers in the cache.
    pub fn serevrs(&'a self) -> usize {
        self.0.servers.len()
    }

    /// Number of channels in the cache.
    pub fn channels(&'a self) -> usize {
        self.0.channels.len()
    }

    /// Number of messages in the cache.
    pub fn messages(&'a self) -> usize {
        self.0.messages.len()
    }

    /// Number of emojis in the cache.
    pub fn emojis(&'a self) -> usize {
        self.0.emojis.len()
    }

    /// Number of members in the cache.
    pub fn members(&'a self) -> usize {
        self.0.members.len()
    }
}
