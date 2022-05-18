use std::hash::{BuildHasher, Hash};
use crate::entries::Entry;

pub mod entries;
pub mod ttl;

pub trait CacheConfiguration<S>
    where
        S: BuildHasher,
{
    /// Sets the max capacity for each cache node
    fn capacity(self, capacity: usize) -> Self;
    /// Sets custom hasher for hash map
    fn hasher(self, hasher: S) -> Self;
}

pub trait CacheNode<V> {
    /// Returns reference to value inside node
    fn value(&self) -> &V;
    /// Returns mutable reference to value inside node
    fn value_mut(&mut self) -> &mut V;
    /// Converts node into value
    fn into_value(self) -> V;
}

pub trait Cache<K, N, V, S>
    where
        K: Hash + Eq,
        N: CacheNode<V>,
        S: BuildHasher,
{
    /// Checks to see if cache contains given key
    fn contains_key(&self, key: &K) -> bool;
    /// Checks for and returns an entry within the cache
    fn entry(&mut self, key: K) -> Option<Entry<K, N, S>>;
    /// Returns a reference to the current value of specified node
    fn get(&mut self, k: &K) -> Option<&V>;
    /// Returns a mutable reference to the current value of the specified node
    fn get_mut(&mut self, k: &K) -> Option<&mut V>;
    /// Removes the specified node & returns the value
    fn remove(&mut self, key: &K) -> Option<V>;
}
