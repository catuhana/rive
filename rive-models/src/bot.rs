use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    attachment::Attachment,
    id::{marker::UserMarker, Id},
    user::User,
};

bitflags::bitflags! {
    /// User badge bitfield
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BotFlags: u64 {
        const Verified = 1;
        const Official = 2;
    }
}

impl Serialize for BotFlags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for BotFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

/// Public bot
#[derive(Deserialize, Debug, Clone)]
pub struct PublicBot {
    /// Bot Id
    #[serde(rename = "_id")]
    pub id: Id<UserMarker>,
    /// Bot Username
    pub username: String,
    /// Profile Avatar
    pub avatar: Option<Attachment>,
    /// Profile Description
    pub description: Option<String>,
}

/// Representation of a bot on Revolt
#[derive(Deserialize, Debug, Clone)]
pub struct Bot {
    /// Bot Id
    ///
    /// This equals the associated bot user's id.
    #[serde(rename = "_id")]
    pub id: Id<UserMarker>,
    /// User Id of the bot owner
    pub owner: Id<UserMarker>,
    /// Token used to authenticate requests for this bot
    pub token: String,
    /// Whether the bot is public
    /// (may be invited by anyone)
    pub public: bool,

    /// Whether to enable analytics
    #[serde(default)]
    pub analytics: bool,
    /// Whether this bot should be publicly discoverable
    #[serde(default)]
    pub discoverable: bool,
    /// Reserved; URL for handling interactions
    pub interactions_url: Option<String>,
    /// URL for terms of service
    pub terms_of_service_url: Option<String>,
    /// URL for privacy policy
    pub privacy_policy_url: Option<String>,

    /// Enum of bot flags
    pub flags: Option<BotFlags>,
}

/// Partial representation of a bot on Revolt
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct PartialBot {
    /// Bot Id
    ///
    /// This equals the associated bot user's id.
    #[serde(rename = "_id")]
    pub id: Option<Id<UserMarker>>,
    /// User Id of the bot owner
    pub owner: Option<Id<UserMarker>>,
    /// Token used to authenticate requests for this bot
    pub token: Option<String>,
    /// Whether the bot is public
    /// (may be invited by anyone)
    pub public: Option<bool>,

    /// Whether to enable analytics
    pub analytics: Option<bool>,
    /// Whether this bot should be publicly discoverable
    pub discoverable: Option<bool>,
    /// Reserved; URL for handling interactions
    pub interactions_url: Option<String>,
    /// URL for terms of service
    pub terms_of_service_url: Option<String>,
    /// URL for privacy policy
    pub privacy_policy_url: Option<String>,

    /// Enum of bot flags
    pub flags: Option<BotFlags>,
}

/// Owned bot.
///
/// Contains bot and user information.
#[derive(Deserialize, Debug, Clone)]
pub struct OwnedBot {
    /// Bot object
    pub bot: Bot,
    /// User object
    pub user: User,
}

/// Optional fields on bot object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsBot {
    Token,
    InteractionsURL,
}

/// Owned bots.
///
/// Both lists are sorted by their IDs.
#[derive(Deserialize, Debug, Clone)]
pub struct OwnedBots {
    /// Bot objects
    pub bots: Vec<Bot>,
    /// User objects
    pub users: Vec<User>,
}
