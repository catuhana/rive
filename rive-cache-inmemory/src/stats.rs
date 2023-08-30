use crate::InMemoryCache;

#[derive(Clone, Debug)]
pub struct InMemoryCacheStats<'a>(&'a InMemoryCache);

impl<'a> InMemoryCacheStats<'a> {
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    pub fn users(&'a self) -> usize {
        self.0.users.len()
    }

    pub fn serevrs(&'a self) -> usize {
        self.0.servers.len()
    }

    pub fn channels(&'a self) -> usize {
        self.0.channels.len()
    }

    pub fn messages(&'a self) -> usize {
        self.0.messages.len()
    }

    pub fn emojis(&'a self) -> usize {
        self.0.emojis.len()
    }
}
