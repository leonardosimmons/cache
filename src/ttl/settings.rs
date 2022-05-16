#![allow(unused)]
use std::time::Duration;

const DEFAULT_TTL: u64 = 30;

pub(crate) enum TtlRevalidationAction {
    Expire,
    Revalidate,
}

pub(crate) struct TtlSettings {
    action: TtlRevalidationAction,
    duration: Duration,
}

// == impl TtlSettings ==

impl TtlSettings {
    pub fn new(action: TtlRevalidationAction, duration: Duration) -> Self {
        Self { action, duration }
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
