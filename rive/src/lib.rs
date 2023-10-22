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
///
/// Some fields and constants are renamed as they have the same name:
///
/// | Original field name             | Renamed field name      |
/// |---------------------------------|-------------------------|
/// | [`rive_autumn::BASE_URL`]       | [`AUTUMN_BASE_URL`]     |
/// | [`rive_autumn::Client`]         | [`AutumnClient`]        |
/// | [`rive_autumn::Error`]          | [`AutumnError`]         |
/// | [`rive_cache_inmemory::Config`] | [`InMemoryCacheConfig`] |
/// | [`rive_gateway::BASE_URL`]      | [`GATEWAY_BASE_URL`]    |
/// | [`rive_http::BASE_URL`]         | [`HTTP_BASE_URL`]       |
/// | [`rive_http::Error`]            | [`HttpError`]           |
///
/// [`AUTUMN_BASE_URL`]: crate::prelude::AUTUMN_BASE_URL
/// [`AutumnClient`]: crate::prelude::AutumnClient
/// [`AutumnError`]: crate::prelude::AutumnError
/// [`InMemoryCacheConfig`]: crate::prelude::InMemoryCacheConfig
/// [`GATEWAY_BASE_URL`]: crate::prelude::GATEWAY_BASE_URL
/// [`HTTP_BASE_URL`]: crate::prelude::HTTP_BASE_URL
/// [`HttpError`]: crate::prelude::HttpError
pub mod prelude {
    pub use rive_models::{
        account::*, attachment::*, authentication::*, bot::*, channel::*, core::*, data::*,
        embed::*, emoji::*, error::*, event::*, invite::*, member::*, message::*, mfa::*,
        onboarding::*, permission::*, report::*, server::*, session::*, snapshot::*, stats::*,
        strike::*, user::*, voice::*, webhook::*,
    };

    pub use rive_autumn::{
        Client as AutumnClient, Error as AutumnError, BASE_URL as AUTUMN_BASE_URL,
    };
    pub use rive_cache_inmemory::{
        Config as InMemoryCacheConfig, InMemoryCache, InMemoryCacheBuilder, InMemoryCacheIter,
        InMemoryCacheStats, IterReference, Reference, ResourceIter,
    };
    pub use rive_gateway::{error::*, Config, Gateway, BASE_URL as GATEWAY_BASE_URL};
    pub use rive_http::{Client, Error as HttpError, BASE_URL as HTTP_BASE_URL};

    pub use crate::Rive;

    pub use futures::StreamExt;
}

#[derive(Debug, Clone)]
pub struct Rive {
    pub http: rive_http::Client,
    pub gateway: Arc<Gateway>,
    pub autumn: rive_autumn::Client,
    pub cache: Arc<InMemoryCache>,
}

impl Rive {
    /// Creates a new [`Rive`].
    // TODO: make a separated error struct instead of gateway error exclusively?
    // i mean that's kinda crappy isn't it? ------------>  VVVVVVVVVVVVVVVVVVV
    pub async fn new(auth: Authentication) -> Self {
        let http = rive_http::Client::new(auth.clone());
        let gateway = Arc::new(Gateway::new(auth));
        let autumn = rive_autumn::Client::new();
        let cache = Arc::new(InMemoryCache::new());

        Self {
            http,
            gateway,
            autumn,
            cache,
        }
    }

    /// Handle an incoming event.
    pub fn update(&self, event: &ServerEvent) {
        self.cache.update(event);
    }
}
