use dashmap::mapref::one::Ref;
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    hash::Hash,
    ops::Deref,
};

/// Immutable reference to a resource in the cache.
// We need this so as not to expose the underlying cache implementation.
// From https://github.com/twilight-rs/twilight/blob/main/twilight-cache-inmemory/src/lib.rs
pub struct Reference<'a, K, V> {
    inner: Ref<'a, K, V>,
}

impl<'a, K: Eq + Hash, V> Reference<'a, K, V> {
    /// Create a new reference from a `DashMap` reference.
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(inner: Ref<'a, K, V>) -> Self {
        Self { inner }
    }

    /// Immutable reference to the key identifying the resource.
    pub fn key(&'a self) -> &'a K {
        self.inner.key()
    }

    /// Immutable reference to the underlying value.
    pub fn value(&'a self) -> &'a V {
        self.inner.value()
    }
}

impl<K: Eq + Hash, V: Debug> Debug for Reference<'_, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Reference")
            .field("inner", self.value())
            .finish()
    }
}

impl<'a, K: Eq + Hash, V> Deref for Reference<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
