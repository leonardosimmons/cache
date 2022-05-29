use std::time::Duration;

pub mod cache;
pub mod node;

use node::TtlEntry;

pub(crate) enum TtlStatus {
    Valid,
    Expired
}

trait Ttl<V> {
    /// Inserts an occupied node into cache
    fn insert(&mut self, entry: TtlEntry<V>) -> &mut V;
    /// Checks to see if node has reached its TTL bound
    fn validate(&self) -> TtlStatus;
}

pub trait TtlConfiguration {
    /// Sets the Time To Live `(TTL)` for new cache nodes
    fn duration(self, duration: Duration) -> Self;
}
