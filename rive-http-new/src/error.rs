pub use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    BuildingRequest,
    SendingRequest,
    Api,
}

#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) source: Option<Box<dyn StdError + Sync + Send>>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, source: Option<Box<dyn StdError + Sync + Send>>) -> Self {
        Self { kind, source }
    }

    #[must_use]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    #[must_use]
    pub fn into_parts(self) -> (ErrorKind, Option<Box<dyn StdError + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::BuildingRequest => f.write_str("request building failed"),
            ErrorKind::SendingRequest => f.write_str("error while sending request"),
            ErrorKind::Api => f.write_str("API returned an error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn StdError + 'static))
    }
}
