use crate::{attachment::Attachment, channel::Channel, server::Server};
use serde::Deserialize;

/// Invite
#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Invite {
    /// Server channel invite
    Server {
        /// Invite code
        code: String,
        /// Id of the server
        server_id: String,
        /// Name of the server
        server_name: String,
        /// Attachment for server icon
        server_icon: Option<Attachment>,
        /// Attachment for server banner
        server_banner: Option<Attachment>,
        /// Enum of server flags
        server_flags: Option<i32>,
        /// Id of server channel
        channel_id: String,
        /// Name of server channel
        channel_name: String,
        /// Description of server channel
        channel_description: Option<String>,
        /// Name of user who created the invite
        user_name: String,
        /// Avatar of the user who created the invite
        user_avatar: Option<Attachment>,
        /// Number of members in this server
        member_count: i64,
    },
    /// Group channel invite
    Group {
        /// Invite code
        code: String,
        /// Id of group channel
        channel_id: String,
        /// Name of group channel
        channel_name: String,
        /// Description of group channel
        channel_description: Option<String>,
        /// Name of user who created the invite
        user_name: String,
        /// Avatar of the user who created the invite
        user_avatar: Option<Attachment>,
    },
}

/// Invite join response
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InviteJoin {
    Server {
        /// Channels in the server
        channels: Vec<Channel>,
        /// Server we are joining
        server: Server,
    },
}
