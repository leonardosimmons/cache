use crate::entries::{OccupiedEntry, VacantEntry};
use crate::ttl::node::{TtlEntry, TtlNode};
use crate::ttl::settings::{TtlRevalidationAction, TtlSettings};
use crate::ttl::{Ttl, TtlConfiguration, TtlStatus};
use crate::utils::Split;
use crate::{Cache, CacheConfiguration, CacheNode, Entry};
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
    pub fn build<K: Hash + Eq, V>(self) -> TtlCache<K, V, S> {
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

    fn settings(mut self, settings: TtlSettings) -> Self {
        let (action, duration) = settings.split();
        self.action = action;
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
    /// Adds new entry to cache and returns old value if there is one
    fn push(&mut self, key: K, val: V) -> Option<V> {
        let entry = TtlEntry::new(val, self.settings.duration().clone());
        self.cache
            .insert(key, entry.into())
            .map(|old_node| old_node.into_value())
            .or(None)
    }

    /// Returns current nodes `TTL Status`
    fn status(&self, key: &K) -> Option<TtlStatus> {
        self.cache
            .get(key)
            .map(|node| node.validate())
            .or(None)
    }
}

impl<K, V, S> Cache<K, TtlNode<V>, V, S> for TtlCache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn capacity(&self) -> usize {
        self.cache.capacity()
    }

    fn clear(&mut self) {
        self.cache.clear()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    fn entry(&mut self, key: K) -> Option<Entry<K, TtlNode<V>, S>> {
        match self.status(&key) {
            Some(status) => match status {
                TtlStatus::Valid => {
                    match self.cache.entry(key) {
                        MapEntry::Occupied(entry) => Some(Entry::Occupied(OccupiedEntry { entry })),
                        MapEntry::Vacant(entry) => Some(Entry::Vacant(VacantEntry { entry }))
                    }
                }
                TtlStatus::Expired => {
                    match self.cache.entry(key) {
                        MapEntry::Occupied(entry) => {
                            entry.remove();
                            None
                        },
                        _ => None
                    }
                }
            }
            None => None
       }
    }

    fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.len() < self.capacity() {
            self.push(key, val)
        } else {
            self.cache.pop_front();
            self.push(key, val)
        }
    }

    fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        match self.status(key) {
            Some(status) => match status {
                TtlStatus::Valid => Some(self.cache.get(key).unwrap().value()),
                TtlStatus::Expired => self.remove(key).and_then(|_| None)
            },
            None => None
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.status(key) {
            Some(status) => match status {
                TtlStatus::Valid => Some(self.cache.get_mut(key).unwrap().value_mut()),
                TtlStatus::Expired => self.remove(key).and_then(|_| None)
            },
            None => None
        }
    }

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.status(key)
            .map(|_| self.cache.remove(key).unwrap().into_value())
            .or(None)
    }
}
