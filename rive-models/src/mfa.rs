use serde::Deserialize;

/// Multi-factor auth ticket
#[derive(Deserialize, Debug, Clone)]
pub struct MFATicket {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,

    /// Account Id
    pub account_id: String,

    /// Unique Token
    pub token: String,

    /// Whether this ticket has been validated
    /// (can be used for account actions)
    pub validated: bool,

    /// Whether this ticket is authorised
    /// (can be used to log a user in)
    pub authorised: bool,

    /// TOTP code at time of ticket creation
    pub last_totp_code: Option<String>,
}
