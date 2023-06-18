use serde::{Deserialize, Serialize};

use crate::attachment::Attachment;

/// Webhook information
#[derive(Deserialize, Debug, Clone)]
pub struct Webhook {
    /// Webhook ID
    pub id: String,

    /// The name of the webhook
    pub name: String,

    /// The avatar of the webhook
    pub avatar: Option<Attachment>,

    /// The channel this webhook belongs to
    pub channel_id: String,

    /// The private token for the webhook
    pub token: Option<String>,
}

/// Partial webhook data
#[derive(Deserialize, Debug, Clone)]
pub struct PartialWebhook {
    /// The name of the webhook
    pub name: Option<String>,

    /// The avatar of the webhook
    pub avatar: Option<Attachment>,

    /// The channel this webhook belongs to
    pub channel_id: Option<String>,

    /// The private token for the webhook
    pub token: Option<String>,
}

/// Optional fields on webhook object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldsWebhook {
    Avatar,
}
