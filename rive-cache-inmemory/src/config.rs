#[derive(Debug, Clone)]
pub struct Config {
    pub cache_users: bool,
    pub cache_servers: bool,
    pub cache_channels: bool,
    pub cache_messages: bool,
    pub cache_emojis: bool,
    pub cache_members: bool,
}

impl Config {
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

    pub fn cache_users(mut self, value: bool) -> Self {
        self.cache_users = value;
        self
    }

    pub fn cache_servers(mut self, value: bool) -> Self {
        self.cache_servers = value;
        self
    }

    pub fn cache_channels(mut self, value: bool) -> Self {
        self.cache_channels = value;
        self
    }

    pub fn cache_messages(mut self, value: bool) -> Self {
        self.cache_messages = value;
        self
    }

    pub fn cache_emojis(mut self, value: bool) -> Self {
        self.cache_emojis = value;
        self
    }

    pub fn cache_members(mut self, value: bool) -> Self {
        self.cache_members = value;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
