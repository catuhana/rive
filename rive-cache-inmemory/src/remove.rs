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
            FieldsUser::Avatar => Self {
                avatar: None,
                ..self
            },
            FieldsUser::StatusText => Self {
                status: self.status.map(|s| UserStatus { text: None, ..s }),
                ..self
            },
            FieldsUser::StatusPresence => Self {
                status: self.status.map(|s| UserStatus {
                    presence: None,
                    ..s
                }),
                ..self
            },
            FieldsUser::ProfileContent => Self {
                profile: self.profile.map(|p| UserProfile { content: None, ..p }),
                ..self
            },
            FieldsUser::ProfileBackground => Self {
                profile: self.profile.map(|p| UserProfile {
                    background: None,
                    ..p
                }),
                ..self
            },
            FieldsUser::DisplayName => Self {
                avatar: None,
                ..self
            },
        }
    }
}

impl Remove<FieldsServer> for Server {
    fn remove(self, field: &FieldsServer) -> Self {
        match field {
            FieldsServer::Description => Self {
                description: None,
                ..self
            },
            FieldsServer::Categories => Self {
                categories: None,
                ..self
            },
            FieldsServer::SystemMessages => Self {
                system_messages: None,
                ..self
            },
            FieldsServer::Icon => Self { icon: None, ..self },
            FieldsServer::Banner => Self {
                banner: None,
                ..self
            },
        }
    }
}

impl Remove<FieldsChannel> for Channel {
    fn remove(self, field: &FieldsChannel) -> Self {
        match self {
            Self::Group {
                id,
                name,
                owner,
                description,
                recipients,
                icon,
                last_message_id,
                permissions,
                nsfw,
            } => Self::Group {
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
            Self::TextChannel {
                id,
                server,
                name,
                description,
                icon,
                last_message_id,
                default_permissions,
                role_permissions,
                nsfw,
            } => Self::TextChannel {
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
            Self::VoiceChannel {
                id,
                server,
                name,
                description,
                icon,
                default_permissions,
                role_permissions,
                nsfw,
            } => Self::VoiceChannel {
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
            FieldsMember::Nickname => Self {
                nickname: None,
                ..self
            },
            FieldsMember::Avatar => Self {
                avatar: None,
                ..self
            },
            FieldsMember::Roles => Self {
                roles: Vec::new(),
                ..self
            },
            FieldsMember::Timeout => Self {
                timeout: None,
                ..self
            },
        }
    }
}

impl Remove<FieldsRole> for Role {
    fn remove(self, field: &FieldsRole) -> Self {
        match field {
            FieldsRole::Colour => Self {
                colour: None,
                ..self
            },
        }
    }
}
