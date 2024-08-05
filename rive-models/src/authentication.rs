#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Authentication {
    SessionToken(String),
    BotToken(String),
    ValidMfaTicket(String),
    UnvalidatedMfaTicket(String),
    None,
}

impl Authentication {
    pub fn header_key(&self) -> String {
        match self {
            Self::SessionToken(_) => "x-session-token",
            Self::BotToken(_) => "x-bot-token",
            Self::ValidMfaTicket(_) => "x-mfa-ticket",
            Self::UnvalidatedMfaTicket(_) => "x-mfa-ticket",
            Self::None => "",
        }
        .to_string()
    }

    pub fn value(&self) -> String {
        match self {
            Self::SessionToken(t) => t,
            Self::BotToken(t) => t,
            Self::ValidMfaTicket(t) => t,
            Self::UnvalidatedMfaTicket(t) => t,
            Self::None => "",
        }
        .to_string()
    }
}
