use serde::Deserialize;

use crate::{
    id::{marker::AccountMarker, Id},
    mfa::MFATicket,
};

/// Account information
#[derive(Deserialize, Debug, Clone)]
pub struct AccountInfo {
    /// Account ID
    #[serde(rename = "_id")]
    pub id: Id<AccountMarker>,
    /// Linked email
    pub email: String,
}

/// Email verification response
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EmailVerification {
    NoTicket,
    WithTicket {
        /// Authorised MFA ticket, can be used to log in
        ticket: MFATicket,
    },
}
