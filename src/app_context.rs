use std::time::Instant;

use derive_getters::Getters;

#[derive(Getters)]
pub(crate) struct AppContext {
    timestamp: Instant
}