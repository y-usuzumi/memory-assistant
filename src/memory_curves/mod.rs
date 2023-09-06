pub mod fixed_interval;

use chrono::NaiveDateTime;

use crate::models::subject::CompositeSubject;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RelearningExpectancy {
    Done,
    Next(NaiveDateTime),
}

pub trait MemoryCurve {
    fn next_trigger(&self, cs: &CompositeSubject) -> RelearningExpectancy {
        let subject_max_runs = *cs.subject().max_runs();
        if subject_max_runs > 0 && cs.subject_runs().len() >= subject_max_runs as usize {
            return RelearningExpectancy::Done;
        }
        return self._next_trigger(cs);
    }

    fn _next_trigger(&self, cs: &CompositeSubject) -> RelearningExpectancy;
}
