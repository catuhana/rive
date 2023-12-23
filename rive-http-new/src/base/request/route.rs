use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum Route<'a> {
    SendMessage { channel_id: &'a str },
}

impl<'a> Route<'a> {
    pub fn method(&self) -> &'static str {
        match self {
            Route::SendMessage { .. } => "POST",
        }
    }
}

impl<'a> fmt::Display for Route<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Route::SendMessage { channel_id } => {
                f.write_str("/channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages")
            }
        }
    }
}
