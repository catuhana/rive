use serde::Deserialize;

use crate::id::{
    marker::{StrikeMarker, UserMarker},
    Id,
};

#[derive(Deserialize, Debug, Clone)]
/// Account Strike
pub struct AccountStrike {
    /// Strike Id
    #[serde(rename = "_id")]
    pub id: Id<StrikeMarker>,

    /// Id of reported user
    pub user_id: Id<UserMarker>,

    /// Attached reason
    pub reason: String,
}
