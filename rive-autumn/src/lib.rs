#![doc = include_str!("../README.md")]

use std::borrow::Cow;

use reqwest::multipart::{Form, Part};
use rive_models::{autumn::UploadData, error::AutumnError};

/// Revolt official instance base URL
pub const BASE_URL: &str = "https://autumn.revolt.chat";

type Result<T> = std::result::Result<T, Error>;

/// Client error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Data serialization/deserialization error
    #[error("Serde JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP error
    #[error("Error while processing an HTTP request: {0}")]
    HttpRequest(#[from] reqwest::Error),

    /// An error returned from Autumn API
    #[error("Error returned from API")]
    Api(AutumnError),
}

/// A wrapper for Autumn API
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Create a client with Revolt official instance base URL.
    pub fn new() -> Self {
        Client::new_base_url(BASE_URL)
    }

    /// Create a client instance with given base URL.
    pub fn new_base_url(base_url: impl Into<String>) -> Self {
        Client {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Download an attachment by its tag and ID.
    pub async fn download(&self, tag: impl Into<String>, id: impl Into<String>) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(format!("{}/{}/{}", self.base_url, tag.into(), id.into()))
            .send()
            .await?;

        match response.status().as_u16() {
            200..=299 => Ok(response.bytes().await?.to_vec()),
            _ => Err(Error::Api(response.json().await?)),
        }
    }

    /// Upload an attachment.
    pub async fn upload(
        &self,
        tag: impl Into<String>,
        filename: impl Into<String>,
        contents: impl Into<Cow<'static, [u8]>>,
    ) -> Result<UploadData> {
        let part = Part::bytes(contents).file_name(filename.into());
        let form = Form::new().part("file", part);

        let response = self
            .client
            .post(format!("{}/{}", self.base_url, tag.into()))
            .multipart(form)
            .send()
            .await?;

        match response.status().as_u16() {
            200..=299 => Ok(response.json().await?),
            _ => Err(Error::Api(response.json().await?)),
        }
    }
}
