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

use rive_models::{authentication::Authentication, ApiError};

type Result<T> = std::result::Result<T, Error>;

pub(crate) mod prelude {
    pub(crate) use crate::{ep, Client, RequestBuilderExt, ResponseExt, Result};
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serde JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Error while processing an HTTP request: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Error returned from API")]
    Api(ApiError),
}

#[macro_export]
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

trait RequestBuilderExt {
    fn auth(self, authentication: &Authentication) -> Self;
}

impl RequestBuilderExt for reqwest::RequestBuilder {
    fn auth(self, authentication: &Authentication) -> Self {
        self.header(authentication.header_key(), authentication.value())
    }
}

#[async_trait::async_trait]
trait ResponseExt {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
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
    pub fn new(authentication: Authentication) -> Self {
        Client::new_base_url(authentication, "https://api.revolt.chat")
    }

    pub fn new_base_url(authentication: Authentication, base_url: impl Into<String>) -> Self {
        Client {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
            authentication,
        }
    }
}
