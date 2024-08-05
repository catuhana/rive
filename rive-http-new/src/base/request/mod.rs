mod builder;
mod route;

pub use builder::RequestBuilder;
pub use route::Route;

use crate::Result;
use hyper::body::Bytes;

#[derive(Debug)]
pub struct Request {
    pub(crate) method: &'static str,
    pub(crate) path: String,
    pub(crate) body: Bytes,
}

impl Request {
    pub const fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }
}

pub trait TryIntoRequest: Send {
    fn try_into_request(self) -> Result<Request>;
}
