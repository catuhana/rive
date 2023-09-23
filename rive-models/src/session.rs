use crate::{
    id::{
        marker::{SessionMarker, UserMarker},
        Id,
    },
    mfa::MFAMethod,
};
use serde::Deserialize;

/// Web Push subscription
#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schemas", derive(JsonSchema))]
pub struct WebPushSubscription {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

/// Session information
#[derive(Deserialize, Debug, Clone)]
pub struct Session {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Id<SessionMarker>,
    /// User Id
    pub user_id: Id<UserMarker>,
    /// Session token
    pub token: String,
    /// Display name
    pub name: String,
    /// Web Push subscription
    pub subscription: Option<WebPushSubscription>,
}

/// Partial session information
#[derive(Deserialize, Debug, Clone)]
pub struct SessionInfo {
    #[serde(rename = "_id")]
    pub id: Id<SessionMarker>,
    pub name: String,
}

/// Login response
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "result")]
pub enum LoginResponse {
    Success(Session),
    MFA {
        ticket: String,
        allowed_methods: Vec<MFAMethod>,
    },
    Disabled {
        user_id: Id<UserMarker>,
    },
}
