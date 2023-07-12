pub mod fixed_interval;

use std::time::Instant;

use crate::subject_desc::SubjectDesc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RelearningExpectancy {
    Done,
    Next(Instant),
}

pub trait MemoryCurve {
    fn next_trigger(&self, sd: &SubjectDesc) -> RelearningExpectancy {
        if *sd.max_runs() > 0 && sd.runs_history().len() as u64 >= *sd.max_runs() {
            return RelearningExpectancy::Done;
        }
        return self._next_trigger(sd);
    }

    fn _next_trigger(&self, sd: &SubjectDesc) -> RelearningExpectancy;
}