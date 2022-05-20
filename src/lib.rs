#![allow(unused)]
use std::hash::{BuildHasher, Hash};
use crate::entries::Entry;

pub mod entries;
pub mod ttl;
pub mod utils;

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
    /// Returns the maximum capacity of the cache
    fn capacity(&self) -> usize;
    /// Clears the cache
    fn clear(&mut self);
    /// Checks to see if cache contains given key
    fn contains_key(&self, key: &K) -> bool;
    /// Checks for and returns an entry within the cache
    fn entry(&mut self, key: K) -> Option<Entry<K, N, S>>;
    /// Inserts a key-value pair into the cache
    fn insert(&mut self, key: K, val: V) -> Option<V>;
    /// Checks rather or not the cache is currently empty
    fn is_empty(&self) -> bool;
    /// Returns a reference to the current value of specified node
    fn get(&mut self, k: &K) -> Option<&V>;
    /// Returns a mutable reference to the current value of the specified node
    fn get_mut(&mut self, k: &K) -> Option<&mut V>;
    /// Returns the current number of key-value pairs within the cache
    fn len(&self) -> usize;
    /// Removes the specified node & returns the value
    fn remove(&mut self, key: &K) -> Option<V>;
}

