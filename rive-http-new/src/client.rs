use http_body_util::Full;
use hyper::body::Bytes;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client as HyperClient},
    rt::TokioExecutor,
};
use rive_models::{
    authentication::Authentication,
    id::{marker::ChannelMarker, Id},
};
use serde::de::DeserializeOwned;

use crate::{
    base::{request::TryIntoRequest, response::Response},
    error::{Error, ErrorKind},
    request::channels::messaging::SendMessageRequest,
    Config, Result, BASE_URL,
};

#[cfg(feature = "native-tls")]
type Connector = hyper_tls::HttpsConnector<HttpConnector>;

#[cfg(not(feature = "native-tls"))]
type Connector = HttpConnector;

#[derive(Debug)]
pub struct Client {
    config: Config,
    client: HyperClient<Connector, Full<Bytes>>,
}

impl Client {
    pub(crate) async fn execute(&self, request: impl TryIntoRequest) -> Result<Response> {
        let request = request.try_into_request()?;

        let request = http::Request::builder()
            .method(request.method)
            .uri(self.config.base_url.clone() + &request.path)
            .header(
                self.config.authentication.header_key(),
                self.config.authentication.value(),
            )
            .body::<Full<Bytes>>(request.body.into())
            .map_err(|source| Error {
                kind: ErrorKind::SendingRequest,
                source: Some(Box::new(source)),
            })?;

        let response = self.client.request(request).await.map_err(|source| Error {
            kind: ErrorKind::SendingRequest,
            source: Some(Box::new(source)),
        })?;

        Ok(response.into())
    }

    pub(crate) async fn fire<T>(&self, request: impl TryIntoRequest) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.execute(request).await?;

        if response.status() >= 200 && response.status() < 300 {
            let json = response.json::<T>().await;
            Ok(json)
        } else {
            Err(Error::new(ErrorKind::Api, None))
        }
    }

    // TODO: remove this attribute when it will be used
    #[allow(dead_code)]
    pub(crate) async fn wind(&self, request: impl TryIntoRequest) -> Result<()> {
        let response = self.execute(request).await?;

        if response.status() >= 200 && response.status() < 300 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Api, None))
        }
    }
}

impl Client {
    pub fn new(authentication: Authentication) -> Self {
        Self::with_config(Config {
            authentication,
            base_url: BASE_URL.to_string(),
        })
    }

    pub fn with_config(config: Config) -> Self {
        let connector = Connector::new();
        let executor = TokioExecutor::new();
        let client = HyperClient::builder(executor).build(connector);

        Self { config, client }
    }

    pub const fn send_message<'a>(
        &'a self,
        channel_id: &'a Id<ChannelMarker>,
    ) -> SendMessageRequest<'a> {
        SendMessageRequest::new(self, channel_id)
    }
}
