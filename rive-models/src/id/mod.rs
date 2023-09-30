//! ID with type-safe markers for each resource.
//!
//! When IDs are just Strings, it's easy to accidentally use, for example,
//! a user ID in a place where a server ID should be used. By using IDs
//! with typed tokens, this can be prevented at compile time.
//!
//! # Parsing
//! IDs can be created in one of the following ways:
//! - `serde` deserialization
//! - [`Id::new`]
//! - [`std::convert::From`]<[`String`]>
//!
//! # Casting between resource types
//!
//! In Revolt, several different resources can have the same ID. For
//! example, a user's private message channel ID has that user's ID.
//! For such cases, the IDs can be casted:
//!
//! ```
//! use rive_models::id::{
//!     marker::{ChannelMarker, UserMarker},
//!     Id,
//! };
//!
//! // Rust is often able to infer the type of an ID.
//! let user_id: Id<UserMarker> = Id::new("ABC".to_string());
//! let channel_id: Id<ChannelMarker> = user_id.clone().cast();
//!
//! assert_eq!(user_id.value(), channel_id.value());
//! ```

use std::{
    any,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub mod marker;

/// ID of a resource, such as the ID of a message or user.
///
// Markers themselves do not perform any logical action and are only used to
// ensure that identifiers of the wrong types are not used.
pub struct Id<T> {
    value: String,
    phantom: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    /// Create a new ID.
    ///
    /// This is mainly useful in case you are creating a hardcoded ID.
    pub const fn new(value: String) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }

    /// Cast an ID from one type to another.
    ///
    /// # Examples
    ///
    /// Cast a user ID to channel ID, useful for sending a DM.
    ///
    /// ```
    /// use rive_models::id::{
    ///     marker::{ChannelMarker, UserMarker},
    ///     Id,
    /// };
    ///
    /// let user_id: Id<UserMarker> = Id::new("ABC".to_string());
    /// let channel_id: Id<ChannelMarker> = user_id.clone().cast();
    ///
    /// assert_eq!(user_id.value(), channel_id.value());
    /// ```
    pub fn cast<New>(self) -> Id<New> {
        Id::new(self.value)
    }

    /// Return the owned inner value.
    pub fn value(self) -> String {
        self.value
    }

    /// Return a reference to the inner value.
    pub fn value_ref(&self) -> &str {
        &self.value
    }

    /// Return a mutable reference to the inner value.
    pub fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("Id")?;
        let type_name = any::type_name::<T>();

        // `any::type_name` will usually provide a fully qualified name,
        // so let's cut it out!
        if let Some(position) = type_name.rfind("::") {
            if let Some(slice) = type_name.get(position + 2..) {
                f.write_str("<")?;
                f.write_str(slice)?;
                f.write_str(">")?;
            }
        }

        f.write_str("(")?;
        Debug::fmt(&self.value, f)?;
        f.write_str(")")
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        other.value == self.value
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id::new(self.value.clone())
    }
}

impl<T> From<Id<T>> for String {
    fn from(value: Id<T>) -> Self {
        value.value
    }
}

impl<T> From<String> for Id<T> {
    fn from(value: String) -> Self {
        Id::new(value)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Id", &self.value)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IdVisitor<T> {
            phantom: PhantomData<T>,
        }

        impl<T> IdVisitor<T> {
            const fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for IdVisitor<T> {
            type Value = Id<T>;

            fn visit_newtype_struct<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                deserializer.deserialize_any(IdVisitor::new())
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Id::new(v.to_string()))
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Id::new(v))
            }

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }
        }

        deserializer.deserialize_newtype_struct("Id", IdVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use static_assertions::assert_impl_all;

    use super::{
        marker::{ChannelMarker, UserMarker},
        Id,
    };

    use std::{
        collections::hash_map::DefaultHasher,
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
    };

    assert_impl_all!(Id<()>: Clone, Debug, Deserialize<'static>, Display, Eq, From<String>,
           Hash, Into<String>, Ord, PartialEq, PartialEq, PartialOrd, Send, Serialize, Sync,
    );

    /// Test that various methods of initializing IDs are correct.
    #[test]
    fn initializers() {
        assert_eq!(
            "01FFD06NDVZ14W5T1WKKB4KKZX",
            Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string()).value()
        );
        assert_eq!(
            "01FFD06NDVZ14W5T1WKKB4KKZX",
            Id::<UserMarker>::from("01FFD06NDVZ14W5T1WKKB4KKZX".to_string()).value()
        );
    }

    /// Test that ID serializes and deserializes correctly.
    #[test]
    fn serde() {
        let id = Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string());

        assert_tokens(
            &id,
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::String("01FFD06NDVZ14W5T1WKKB4KKZX"),
            ],
        );

        assert_tokens(
            &id,
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("01FFD06NDVZ14W5T1WKKB4KKZX"),
            ],
        );
    }

    /// Test that debugging IDs formats the generic and value as a newtype.
    #[test]
    fn debug() {
        let id = Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string());
        assert_eq!(
            format!("{id:?}"),
            r#"Id<UserMarker>("01FFD06NDVZ14W5T1WKKB4KKZX")"#
        );
    }

    /// Test that display formatting an ID formats the value.
    #[test]
    fn display() {
        let id = Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string());
        assert_eq!(format!("{id}"), "01FFD06NDVZ14W5T1WKKB4KKZX");
    }

    /// Test that ID casting maintains the value.
    #[test]
    fn cast() {
        let id = Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string());
        let casted_id = id.clone().cast::<ChannelMarker>();

        assert_eq!(id.value(), casted_id.value());
    }

    /// Test that hashing an ID is equivalent to hashing only its inner value.
    #[test]
    fn hash() {
        let id = Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string());

        let mut id_hasher = DefaultHasher::new();
        id.hash(&mut id_hasher);

        let mut value_hasher = DefaultHasher::new();
        "01FFD06NDVZ14W5T1WKKB4KKZX".hash(&mut value_hasher);

        assert_eq!(id_hasher.finish(), value_hasher.finish());
    }

    /// Test that IDs are ordered exactly like their inner values.
    #[test]
    fn ord() {
        let lesser = Id::<UserMarker>::new("01EX2NCWQ0CHS3QJF0FEQS1GR4".to_string());
        let center = Id::<UserMarker>::new("01FFD06NDVZ14W5T1WKKB4KKZX".to_string());
        let greater = Id::<UserMarker>::new("01FSRTTGJC1XJ6ZEQJMSX8Q96C".to_string());

        assert!(center.cmp(&greater).is_lt());
        assert!(center.cmp(&center).is_eq());
        assert!(center.cmp(&lesser).is_gt());
    }
}
