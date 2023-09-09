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
        ServerRoleUpdateEvent, ServerUpdateEvent, UserPlatformWipeEvent, UserUpdateEvent,
    },
    member::{Member, MemberCompositeKey},
    message::Message,
    user::{User, UserFlags},
};

use crate::{patch::Patch, remove::Remove, util::channel_id, InMemoryCache};

/// A shorthand method to patch and remove fields of a given resource.
#[inline(always)]
fn update_fields<T: Patch<P> + Remove<F>, P, F>(resource: T, partial: &P, fields: &Vec<F>) -> T {
    let mut new_resource = resource.patch(partial);
    for field in fields {
        new_resource = new_resource.remove(field);
    }

    new_resource
}

/// Implemented and sealed trait for incoming events.
pub trait CacheUpdate: private::Sealed {
    /// Update the cache based on an event data.
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
            ServerEvent::UserPlatformWipe(event) => cache.update(event),
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
        if cache.config.cache_users {
            for user in &self.users {
                cache.users.insert(user.id.clone(), user.clone());
            }
        }

        if cache.config.cache_servers {
            for server in &self.servers {
                cache.servers.insert(server.id.clone(), server.clone());
            }
        }

        if cache.config.cache_channels {
            for channel in &self.channels {
                cache
                    .channels
                    .insert(channel_id(channel).clone(), channel.clone());
            }
        }

        if cache.config.cache_emojis {
            if let Some(emojis) = &self.emojis {
                for emoji in emojis {
                    cache.emojis.insert(emoji.id.clone(), emoji.clone());
                }
            }
        }

        if cache.config.cache_members {
            for member in &self.members {
                cache.members.insert(member.id.clone(), member.clone());
            }
        }
    }
}

impl CacheUpdate for UserUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.config.cache_users {
            return;
        }

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
        if !cache.config.cache_servers {
            return;
        }

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
        if !cache.config.cache_servers {
            return;
        }

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
        if cache.config.cache_servers {
            cache.servers.remove(&self.id);
        }
    }
}

impl CacheUpdate for Channel {
    fn update(&self, cache: &InMemoryCache) {
        if cache.config.cache_channels {
            cache
                .channels
                .insert(channel_id(self).clone(), self.clone());
        }
    }
}

impl CacheUpdate for ChannelUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.config.cache_channels {
            return;
        }

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
        if cache.config.cache_channels {
            cache.channels.remove(&self.id);
        }
    }
}

impl CacheUpdate for Message {
    fn update(&self, cache: &InMemoryCache) {
        if cache.config.cache_messages {
            cache.messages.insert(self.id.clone(), self.clone());
        }
    }
}

impl CacheUpdate for MessageUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.config.cache_messages {
            return;
        }

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
        if !cache.config.cache_messages {
            return;
        }

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
        if !cache.config.cache_messages {
            return;
        }

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
        if !cache.config.cache_messages {
            return;
        }

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
        if !cache.config.cache_messages {
            return;
        }

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
        if cache.config.cache_messages {
            cache.messages.remove(&self.id);
        }
    }
}

impl CacheUpdate for BulkMessageDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        if cache.config.cache_messages {
            for id in &self.ids {
                cache.messages.remove(id);
            }
        }
    }
}

impl CacheUpdate for Emoji {
    fn update(&self, cache: &InMemoryCache) {
        if cache.config.cache_emojis {
            cache.emojis.insert(self.id.clone(), self.clone());
        }
    }
}

impl CacheUpdate for EmojiDeleteEvent {
    fn update(&self, cache: &InMemoryCache) {
        if cache.config.cache_emojis {
            cache.emojis.remove(&self.id);
        }
    }
}

impl CacheUpdate for ServerMemberJoinEvent {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.config.cache_members {
            return;
        }

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
        if !cache.config.cache_members {
            return;
        }

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
        if !cache.config.cache_members {
            cache.members.remove(&MemberCompositeKey {
                server: self.id.clone(),
                user: self.user.clone(),
            });
        }
    }
}

impl CacheUpdate for ServerRoleUpdateEvent {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.config.cache_servers {
            return;
        }

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
        if !cache.config.cache_servers {
            return;
        }

        let mut server = match cache.server(&self.id) {
            Some(server) => server.clone(),
            None => return,
        };

        server.roles.remove(&self.role_id);

        cache.servers.insert(self.id.clone(), server);
    }
}

impl CacheUpdate for UserPlatformWipeEvent {
    fn update(&self, cache: &InMemoryCache) {
        // as documented, the following associated data should be removed:
        // - messages
        // - dm channels
        // - relationships
        // - server memberships
        if cache.config.cache_messages {
            cache.messages.retain(|_, v| v.author != self.user_id);
        }
        if cache.config.cache_members {
            cache.members.retain(|_, v| v.id.user != self.user_id);
        }
        if cache.config.cache_channels {
            cache.channels.retain(|_, v| match v {
                Channel::DirectMessage { recipients, .. } => recipients.contains(&self.user_id),
                _ => true,
            });
        }

        if !cache.config.cache_users {
            return;
        }

        let user = match cache.user(&self.user_id) {
            Some(user) => user.clone(),
            None => return,
        };

        let new_user = User {
            flags: Some(UserFlags::from_bits_retain(self.flags.try_into().unwrap())),
            ..user
        };

        cache.users.insert(self.user_id.clone(), new_user);
    }
}

mod private {
    use rive_models::{
        channel::Channel,
        emoji::Emoji,
        event::{
            BulkEvent, BulkMessageDeleteEvent, ChannelDeleteEvent, ChannelUpdateEvent,
            EmojiDeleteEvent, MessageAppendEvent, MessageDeleteEvent, MessageReactEvent,
            MessageRemoveReactionEvent, MessageUnreactEvent, MessageUpdateEvent, ReadyEvent,
            ServerCreateEvent, ServerDeleteEvent, ServerEvent, ServerMemberJoinEvent,
            ServerMemberLeaveEvent, ServerMemberUpdateEvent, ServerRoleDeleteEvent,
            ServerRoleUpdateEvent, ServerUpdateEvent, UserPlatformWipeEvent, UserUpdateEvent,
        },
        message::Message,
    };

    pub trait Sealed {}

    impl Sealed for BulkEvent {}
    impl Sealed for BulkMessageDeleteEvent {}
    impl Sealed for Channel {}
    impl Sealed for ChannelDeleteEvent {}
    impl Sealed for ChannelUpdateEvent {}
    impl Sealed for Emoji {}
    impl Sealed for EmojiDeleteEvent {}
    impl Sealed for Message {}
    impl Sealed for MessageAppendEvent {}
    impl Sealed for MessageDeleteEvent {}
    impl Sealed for MessageReactEvent {}
    impl Sealed for MessageRemoveReactionEvent {}
    impl Sealed for MessageUnreactEvent {}
    impl Sealed for MessageUpdateEvent {}
    impl Sealed for ReadyEvent {}
    impl Sealed for ServerCreateEvent {}
    impl Sealed for ServerDeleteEvent {}
    impl Sealed for ServerEvent {}
    impl Sealed for ServerMemberJoinEvent {}
    impl Sealed for ServerMemberLeaveEvent {}
    impl Sealed for ServerMemberUpdateEvent {}
    impl Sealed for ServerRoleDeleteEvent {}
    impl Sealed for ServerRoleUpdateEvent {}
    impl Sealed for ServerUpdateEvent {}
    impl Sealed for UserPlatformWipeEvent {}
    impl Sealed for UserUpdateEvent {}
}
