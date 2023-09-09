/// Configuration for an [`InMemoryCache`].
///
/// [`InMemoryCache`]: crate::InMemoryCache
#[derive(Debug, Clone)]
pub struct Config {
    /// Whether to cache users
    pub cache_users: bool,

    /// Whether to cache servers
    pub cache_servers: bool,

    /// Whether to cache channels
    pub cache_channels: bool,

    /// Whether to cache messages
    pub cache_messages: bool,

    /// Whether to cache emojis
    pub cache_emojis: bool,

    /// Whether to cache members
    pub cache_members: bool,
}

impl Config {
    /// Create a new [`Config`].
    ///
    /// All resource caching are enabled by default.
    ///
    /// [`Config`]: crate::Config
    pub const fn new() -> Self {
        Self {
            cache_users: true,
            cache_servers: true,
            cache_channels: true,
            cache_messages: true,
            cache_emojis: true,
            cache_members: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
