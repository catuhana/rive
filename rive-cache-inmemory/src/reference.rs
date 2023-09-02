use dashmap::mapref::{multiple::RefMulti, one::Ref};
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

pub struct IterReference<'a, K, V> {
    inner: RefMulti<'a, K, V>,
}

impl<'a, K, V> IterReference<'a, K, V> {
    /// Create a new iterator element reference.
    pub(crate) const fn new(inner: RefMulti<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<K: Eq + Hash, V> IterReference<'_, K, V> {
    /// Immutable reference to the resource's key.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Immutable reference to the resource's value.
    pub fn value(&self) -> &V {
        self.inner.value()
    }
}

impl<K: Eq + Hash, V> Deref for IterReference<'_, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

impl<K: Eq + Hash, V: Debug> Debug for IterReference<'_, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("IterReference")
            .field("inner", self.value())
            .finish()
    }
}
