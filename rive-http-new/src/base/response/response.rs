use http_body_util::BodyExt as _;
use hyper::body::Incoming;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Response {
    pub(crate) inner: http::Response<Incoming>,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.inner.status().as_u16()
    }

    pub async fn json<T>(self) -> T
    where
        T: DeserializeOwned,
    {
        let bytes = self.inner.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }
}

impl From<http::Response<Incoming>> for Response {
    fn from(value: http::Response<Incoming>) -> Self {
        Self { inner: value }
    }
}
