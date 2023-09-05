pub mod fixed_interval;

use std::time::Instant;

use crate::models::subject::CompositeSubject;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RelearningExpectancy {
    Done,
    Next(Instant),
}

pub trait MemoryCurve {
    fn next_trigger(&self, cs: &CompositeSubject) -> RelearningExpectancy {
        let subject_max_runs = *cs.subject().max_runs();
        if subject_max_runs > 0 && cs.subject_runs().len() as u64 >= subject_max_runs {
            return RelearningExpectancy::Done;
        }
        return self._next_trigger(cs);
    }

    fn _next_trigger(&self, cs: &CompositeSubject) -> RelearningExpectancy;
}
