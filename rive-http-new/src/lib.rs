pub mod base;
pub mod error;
pub mod request;

mod client;
mod connector;
pub use client::Client;

mod config;
pub use config::Config;

use error::Error;
use std::{future::Future, pin::Pin, result::Result as StdResult};

pub(crate) type Result<T> = StdResult<T, Error>;
pub(crate) type ResponseFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Sync + Send + 'a>>;

pub const BASE_URL: &str = "https://api.revolt.chat";
