use std::time::{SystemTime, UNIX_EPOCH};

use rive_models::authentication::Authentication;

use crate::BASE_URL;

fn default_heartbeat_fn() -> i32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32
}

/// Gateway configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Auth token. If it is not [`Authentication::None`] then the event will be sent automatically.
    pub auth: Authentication,
    /// WebSocket API base URL
    pub base_url: String,
    pub heartbeat: Option<fn() -> i32>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Creates a new [`GatewayConfig`].
    pub fn new() -> Self {
        Self {
            auth: Authentication::None,
            base_url: BASE_URL.to_string(),
            heartbeat: Some(default_heartbeat_fn),
        }
    }
}
