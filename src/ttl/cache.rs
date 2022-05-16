#![allow(unused)]
use crate::ttl::node::TtlNode;
use crate::ttl::settings::{TtlRevalidationAction, TtlSettings};
use crate::ttl::TtlConfiguration;
use crate::{Cache, CacheConfiguration};
use bytes::Bytes;
use linked_hash_map::LinkedHashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::time::Duration;

const DEFAULT_CAPACITY: usize = 1024;
const DEFAULT_TTL: u64 = 30;

pub struct TtlCacheBuilder<S = RandomState>
where
    S: BuildHasher,
{
    action: TtlRevalidationAction,
    capacity: usize,
    duration: Duration,
    hasher: S,
}

pub struct TtlCache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    cache: LinkedHashMap<K, TtlNode<V>, S>,
    settings: TtlSettings,
}

// == impl TtlCacheBuilder ==

impl TtlCacheBuilder {
    pub fn new() -> Self {
        Self {
            action: TtlRevalidationAction::default(),
            capacity: DEFAULT_CAPACITY,
            duration: Duration::from_secs(DEFAULT_TTL),
            hasher: RandomState::new(),
        }
    }
}

impl<S> TtlCacheBuilder<S>
where
    S: BuildHasher,
{
    fn build<K: Hash + Eq, V>(self) -> TtlCache<K, V, S> {
        TtlCache {
            cache: LinkedHashMap::with_capacity_and_hasher(self.capacity, self.hasher),
            settings: TtlSettings::new(self.action, self.duration),
        }
    }
}

impl<S> CacheConfiguration<S> for TtlCacheBuilder<S>
where
    S: BuildHasher,
{
    fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    fn hasher(mut self, hasher: S) -> Self {
        self.hasher = hasher;
        self
    }
}

impl<S> TtlConfiguration for TtlCacheBuilder<S>
where
    S: BuildHasher,
{
    fn action(mut self, action: TtlRevalidationAction) -> Self {
        self.action = action;
        self
    }

    fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

// == impl TtlCache ==

impl<K, V, S> TtlCache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn test() {
        let cache: TtlCache<String, Bytes, _> = TtlCacheBuilder::new()
            .duration(Duration::from_secs(30))
            .action(TtlRevalidationAction::Expire)
            .capacity(4096)
            .hasher(RandomState::new())
            .build();
    }
}

impl<K, V, S> Cache<K, V, S> for TtlCache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
}
