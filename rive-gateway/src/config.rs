use std::time::{SystemTime, UNIX_EPOCH};

use rive_models::{authentication::Authentication, event::Ping};

use crate::BASE_URL;

fn default_heartbeat_fn() -> Ping {
    Ping::Binary(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock may have gone backwards")
            .as_millis()
            .to_be_bytes()
            .to_vec(),
    )
}

/// Gateway configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Auth token. If it is not [`Authentication::None`] then the event will be sent automatically.
    pub auth: Authentication,
    /// WebSocket API base URL
    pub base_url: String,
    pub heartbeat: Option<HeartbeatFn>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Creates a new [`Config`].
    pub fn new() -> Self {
        Self {
            auth: Authentication::None,
            base_url: BASE_URL.to_string(),
            heartbeat: Some(default_heartbeat_fn),
        }
    }
}
