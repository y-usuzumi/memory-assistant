use std::time::Instant;

use derive_getters::Getters;
use uuid::Uuid;

#[derive(Getters)]
pub struct SubjectDesc {
    id: Uuid,
    runs_history: Vec<Instant>,
    max_runs: u64 // 0 represents infinite re-learnings
}

impl SubjectDesc {
    pub fn new(max_runs: u64) -> Self {
        Self::_new(Uuid::new_v4(), Vec::new(), max_runs)
    }

    pub(crate) fn _new(id: Uuid, runs_history: Vec<Instant>, max_runs: u64) -> Self {
        Self {
            id,
            runs_history,
            max_runs
        }
    }
}