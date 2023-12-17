use hyper::body::Bytes;

use crate::{base::request::builder::RequestBuilder, Result};

#[derive(Debug)]
pub struct Request {
    pub(crate) method: &'static str,
    pub(crate) path: String,
    pub(crate) body: Bytes,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }
}
pub trait TryIntoRequest {
    fn try_into_request(self) -> Result<Request>;
}
