#![doc = include_str!("../README.md")]

/// Revolt entities
///
/// A re-export of [rive_models].
pub mod models {
    pub use rive_models::*;
}

/// HTTP client for the Revolt REST API
///
/// A re-export of [rive_http].
pub mod http {
    pub use rive_http::*;
}

/// Client for the Revolt WebSocket API
///
/// A re-export of [rive_gateway].
pub mod gateway {
    pub use rive_gateway::*;
}
