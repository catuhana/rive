use std::{collections::hash_map::Entry, time::SystemTime};

use rive_models::{
    channel::Channel,
    emoji::Emoji,
    event::{
        BulkEvent, BulkMessageDeleteEvent, ChannelDeleteEvent, ChannelUpdateEvent,
        EmojiDeleteEvent, MessageAppendEvent, MessageDeleteEvent, MessageReactEvent,
        MessageRemoveReactionEvent, MessageUnreactEvent, MessageUpdateEvent, ReadyEvent,
        ServerCreateEvent, ServerDeleteEvent, ServerEvent, ServerMemberJoinEvent,
        ServerMemberLeaveEvent, ServerMemberUpdateEvent, ServerRoleDeleteEvent,
        ServerRoleUpdateEvent, ServerUpdateEvent, UserUpdateEvent,
    },
    member::{Member, MemberCompositeKey},
    message::Message,
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
            ServerEvent::Message(event) => cache.update(event),
            ServerEvent::MessageUpdate(event) => cache.update(event),
            ServerEvent::MessageAppend(event) => cache.update(event),
            ServerEvent::MessageReact(event) => cache.update(event),
            ServerEvent::MessageUnreact(event) => cache.update(event),
            ServerEvent::MessageRemoveReaction(event) => cache.update(event),
            ServerEvent::MessageDelete(event) => cache.update(event),
            ServerEvent::BulkMessageDelete(event) => cache.update(event),
            ServerEvent::EmojiCreate(event) => cache.update(event),
            ServerEvent::EmojiDelete(event) => cache.update(event),
            ServerEvent::ServerMemberJoin(event) => cache.update(event),
            ServerEvent::ServerMemberUpdate(event) => cache.update(event),
            ServerEvent::ServerMemberLeave(event) => cache.update(event),
            ServerEvent::ServerRoleUpdate(event) => cache.update(event),
            ServerEvent::ServerRoleDelete(event) => cache.update(event),
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

        if let Some(emojis) = &self.emojis {
            for emoji in emojis {
                cache.emojis.insert(emoji.id.clone(), emoji.clone());
            }
        }

        for member in &self.members {
            cache.members.insert(member.id.clone(), member.clone());
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

impl CacheUpdate for Message {
    fn update(&self, cache: &InMemoryCache) {
        cache.messages.insert(self.id.clone(), self.clone());
    }
}

impl CacheUpdate for MessageUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let message = match cache.message(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };
        let new_message = message.patch(&self.data);

        cache.messages.insert(new_message.id.clone(), new_message);
    }
}

impl CacheUpdate for MessageAppendEvent {
    fn update(&self, cache: &InMemoryCache) {
        let message = match cache.message(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };

        // it should work like this, right? it's 3 AM im kinda eepy
        let new_embeds = match message.embeds {
            Some(embeds) => match &self.append.embeds {
                Some(append_embeds) => {
                    let mut emb = embeds;
                    emb.extend(append_embeds.clone());
                    Some(emb)
                }
                None => Some(embeds),
            },
            None => self.append.embeds.clone(),
        };

        let new_message = Message {
            embeds: new_embeds,
            ..message
        };

        cache.messages.insert(new_message.id.clone(), new_message);
    }
}

impl CacheUpdate for MessageReactEvent {
    fn update(&self, cache: &InMemoryCache) {
        let message = match cache.message(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };

        let mut new_message = message.clone();
        new_message
            .reactions
            .entry(self.emoji_id.clone())
            .or_default()
            .insert(self.user_id.clone());

        cache.messages.insert(new_message.id.clone(), new_message);
    }
}

impl CacheUpdate for MessageUnreactEvent {
    fn update(&self, cache: &InMemoryCache) {
        let message = match cache.message(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };

        let mut new_message = message.clone();
        if let Entry::Occupied(mut entry) = new_message.reactions.entry(self.emoji_id.clone()) {
            entry.get_mut().remove(&self.user_id);
            if entry.get().is_empty() {
                entry.remove_entry();
            }
        };

        cache.messages.insert(new_message.id.clone(), new_message);
    }
}

impl CacheUpdate for MessageRemoveReactionEvent {
    fn update(&self, cache: &InMemoryCache) {
        let message = match cache.message(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };

        let mut new_message = message.clone();
        new_message.reactions.remove(&self.emoji_id);

        cache.messages.insert(new_message.id.clone(), new_message);
    }
}

impl CacheUpdate for MessageDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.messages.remove(&self.id);
    }
}

impl CacheUpdate for BulkMessageDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        for id in &self.ids {
            cache.messages.remove(id);
        }
    }
}

impl CacheUpdate for Emoji {
    fn update(&self, cache: &InMemoryCache) {
        cache.emojis.insert(self.id.clone(), self.clone());
    }
}

impl CacheUpdate for EmojiDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.emojis.remove(&self.id);
    }
}

impl CacheUpdate for ServerMemberJoinEvent {
    fn update(&self, cache: &InMemoryCache) {
        let id = MemberCompositeKey {
            server: self.id.clone(),
            user: self.user.clone(),
        };
        let member = Member {
            id: id.clone(),
            // TODO: should it be like that?
            joined_at: SystemTime::now().into(),
            nickname: Default::default(),
            avatar: Default::default(),
            roles: Default::default(),
            timeout: Default::default(),
        };

        cache.members.insert(id, member);
    }
}

impl CacheUpdate for ServerMemberUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let member = match cache.member(&self.id) {
            Some(channel) => channel.clone(),
            None => return,
        };
        let new_member = update_fields(member, &self.data, &self.clear);

        cache.members.insert(new_member.id.clone(), new_member);
    }
}

impl CacheUpdate for ServerMemberLeaveEvent {
    fn update(&self, cache: &InMemoryCache) {
        cache.members.remove(&MemberCompositeKey {
            server: self.id.clone(),
            user: self.user.clone(),
        });
    }
}

impl CacheUpdate for ServerRoleUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        let mut server = match cache.server(&self.id) {
            Some(server) => server.clone(),
            None => return,
        };
        let role = match server.roles.get(&self.role_id) {
            Some(role) => role.clone(),
            None => return,
        };

        let new_role = update_fields(role, &self.data, &self.clear);
        server.roles.insert(self.role_id.clone(), new_role);

        cache.servers.insert(self.id.clone(), server);
    }
}

impl CacheUpdate for ServerRoleDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        let mut server = match cache.server(&self.id) {
            Some(server) => server.clone(),
            None => return,
        };

        server.roles.remove(&self.role_id);

        cache.servers.insert(self.id.clone(), server);
    }
}
