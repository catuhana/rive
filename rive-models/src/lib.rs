#![doc = include_str!("../README.md")]

pub mod account;
pub mod attachment;
pub mod authentication;
pub mod autumn;
pub mod bot;
pub mod channel;
pub mod core;
pub mod data;
pub mod embed;
pub mod emoji;
pub mod error;
pub mod event;
pub mod invite;
pub mod member;
pub mod message;
pub mod mfa;
pub mod onboarding;
pub mod permission;
pub mod report;
pub mod server;
pub mod session;
pub mod snapshot;
pub mod stats;
pub mod strike;
pub mod user;
pub mod voice;
pub mod webhook;

#[deprecated = "Please import `rive_models::error::ApiError` instead"]
pub use error::ApiError;

macro_rules! impl_serde_bitflags(
    ($type:ident) => {
        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u64(self.bits())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
            }
        }
    }
);
pub(crate) use impl_serde_bitflags;
