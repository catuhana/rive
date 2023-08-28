use rive_models::event::{
    BulkEvent, ReadyEvent, ServerCreateEvent, ServerDeleteEvent, ServerEvent, ServerUpdateEvent,
    UserUpdateEvent,
};

use crate::{patch::Patch, remove::Remove, InMemoryCache};

pub trait CacheUpdate {
    fn update(&self, cache: &InMemoryCache);
}

impl CacheUpdate for ServerEvent {
    fn update(&self, cache: &InMemoryCache) {
        match self {
            ServerEvent::Bulk(event) => cache.update(event),
            ServerEvent::Ready(event) => cache.update(event),
            ServerEvent::UserUpdate(event) => cache.update(event),
            ServerEvent::ServerCreate(event) => cache.update(event),
            ServerEvent::ServerUpdate(event) => cache.update(event),
            ServerEvent::ServerDelete(event) => cache.update(event),
            _ => {}
        };
    }
}

impl CacheUpdate for BulkEvent {
    fn update(&self, cache: &InMemoryCache) {
        for event in &self.v {
            cache.update(event);
        }
    }
}

impl CacheUpdate for ReadyEvent {
    fn update(&self, cache: &InMemoryCache) {
        for user in &self.users {
            cache.users.insert(user.id.clone(), user.clone());
        }

        for server in &self.servers {
            cache.servers.insert(server.id.clone(), server.clone());
        }
    }
}

impl CacheUpdate for UserUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let user = match cache.user(&self.id) {
            Some(user) => user.clone(),
            None => return,
        };

        let mut new_user = user.patch(&self.data);
        for field in &self.clear {
            new_user = new_user.remove(field);
        }

        cache.users.insert(new_user.id.clone(), new_user);
    }
}

impl CacheUpdate for ServerCreateEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.servers.insert(self.id.clone(), self.server.clone());
        // TODO: insert self.channels
    }
}

impl CacheUpdate for ServerUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let server = match cache.server(&self.id) {
            Some(user) => user.clone(),
            None => return,
        };

        let mut new_server = server.patch(&self.data);
        for field in &self.clear {
            new_server = new_server.remove(field);
        }

        cache.servers.insert(new_server.id.clone(), new_server);
    }
}

impl CacheUpdate for ServerDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.servers.remove(&self.id);
    }
}
