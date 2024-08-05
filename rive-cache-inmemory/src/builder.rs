use crate::{Config, InMemoryCache};

/// Builder to configure and construct an [`InMemoryCache`].
///
/// [`InMemoryCache`]: crate::InMemoryCache
#[derive(Debug, Default)]
#[must_use]
pub struct InMemoryCacheBuilder(Config);

impl InMemoryCacheBuilder {
    /// Create a builder to configure and construct [`InMemoryCache`]
    pub const fn new() -> Self {
        Self(Config::new())
    }

    /// Set whether users should be cached.
    pub const fn cache_users(mut self, value: bool) -> Self {
        self.0.cache_users = value;
        self
    }

    /// Set whether servers should be cached.
    pub const fn cache_servers(mut self, value: bool) -> Self {
        self.0.cache_servers = value;
        self
    }

    /// Set whether channels should be cached.
    pub const fn cache_channels(mut self, value: bool) -> Self {
        self.0.cache_channels = value;
        self
    }

    /// Set whether messages should be cached.
    pub const fn cache_messages(mut self, value: bool) -> Self {
        self.0.cache_messages = value;
        self
    }

    /// Set whether emojis should be cached.
    pub const fn cache_emojis(mut self, value: bool) -> Self {
        self.0.cache_emojis = value;
        self
    }

    /// Set whether members should be cached.
    pub const fn cache_members(mut self, value: bool) -> Self {
        self.0.cache_members = value;
        self
    }

    /// Consume the builder, returning a configured cache.
    pub fn build(self) -> InMemoryCache {
        InMemoryCache::new_with_config(self.0)
    }
}
