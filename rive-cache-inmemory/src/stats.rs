use crate::InMemoryCache;

#[derive(Clone, Debug)]
pub struct InMemoryCacheStats<'a>(&'a InMemoryCache);

impl<'a> InMemoryCacheStats<'a> {
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    pub fn users(&self) -> usize {
        self.0.users.len()
    }

    pub fn serevrs(&self) -> usize {
        self.0.servers.len()
    }
}
