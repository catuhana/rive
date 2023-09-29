use std::marker::PhantomData;

use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub mod marker;

pub struct Id<T> {
    value: String,

    phantom: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub fn new(value: String) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }

    pub fn cast<New>(self) -> Id<New> {
        Id::new(self.value)
    }

    pub fn value(self) -> String {
        self.value
    }

    pub fn value_ref(&self) -> &str {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
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

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }
        }

        deserializer.deserialize_newtype_struct(&"Id", IdVisitor::new())
    }
}
