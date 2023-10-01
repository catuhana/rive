use serde::Deserialize;

use crate::permission::{Permission, UserPermission};

/// A representation of an API error.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum ApiError {
    LabelMe,

    // Onboarding related errors
    AlreadyOnboarded,

    // User related errors
    UsernameTaken,
    InvalidUsername,
    UnknownUser,
    AlreadyFriends,
    AlreadySentRequest,
    Blocked,
    BlockedByOther,
    NotFriends,

    // Channel related errors
    UnknownChannel,
    UnknownAttachment,
    UnknownMessage,
    CannotEditMessage,
    CannotJoinCall,
    TooManyAttachments,
    TooManyReplies,
    EmptyMessage,
    PayloadTooLarge,
    CannotRemoveYourself,
    GroupTooLarge { max: usize },
    AlreadyInGroup,
    NotInGroup,

    // Server related errors
    UnknownServer,
    InvalidRole,
    Banned,
    TooManyServers { max: usize },
    TooManyEmoji,

    // Bot related errors
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,

    // Permission errors
    MissingPermission { permission: Permission },
    MissingUserPermission { permission: UserPermission },
    NotElevated,
    CannotGiveMissingPermissions,
    NotOwner,

    // General errors
    DatabaseError { operation: String, with: String },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    InvalidSession,
    DuplicateNonce,
    VosoUnavailable,
    NotFound,
    NoEffect,
    FailedValidation,

    // Other errors that API does not return but it's still API related things
    Unauthenticated,
}

/// Revolt file storage API error
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(tag = "type")]
pub enum AutumnError {
    FileTooLarge { max_size: usize },
    FileTypeNotAllowed,
    FailedToReceive,
    BlockingError,
    DatabaseError,
    MissingData,
    UnknownTag,
    ProbeError,
    NotFound,
    Malware,
    IOError,
    S3Error,
    LabelMe,
}
