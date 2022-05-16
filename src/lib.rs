use std::hash::{BuildHasher, Hash};

pub mod entries;
pub mod ttl;

pub trait Cache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
}

pub trait CacheConfiguration<S>
where
    S: BuildHasher,
{
    /// Sets the max capacity for each cache node
    fn capacity(self, capacity: usize) -> Self;
    /// Sets custom hasher for hash map
    fn hasher(self, hasher: S) -> Self;
}

pub trait CacheNodeController<V> {
    /// Returns reference to value inside node
    fn value(&self) -> &V;
    /// Returns mutable reference to value inside node
    fn value_mut(&mut self) -> &mut V;
    /// Converts node into value
    fn into_value(self) -> V;
}