//! Errors returned by API operations.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Type of [`ReceiveError`] that occured.
///
/// [`ReceiveError`]: crate::error::ReceiveError
#[derive(Debug)]
#[non_exhaustive]
pub enum ReceiveErrorKind {
    /// WebSocket connection error.
    Io,
    /// Failed to connect to the API.
    Reconnect,
    /// Gateway event could not be deserialized.
    Deserialize,
    /// Message could not be sent over the Websocket connection.
    ///
    /// This may happen when the shard sends heartbeats or attempts to authenticate.
    SendMessage,
}

/// Receiving the next Websocket message failed.
#[derive(Debug)]
pub struct ReceiveError {
    /// Type of error.
    pub(crate) kind: ReceiveErrorKind,
    /// Source error, if any.
    pub(crate) source: Option<Box<dyn Error + Sync + Send>>,
}

impl ReceiveError {
    /// Create new [`ReceiveError`] with given error type and source.
    ///
    /// [`ReceiveError`]: crate::error::ReceiveError
    pub(crate) fn new(
        kind: ReceiveErrorKind,
        source: Option<Box<dyn Error + Sync + Send>>,
    ) -> Self {
        Self { kind, source }
    }

    /// Create new [`ReceiveError`] from [`SendError`].
    ///
    /// The type of error is [`ReceiveErrorKind::SendMessage`].
    ///
    /// [`ReceiveError`]: crate::error::ReceiveError
    /// [`SendError`]: crate::error::SendError
    /// [`ReceiveErrorKind::SendMessage`]: crate::error::ReceiveErrorKind::SendMessage
    pub(crate) fn from_send(error: SendError) -> Self {
        Self::new(ReceiveErrorKind::SendMessage, error.source)
    }

    /// An immutable reference to the type of error that occurred.
    #[must_use]
    pub fn kind(&self) -> &ReceiveErrorKind {
        &self.kind
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use]
    pub fn into_parts(self) -> (ReceiveErrorKind, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for ReceiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            ReceiveErrorKind::Io => f.write_str("websocket connection error"),
            ReceiveErrorKind::Reconnect => f.write_str("failed to reconnect to the gateway"),
            ReceiveErrorKind::Deserialize => f.write_str("gateway event could not be deserialized"),
            ReceiveErrorKind::SendMessage => {
                f.write_str("failed to send a message over the websocket")
            }
        }
    }
}

impl Error for ReceiveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`SendError`] that occured.
///
/// [`SendError`]: crate::error::SendError
#[derive(Debug)]
#[non_exhaustive]
pub enum SendErrorKind {
    /// Sending the payload over the WebSocket failed.
    ///
    /// This is indicative of a shutdown shard.
    Send,
    /// Serializing the payload failed.
    Serialize,
}

/// Sending the next Websocket message failed.
#[derive(Debug)]
pub struct SendError {
    /// Type of error.
    pub(crate) kind: SendErrorKind,
    /// Source error, if any.
    pub(crate) source: Option<Box<dyn Error + Sync + Send>>,
}

impl SendError {
    /// Create new [`SendError`] with given error type and source.
    ///
    /// [`SendError`]: crate::error::SendError
    pub(crate) fn new(kind: SendErrorKind, source: Option<Box<dyn Error + Sync + Send>>) -> Self {
        Self { kind, source }
    }

    /// An immutable reference to the type of error that occurred.
    #[must_use]
    pub fn kind(&self) -> &SendErrorKind {
        &self.kind
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use]
    pub fn into_parts(self) -> (SendErrorKind, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for SendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            SendErrorKind::Serialize => f.write_str("client event could not be serialized"),
            SendErrorKind::Send => f.write_str("failed to send a message over the websocket"),
        }
    }
}

impl Error for SendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}
