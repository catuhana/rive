use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
#[non_exhaustive]
pub enum ReceiveErrorKind {
    Io,
    Reconnect,
    Deserialize,
    SendMessage,
}

#[derive(Debug)]
pub struct ReceiveError {
    pub kind: ReceiveErrorKind,
    pub source: Option<Box<dyn Error + Sync + Send>>,
}

impl ReceiveError {
    pub fn new(kind: ReceiveErrorKind, source: Option<Box<dyn Error + Sync + Send>>) -> Self {
        Self { kind, source }
    }

    pub fn from_send(error: SendError) -> Self {
        Self::new(ReceiveErrorKind::SendMessage, error.source)
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

#[derive(Debug)]
#[non_exhaustive]
pub enum SendErrorKind {
    Serialize,
    Send,
}

#[derive(Debug)]
pub struct SendError {
    pub kind: SendErrorKind,
    pub source: Option<Box<dyn std::error::Error + Sync + Send>>,
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

impl SendError {
    pub fn new(kind: SendErrorKind, source: Option<Box<dyn Error + Sync + Send>>) -> Self {
        Self { kind, source }
    }
}
