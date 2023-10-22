use rive_models::authentication::Authentication;

use crate::{Config, Gateway, HeartbeatFn};

/// Builder to configure and construct a [`Gateway`].
///
/// [`Gateway`]: crate::Gateway
#[derive(Debug, Default)]
#[must_use]
pub struct GatewayBuilder(Config);

impl GatewayBuilder {
    pub fn new() -> Self {
        Self(Config::new())
    }

    pub fn auth(mut self, auth: Authentication) -> GatewayBuilder {
        self.0.auth = auth;
        self
    }

    pub fn base_url(mut self, base_url: String) -> GatewayBuilder {
        self.0.base_url = base_url;
        self
    }

    pub fn heartbeat_fn(mut self, heartbeat: Option<HeartbeatFn>) -> GatewayBuilder {
        self.0.heartbeat = heartbeat;
        self
    }

    pub fn build(self) -> Gateway {
        Gateway::with_config(self.0)
    }
}
