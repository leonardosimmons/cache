#![allow(unused)]
use crate::ttl::node::{TtlEntry, TtlNode};
use crate::CacheNodeController;
use linked_hash_map::Entry as MapEntry;
use linked_hash_map::LinkedHashMap;
use linked_hash_map::OccupiedEntry as OccupiedMapEntry;
use linked_hash_map::VacantEntry as VacantMapEntry;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::time::{Duration, Instant};

pub struct OccupiedEntry<'a, K: 'a, V: 'a, S: 'a = RandomState> {
    entry: OccupiedMapEntry<'a, K, V, S>,
}

pub struct VacantEntry<'a, K: 'a, V: 'a, S: 'a = RandomState> {
    entry: VacantMapEntry<'a, K, V, S>,
}

pub enum Entry<'a, K: 'a, V: 'a, S: 'a = RandomState> {
    Occupied(OccupiedEntry<'a, K, V, S>),
    Vacant(VacantEntry<'a, K, V, S>),
}

// == impl Entry ==

impl<'a, K, V, S> Entry<'a, K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(e) => e.key(),
            Entry::Vacant(e) => e.key(),
        }
    }
}

// == impl Occupied Entry ==

impl<'a, K, V, S> OccupiedEntry<'a, K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn key(&self) -> &K {
        self.entry.key()
    }

    fn get(&self) -> &V {
        self.entry.get()
    }

    fn get_mut(&mut self) -> &mut V {
        self.entry.get_mut()
    }
}

impl<'a, K, V, S> OccupiedEntry<'a, K, TtlNode<V>, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn insert(mut self, value: V, duration: Duration) -> V {
        let entry = TtlEntry::new(value, duration);
        let prev = self.entry.insert(TtlNode::new(entry));
        prev.into_value()
    }
}

// == impl VacantEntry ==

impl<'a, K, V, S> VacantEntry<'a, K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn key(&self) -> &K {
        self.entry.key()
    }
}

impl<'a, K, V, S> VacantEntry<'a, K, TtlNode<V>, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    fn insert(self, value: V, duration: Duration) -> &'a mut V {
        let entry = TtlEntry::new(value, duration);
        let node = self.entry.insert(TtlNode::new(entry));
        node.value_mut()
    }
}
