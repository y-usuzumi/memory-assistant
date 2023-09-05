use std::time::{Duration, Instant};

use derive_getters::Getters;

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
    fn _next_trigger(&self, td: &crate::subject_desc::SubjectDesc) -> RelearningExpectancy {
        match td.runs_history().last() {
            None => RelearningExpectancy::Next(Instant::now()),
            Some(instant) => RelearningExpectancy::Next(*instant + self.interval),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use uuid::Uuid;

    use crate::{
        memory_curves::{MemoryCurve, RelearningExpectancy},
        subject_desc::SubjectDesc,
    };

    use super::FixedInterval;

    #[test]
    fn test_next_trigger_returns_done_when_out_of_runs() {
        let duration = Duration::from_secs(10000);
        let now = Instant::now();
        let runs_history = vec![now, now + duration, now + 2 * duration];
        let fixed_interval = FixedInterval::new(duration);
        {
            let subject_desc = SubjectDesc::_new(
                Uuid::new_v4(),
                "subject1",
                "description1",
                runs_history.clone(),
                4,
            );
            let relearning_expectancy = fixed_interval.next_trigger(&subject_desc);
            assert_eq!(
                relearning_expectancy,
                RelearningExpectancy::Next(now + 3 * Duration::from_secs(10000))
            );
        }
        {
            let subject_desc = SubjectDesc::_new(
                Uuid::new_v4(),
                "subject2",
                "description2",
                runs_history.clone(),
                3,
            );
            let relearning_expectancy = fixed_interval.next_trigger(&subject_desc);
            assert_eq!(relearning_expectancy, RelearningExpectancy::Done);
        }
        {
            let subject_desc = SubjectDesc::_new(
                Uuid::new_v4(),
                "subject3",
                "description3",
                runs_history.clone(),
                0,
            );
            let relearning_expectancy = fixed_interval.next_trigger(&subject_desc);
            assert_eq!(
                relearning_expectancy,
                RelearningExpectancy::Next(now + 3 * Duration::from_secs(10000))
            );
        }
    }
}
