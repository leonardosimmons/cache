use std::time::Duration;

pub mod cache;
pub mod node;
pub mod settings;

use node::TtlEntry;
use settings::TtlRevalidationAction;

pub enum TtlStatus {
    Valid,
    Expired
}

pub(crate) trait Ttl<V> {
    /// Inserts an occupied node into cache
    fn insert(&mut self, entry: TtlEntry<V>) -> &mut V;
    /// Checks to see if node has reached its TTL bound
    fn validate(&self) -> TtlStatus;
}

trait TtlConfiguration {
    /// Sets the `action` type for new cache nodes
    fn action(self, action: TtlRevalidationAction) -> Self;
    /// Sets the Time To Live `(TTL)` for new cache nodes
    fn duration(self, duration: Duration) -> Self;
}
