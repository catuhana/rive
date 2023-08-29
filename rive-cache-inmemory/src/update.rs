use rive_models::{
    channel::Channel,
    event::{
        BulkEvent, ChannelDeleteEvent, ChannelUpdateEvent, ReadyEvent, ServerCreateEvent,
        ServerDeleteEvent, ServerEvent, ServerUpdateEvent, UserUpdateEvent,
    },
};

use crate::{patch::Patch, remove::Remove, util::channel_id, InMemoryCache};

#[inline(always)]
fn update_fields<T: Patch<P> + Remove<F>, P, F>(object: T, partial: &P, fields: &Vec<F>) -> T {
    let mut new_object = object.patch(partial);
    for field in fields {
        new_object = new_object.remove(field);
    }

    new_object
}

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
            ServerEvent::ChannelCreate(event) => cache.update(event),
            ServerEvent::ChannelUpdate(event) => cache.update(event),
            ServerEvent::ChannelDelete(event) => cache.update(event),
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

        for channel in &self.channels {
            cache
                .channels
                .insert(channel_id(channel).clone(), channel.clone());
        }
    }
}

impl CacheUpdate for UserUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let user = match cache.user(&self.id) {
            Some(user) => user.clone(),
            None => return,
        };
        let new_user = update_fields(user, &self.data, &self.clear);

        cache.users.insert(new_user.id.clone(), new_user.clone());
    }
}

impl CacheUpdate for ServerCreateEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.servers.insert(self.id.clone(), self.server.clone());
        for channel in &self.channels {
            cache
                .channels
                .insert(channel_id(channel).clone(), channel.clone());
        }
    }
}

impl CacheUpdate for ServerUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let server = match cache.server(&self.id) {
            Some(server) => server.clone(),
            None => return,
        };
        let new_server = update_fields(server.clone(), &self.data, &self.clear);

        cache.servers.insert(new_server.id.clone(), new_server);
    }
}

impl CacheUpdate for ServerDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.servers.remove(&self.id);
    }
}

impl CacheUpdate for Channel {
    fn update(&self, cache: &InMemoryCache) {
        cache
            .channels
            .insert(channel_id(self).clone(), self.clone());
    }
}

impl CacheUpdate for ChannelUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let channel = match cache.channel(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };
        let new_channel = update_fields(channel.clone(), &self.data, &self.clear);

        cache
            .channels
            .insert(channel_id(&new_channel).clone(), new_channel);
    }
}

impl CacheUpdate for ChannelDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.channels.remove(&self.id);
    }
}
