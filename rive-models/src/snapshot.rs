use serde::Deserialize;

use crate::{
    channel::Channel,
    id::{
        marker::{ReportMarker, SnapshotMarker},
        Id,
    },
    message::Message,
    server::Server,
    user::User,
};

/// Enum to map into different models
/// that can be saved in a snapshot
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "_type")]
pub enum SnapshotContent {
    Message {
        /// Context before the message
        #[serde(rename = "_prior_context", default)]
        prior_context: Vec<Message>,

        /// Context after the message
        #[serde(rename = "_leading_context", default)]
        leading_context: Vec<Message>,

        /// Message
        #[serde(flatten)]
        message: Box<Message>,
    },
    Server(Server),
    User(User),
}

/// Snapshot of some content
#[derive(Deserialize, Debug, Clone)]
pub struct Snapshot {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Id<SnapshotMarker>,
    /// Report parent Id
    pub report_id: Id<ReportMarker>,
    /// Snapshot of content
    pub content: SnapshotContent,
}

/// Snapshot of some content with required data to render
#[derive(Deserialize, Debug, Clone)]
pub struct SnapshotWithContext {
    /// Snapshot itself
    #[serde(flatten)]
    pub snapshot: Snapshot,
    /// Users involved in snapshot
    #[serde(rename = "_users")]
    pub users: Vec<User>,
    /// Channels involved in snapshot
    #[serde(rename = "_channels")]
    pub channels: Vec<Channel>,
    /// Server involved in snapshot
    #[serde(rename = "_server")]
    pub server: Option<Server>,
}
