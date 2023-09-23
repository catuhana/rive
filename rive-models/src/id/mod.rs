use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

pub mod marker;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T> {
    value: String,

    #[serde(skip, default)]
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

impl<T> From<String> for Id<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
