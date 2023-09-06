use chrono::{Duration, Utc};
use derive_getters::Getters;

use crate::models::subject::CompositeSubject;

use super::{MemoryCurve, RelearningExpectancy};

#[derive(Getters)]
struct FixedInterval {
    interval: Duration,
}

impl FixedInterval {
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }
}

impl MemoryCurve for FixedInterval {
    fn _next_trigger(&self, cs: &CompositeSubject) -> RelearningExpectancy {
        match cs.subject_runs().last() {
            None => RelearningExpectancy::Next(Utc::now().naive_utc()),
            Some(subject_run) => {
                RelearningExpectancy::Next(*subject_run.datetime() + self.interval)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use crate::{
        memory_curves::{MemoryCurve, RelearningExpectancy},
        models::subject::CompositeSubject,
        types::UUID,
    };

    use super::FixedInterval;

    #[test]
    fn test_next_trigger_returns_done_when_out_of_runs() {
        let duration = Duration::seconds(10000);
        let now = Utc::now().naive_utc();
        let subject_run_times = vec![now, now + duration, now + (duration + duration)];
        let fixed_interval = FixedInterval::new(duration);
        {
            let id = UUID::random();
            let title = "subject1";
            let description = "description1";
            let max_runs = 4;
            let composite_subject =
                CompositeSubject::_new(id, title, description, max_runs, &subject_run_times);
            let relearning_expectancy = fixed_interval.next_trigger(&composite_subject);
            assert_eq!(
                relearning_expectancy,
                RelearningExpectancy::Next(now + Duration::seconds(30000))
            );
        }
        {
            let id = UUID::random();
            let title = "subject1";
            let description = "description1";
            let max_runs = 3;
            let composite_subject =
                CompositeSubject::_new(id, title, description, max_runs, &subject_run_times);
            let relearning_expectancy = fixed_interval.next_trigger(&composite_subject);
            assert_eq!(relearning_expectancy, RelearningExpectancy::Done);
        }
        {
            let id = UUID::random();
            let title = "subject1";
            let description = "description1";
            let max_runs = 0;
            let composite_subject =
                CompositeSubject::_new(id, title, description, max_runs, &subject_run_times);
            let relearning_expectancy = fixed_interval.next_trigger(&composite_subject);
            assert_eq!(
                relearning_expectancy,
                RelearningExpectancy::Next(now + Duration::seconds(30000))
            );
        }
    }
}
