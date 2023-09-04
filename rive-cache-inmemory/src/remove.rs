use rive_models::{
    channel::{Channel, FieldsChannel},
    member::{FieldsMember, Member},
    server::{FieldsRole, FieldsServer, Role, Server},
    user::{FieldsUser, User, UserProfile, UserStatus},
};

/// A trait for removing resource fields.
// TODO: maybe move this to rive_models?
pub trait Remove<T> {
    /// Return a resource with a removed field.
    fn remove(self, field: &T) -> Self;
}

impl Remove<FieldsUser> for User {
    fn remove(self, field: &FieldsUser) -> Self {
        match field {
            FieldsUser::Avatar => User {
                avatar: None,
                ..self
            },
            FieldsUser::StatusText => User {
                status: self.status.map(|s| UserStatus { text: None, ..s }),
                ..self
            },
            FieldsUser::StatusPresence => User {
                status: self.status.map(|s| UserStatus {
                    presence: None,
                    ..s
                }),
                ..self
            },
            FieldsUser::ProfileContent => User {
                profile: self.profile.map(|p| UserProfile { content: None, ..p }),
                ..self
            },
            FieldsUser::ProfileBackground => User {
                profile: self.profile.map(|p| UserProfile {
                    background: None,
                    ..p
                }),
                ..self
            },
            FieldsUser::DisplayName => User {
                avatar: None,
                ..self
            },
        }
    }
}

impl Remove<FieldsServer> for Server {
    fn remove(self, field: &FieldsServer) -> Self {
        match field {
            FieldsServer::Description => Server {
                description: None,
                ..self
            },
            FieldsServer::Categories => Server {
                categories: None,
                ..self
            },
            FieldsServer::SystemMessages => Server {
                system_messages: None,
                ..self
            },
            FieldsServer::Icon => Server { icon: None, ..self },
            FieldsServer::Banner => Server {
                banner: None,
                ..self
            },
        }
    }
}

impl Remove<FieldsChannel> for Channel {
    fn remove(self, field: &FieldsChannel) -> Self {
        match self {
            Channel::Group {
                id,
                name,
                owner,
                description,
                recipients,
                icon,
                last_message_id,
                permissions,
                nsfw,
            } => Channel::Group {
                id,
                name,
                owner,
                description: match field {
                    FieldsChannel::Description => None,
                    _ => description,
                },
                recipients,
                icon: match field {
                    FieldsChannel::Icon => None,
                    _ => icon,
                },
                last_message_id,
                permissions,
                nsfw,
            },
            Channel::TextChannel {
                id,
                server,
                name,
                description,
                icon,
                last_message_id,
                default_permissions,
                role_permissions,
                nsfw,
            } => Channel::TextChannel {
                id,
                server,
                name,
                description: match field {
                    FieldsChannel::Description => None,
                    _ => description,
                },
                icon: match field {
                    FieldsChannel::Icon => None,
                    _ => icon,
                },
                last_message_id,
                default_permissions: match field {
                    FieldsChannel::DefaultPermissions => None,
                    _ => default_permissions,
                },
                role_permissions,
                nsfw,
            },
            Channel::VoiceChannel {
                id,
                server,
                name,
                description,
                icon,
                default_permissions,
                role_permissions,
                nsfw,
            } => Channel::VoiceChannel {
                id,
                server,
                name,
                description: match field {
                    FieldsChannel::Description => None,
                    _ => description,
                },
                icon: match field {
                    FieldsChannel::Icon => None,
                    _ => icon,
                },
                default_permissions: match field {
                    FieldsChannel::DefaultPermissions => None,
                    _ => default_permissions,
                },
                role_permissions,
                nsfw,
            },
            _ => self,
        }
    }
}

impl Remove<FieldsMember> for Member {
    fn remove(self, field: &FieldsMember) -> Self {
        match field {
            FieldsMember::Nickname => Member {
                nickname: None,
                ..self
            },
            FieldsMember::Avatar => Member {
                avatar: None,
                ..self
            },
            FieldsMember::Roles => Member {
                roles: vec![],
                ..self
            },
            FieldsMember::Timeout => Member {
                timeout: None,
                ..self
            },
        }
    }
}

impl Remove<FieldsRole> for Role {
    fn remove(self, field: &FieldsRole) -> Self {
        match field {
            FieldsRole::Colour => Role {
                colour: None,
                ..self
            },
        }
    }
}
