#![doc = include_str!("../README.md")]

mod authentication;
mod bots;
mod channels;
mod customisation;
mod invites;
mod miscellaneous;
mod platform_administration;
mod revolt;
mod servers;
mod users;

use rive_models::{authentication::Authentication, error::ApiError};

type Result<T> = std::result::Result<T, Error>;

pub mod prelude {
    pub(crate) use crate::{ep, Client, RequestBuilderExt, ResponseExt, Result};
}

/// Base URL of the official Revolt instance API
pub const BASE_URL: &str = "https://api.revolt.chat";

/// Client error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Data serialization/deserialization error
    #[error("Serde JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP error
    #[error("Error while processing an HTTP request: {0}")]
    HttpRequest(#[from] reqwest::Error),

    /// An error returned from Revolt API
    #[error("Error returned from API: {0:#?}")]
    Api(ApiError),
}

macro_rules! ep {
    ($self:ident, $ep:literal, $($args:tt)*) => {
        format!(concat!("{}", $ep), $self.base_url, $($args)*)
    };

    ($self:ident, $ep:literal) => {
        format!(concat!("{}", $ep), $self.base_url)
    };

    (api_root = $api_root:expr, $ep:literal $($args:tt)*) => {
        format!(concat!("{}", $ep), $api_root, $($args)*)
    };
}
pub(crate) use ep;

trait RequestBuilderExt {
    fn auth(self, authentication: &Authentication) -> Self;
}

impl RequestBuilderExt for reqwest::RequestBuilder {
    fn auth(self, authentication: &Authentication) -> Self {
        self.header(authentication.header_key(), authentication.value())
    }
}

trait ResponseExt {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized;
}

impl ResponseExt for reqwest::Response {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized,
    {
        match self.status().as_u16() {
            200..=299 => Ok(self),
            // NOTE: it's a workaround thing but there are no alternative ways
            // because API returns some rocket's HTML instead of parseable JSON
            401 => Err(Error::Api(ApiError::Unauthenticated)),
            _ => Err(Error::Api(self.json().await?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
    authentication: Authentication,
}

impl Client {
    /// Create a client instance with the API base URL of Revolt official instance.
    pub fn new(authentication: Authentication) -> Self {
        Self::new_base_url(authentication, BASE_URL)
    }

    /// Create a client instance with given base URL.
    pub fn new_base_url(authentication: Authentication, base_url: impl ToString) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::builder()
                .user_agent("rive-http")
                .build()
                .unwrap(),
            authentication,
        }
    }
}
