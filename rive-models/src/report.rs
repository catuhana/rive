use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{MessageMarker, ReportMarker, ServerMarker, UserMarker},
    Id,
};

/// Reason for reporting content (message or server)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentReportReason {
    /// No reason has been specified
    NoneSpecified,

    /// Blatantly illegal content
    Illegal,

    /// Content that promotes harm to others / self
    PromotesHarm,

    /// Spam or platform abuse
    SpamAbuse,

    /// Distribution of malware
    Malware,

    /// Harassment or abuse targeted at another user
    Harassment,
}

/// Reason for reporting a user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserReportReason {
    /// No reason has been specified
    NoneSpecified,

    /// User is sending spam or otherwise abusing the platform
    SpamAbuse,

    /// User's profile contains inappropriate content for a general audience
    InappropriateProfile,

    /// User is impersonating another user
    Impersonation,

    /// User is evading a ban
    BanEvasion,

    /// User is not of minimum age to use the platform
    Underage,
}

/// The content being reported
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ReportedContent {
    /// Report a message
    Message {
        /// ID of the message
        id: Id<MessageMarker>,
        /// Reason for reporting message
        report_reason: ContentReportReason,
    },
    /// Report a server
    Server {
        /// ID of the server
        id: Id<ServerMarker>,
        /// Reason for reporting server
        report_reason: ContentReportReason,
    },
    /// Report a user
    User {
        /// ID of the user
        id: Id<UserMarker>,
        /// Reason for reporting a user
        report_reason: UserReportReason,
    },
}

/// Status of the report
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum ReportStatus {
    /// Report is waiting for triage / action
    Created,

    /// Report was rejected
    Rejected { rejection_reason: String },

    /// Report was actioned and resolved
    Resolved,
}

/// User-generated platform moderation report.
#[derive(Deserialize, Debug, Clone)]
pub struct Report {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: Id<ReportMarker>,
    /// Id of the user creating this report
    pub author_id: Id<UserMarker>,
    /// Reported content
    pub content: ReportedContent,
    /// Additional report context
    pub additional_context: String,
    /// Status of the report
    #[serde(flatten)]
    pub status: ReportStatus,
    /// Additional notes included on the report
    #[serde(default)]
    pub notes: String,
}
