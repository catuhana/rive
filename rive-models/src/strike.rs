use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// Account Strike
pub struct AccountStrike {
    /// Strike Id
    #[serde(rename = "_id")]
    pub id: String,

    /// Id of reported user
    pub user_id: String,

    /// Attached reason
    pub reason: String,
}
