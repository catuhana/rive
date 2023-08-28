pub mod patch;
mod reference;
pub mod remove;
mod stats;
pub mod update;

pub use reference::Reference;
pub use stats::InMemoryCacheStats;

use dashmap::DashMap;
use rive_models::user::User;
use update::CacheUpdate;

#[derive(Debug, Clone, Default)]
pub struct InMemoryCache {
    users: DashMap<String, User>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&self) {
        self.users.clear();
    }

    pub const fn stats(&self) -> InMemoryCacheStats {
        InMemoryCacheStats::new(self)
    }

    pub fn user(&self, id: impl Into<String>) -> Option<Reference<String, User>> {
        self.users.get(&id.into()).map(Reference::new)
    }

    pub fn update(&self, event: &impl CacheUpdate) {
        event.update(self);
    }
}
