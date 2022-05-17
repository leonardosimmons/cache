use crate::entries::{OccupiedEntry, VacantEntry};
use crate::ttl::node::TtlNode;
use crate::ttl::settings::{TtlRevalidationAction, TtlSettings};
use crate::ttl::{Ttl, TtlConfiguration, TtlStatus};
use crate::{Cache, CacheConfiguration, Entry};
use linked_hash_map::Entry as MapEntry;
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
    /// Returns current nodes `TTL Status`
    fn ttl(&self, key: &K) -> TtlStatus {
        self.cache.get(key).map(|node| node.validate()).unwrap()
    }
}

impl<K, V, S> Cache<K, TtlNode<V>, S> for TtlCache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn entry(&mut self, key: K) -> Entry<K, TtlNode<V>, S> {
        if let TtlStatus::Expired = self.ttl(&key) {
            self.cache.remove(&key);
        }

        match self.cache.entry(key) {
            MapEntry::Occupied(entry) => Entry::Occupied(OccupiedEntry { entry }),
            MapEntry::Vacant(entry) => Entry::Vacant(VacantEntry { entry }),
        }
    }
}
