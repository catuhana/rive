#![doc = include_str!("../README.md")]

use rive_gateway::Gateway;
use rive_http::Client;
use rive_models::authentication::Authentication;

/// Revolt entities
///
/// A re-export of [rive_models].
pub mod models {
    pub use rive_models::*;
}

/// HTTP client for the Revolt REST API
///
/// A re-export of [rive_http].
pub mod http {
    pub use rive_http::*;
}

/// Client for the Revolt WebSocket API
///
/// A re-export of [rive_gateway].
pub mod gateway {
    pub use rive_gateway::*;
}

/// Re-export of everything
pub mod prelude {
    pub use rive_models::{
        account::*, attachment::*, authentication::*, bot::*, channel::*, core::*, data::*,
        embed::*, emoji::*, event::*, invite::*, member::*, message::*, mfa::*, onboarding::*,
        permission::*, report::*, server::*, session::*, snapshot::*, stats::*, strike::*, user::*,
        voice::*, webhook::*, *,
    };

    pub use rive_gateway::{Error as GatewayError, Gateway, GatewayConfig};
    pub use rive_http::{Client, Error as HttpError};

    pub use crate::Rive;
    pub use futures::StreamExt;
}

#[derive(Debug, Clone)]
pub struct Rive {
    pub http: Client,
    pub gateway: Gateway,
}

impl Rive {
    /// Creates a new [`Rive`].
    // TODO: make a separated error struct instead of gateway error exclusively?
    // i mean that's kinda crappy isn't it? ------------>  VVVVVVVVVVVVVVVVVVV
    pub async fn new(auth: Authentication) -> Result<Self, rive_gateway::Error> {
        let http = Client::new(auth.clone());
        let gateway = Gateway::connect(auth).await?;

        Ok(Self { http, gateway })
    }
}
