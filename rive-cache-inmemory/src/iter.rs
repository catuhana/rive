use dashmap::iter::Iter;
use rive_models::{channel::Channel, message::Message, server::Server, user::User};
use std::hash::Hash;

use crate::{reference::IterReference, InMemoryCache};

pub struct ResourceIter<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> ResourceIter<'a, K, V> {
    pub(crate) const fn new(iter: Iter<'a, K, V>) -> Self {
        Self { iter }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for ResourceIter<'a, K, V> {
    type Item = IterReference<'a, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(IterReference::new)
    }
}

#[derive(Clone, Debug)]
pub struct InMemoryCacheIter<'a>(&'a InMemoryCache);

impl<'a> InMemoryCacheIter<'a> {
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    pub fn users(&'a self) -> ResourceIter<String, User> {
        ResourceIter::new(self.0.users.iter())
    }

    pub fn serevrs(&'a self) -> ResourceIter<String, Server> {
        ResourceIter::new(self.0.servers.iter())
    }

    pub fn channels(&'a self) -> ResourceIter<String, Channel> {
        ResourceIter::new(self.0.channels.iter())
    }

    pub fn messages(&'a self) -> ResourceIter<String, Message> {
        ResourceIter::new(self.0.messages.iter())
    }
}
