use rive_models::user::{FieldsUser, User, UserProfile, UserStatus};

// TODO: maybe move this to rive_models?
pub trait Remove<T> {
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
