use std::time::Duration;
use crate::utils::Split;

const DEFAULT_TTL: u64 = 30;

pub enum TtlRevalidationAction {
    Expire,
    Revalidate,
}

pub struct TtlSettings {
    action: TtlRevalidationAction,
    duration: Duration,
}

// == impl TtlSettings ==

impl TtlSettings {
    pub fn new(action: TtlRevalidationAction, duration: Duration) -> Self {
        Self { action, duration }
    }

    pub fn action(&self, action: TtlRevalidationAction) -> &TtlRevalidationAction {
        &self.action
    }

    pub fn duration(&self, duration: Duration) -> &Duration {
        &self.duration
    }
}

// == impl std ==

impl Default for TtlRevalidationAction {
    fn default() -> Self {
        TtlRevalidationAction::Expire
    }
}

impl Default for TtlSettings {
    fn default() -> Self {
        Self {
            action: TtlRevalidationAction::default(),
            duration: Duration::from_secs(DEFAULT_TTL),
        }
    }
}

impl Split<TtlRevalidationAction, Duration> for TtlSettings{
    fn split(self) -> (TtlRevalidationAction, Duration) {
        (self.action, self.duration)
    }
}