use crate::CacheNodeController;
use crate::ttl::{Ttl, TtlStatus};
use std::time::{Duration, Instant};

pub struct TtlEntry<V> {
    value: V,
    duration: Duration,
}

pub(crate) struct TtlNode<V> {
    value: V,
    expiration: Instant,
}

// == impl TtlEntry ==

impl<V> TtlEntry<V> {
    pub(crate) fn new(value: V, duration: Duration) -> Self {
        Self { value, duration }
    }
}

// == impl TtlNode ==

impl<V> TtlNode<V> {
    pub fn new(entry: TtlEntry<V>) -> Self {
        Self {
            value: entry.value,
            expiration: Instant::now() + entry.duration,
        }
    }
}

impl<V> CacheNodeController<V> for TtlNode<V> {
    fn value(&self) -> &V {
        &self.value
    }

    fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    fn into_value(self) -> V {
        self.value
    }
}

impl<V> Ttl<V> for TtlNode<V> {
    fn insert(&mut self, entry: TtlEntry<V>) -> &mut V {
        self.value = entry.value;
        self.expiration = Instant::now() + entry.duration;
        &mut self.value
    }

    fn validate(&self) -> TtlStatus {
        match Instant::now() > self.expiration {
            true => TtlStatus::Expired,
            false => TtlStatus::Valid
        }
    }
}

// == impl std ==

impl<V> From<TtlEntry<V>> for TtlNode<V> {
    fn from(entry: TtlEntry<V>) -> Self {
        TtlNode {
            value: entry.value,
            expiration: Instant::now() + entry.duration,
        }
    }
}
