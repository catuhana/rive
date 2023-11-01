//! User configuration for gateway.

use std::time::{SystemTime, UNIX_EPOCH};

use rive_models::{authentication::Authentication, event::Ping};

use crate::BASE_URL;

/// The default heartbeat function that returns current Unix timestamp.
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
    /// Authentication token.
    ///
    /// If it is not [`Authentication::None`] the authentication event will be sent automatically.
    pub auth: Authentication,
    /// Basic URL of Websocket API
    pub base_url: String,
    /// A function that generates the payload sent in a ping packet.
    ///
    /// If [`None`], heartbeat will not occur.
    pub heartbeat: Option<fn() -> Ping>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Creates a new [`Config`].
    ///
    /// The default [`heartbeat`] returns the big endian byte array presentation of
    /// current Unix timestamp in milliseconds as [`u128`].
    ///
    /// [`Config`]: crate::Config
    /// [`heartbeat`]: crate::Config::heartbeat
    pub fn new() -> Self {
        Self {
            auth: Authentication::None,
            base_url: BASE_URL.to_string(),
            heartbeat: Some(default_heartbeat_fn),
        }
    }
}
