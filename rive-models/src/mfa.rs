use serde::{Deserialize, Serialize};

/// MFA request/response data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MFAData {
    Password { password: String },
    Recovery { recovery_code: String },
    Totp { totp_code: String },
}

/// MFA method
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum MFAMethod {
    Password,
    Recovery,
    Totp,
}

/// MFA recovery code
pub type MFARecoveryCode = String;

/// MFA status
#[derive(Deserialize, Debug, Clone)]
pub struct MFAStatus {
    pub email_otp: bool,
    pub trusted_handover: bool,
    pub email_mfa: bool,
    pub totp_mfa: bool,
    pub security_key_mfa: bool,
    pub recovery_active: bool,
}

/// TOTP secret response
#[derive(Deserialize, Debug, Clone)]
pub struct TOTPSecret {
    pub secret: String,
}

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
