#![doc = include_str!("../README.md")]

use std::sync::Arc;

use rive_cache_inmemory::InMemoryCache;
use rive_gateway::Gateway;
use rive_models::{authentication::Authentication, event::ServerEvent};

/// Revolt entities
///
/// A re-export of [`rive_models`].
pub mod models {
    pub use rive_models::*;
}

/// HTTP client for the Revolt REST API
///
/// A re-export of [`rive_http`].
pub mod http {
    pub use rive_http::*;
}

/// Client for the Revolt WebSocket API
///
/// A re-export of [`rive_gateway`].
pub mod gateway {
    pub use rive_gateway::*;
}

/// Client for the Autumn (Revolt file storage) API.
///
/// A re-export of [`rive_autumn`].
pub mod autumn {
    pub use rive_autumn::*;
}

/// In-memory cache.
///
/// A re-export of [`rive_cache_inmemory`].
pub mod cache_inmemory {
    pub use rive_cache_inmemory::*;
}

/// Re-export of everything
pub mod prelude {
    // TODO: remove re-export of "*" (breaking change!)
    pub use rive_models::{
        account::*, attachment::*, authentication::*, bot::*, channel::*, core::*, data::*,
        embed::*, emoji::*, error::*, event::*, invite::*, member::*, message::*, mfa::*,
        onboarding::*, permission::*, report::*, server::*, session::*, snapshot::*, stats::*,
        strike::*, user::*, voice::*, webhook::*, *,
    };

    pub use rive_autumn::{
        Client as AutumnClient, Error as AutumnError, BASE_URL as AUTUMN_BASE_URL,
    };
    pub use rive_cache_inmemory::{
        Config as InMemoryCacheConfig, InMemoryCache, InMemoryCacheBuilder, InMemoryCacheIter,
        InMemoryCacheStats, IterReference, Reference, ResourceIter,
    };
    pub use rive_gateway::{
        Error as GatewayError, Gateway, GatewayConfig, BASE_URL as GATEWAY_BASE_URL,
    };
    pub use rive_http::{Client, Error as HttpError, BASE_URL as HTTP_BASE_URL};

    pub use crate::Rive;

    pub use futures::StreamExt;
}

#[derive(Debug, Clone)]
pub struct Rive {
    pub http: rive_http::Client,
    pub gateway: Gateway,
    pub autumn: rive_autumn::Client,
    pub cache: Arc<InMemoryCache>,
}

impl Rive {
    /// Creates a new [`Rive`].
    // TODO: make a separated error struct instead of gateway error exclusively?
    // i mean that's kinda crappy isn't it? ------------>  VVVVVVVVVVVVVVVVVVV
    pub async fn new(auth: Authentication) -> Result<Self, rive_gateway::Error> {
        let http = rive_http::Client::new(auth.clone());
        let gateway = Gateway::connect(auth).await?;
        let autumn = rive_autumn::Client::new();
        let cache = Arc::new(InMemoryCache::new());

        Ok(Self {
            http,
            gateway,
            autumn,
            cache,
        })
    }

    /// Handle an incoming event.
    pub fn update(&self, event: &ServerEvent) {
        self.cache.update(event);
    }
}
