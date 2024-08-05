use hyper::body::Bytes;
use serde::Serialize;

use crate::{
    base::request::{Request, Route},
    error::{Error, ErrorKind},
    Result,
};

#[derive(Debug)]
pub struct RequestBuilder {
    inner: Result<Request>,
}

impl RequestBuilder {
    pub const fn new() -> Self {
        Self {
            inner: Ok(Request {
                method: "GET",
                path: String::new(),
                body: Bytes::new(),
            }),
        }
    }

    pub fn from_route(route: Route) -> Self {
        Self::new().route(route)
    }

    pub fn route(self, route: Route) -> Self {
        self.and_then(move |mut request| {
            request.method = route.method();
            request.path = route.to_string();
            Ok(request)
        })
    }

    pub fn json<T>(self, object: &T) -> Self
    where
        T: Serialize,
    {
        self.and_then(move |mut request| {
            let vec = serde_json::to_vec(object)
                .map_err(|source| Error::new(ErrorKind::BuildingRequest, Some(Box::new(source))))?;
            let bytes = Bytes::from(vec);
            request.body = bytes;
            Ok(request)
        })
    }

    pub fn build(self) -> Result<Request> {
        self.inner
    }

    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Request) -> Result<Request>,
    {
        Self {
            inner: self.inner.and_then(func),
        }
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
