use std::time::Instant;

use derive_getters::Getters;
use diesel::{prelude::Queryable, Selectable};
use uuid::Uuid;

use crate::app_context::AppContext;

#[derive(Queryable, Selectable, Getters)]
#[diesel(table_name = crate::schema::subject_descs)]
pub struct SubjectDesc {
    id: Uuid,
    title: String,
    description: String,
    runs_history: Vec<Instant>,
    max_runs: u64, // 0 represents infinite re-learnings
}

impl SubjectDesc {
    pub fn new(appctx: &AppContext, title: &str, description: &str, max_runs: u64) -> Self {
        Self::_new(
            Uuid::new_v4(),
            title,
            description,
            vec![*appctx.timestamp()],
            max_runs,
        )
    }

    pub(crate) fn _new(
        id: Uuid,
        title: &str,
        description: &str,
        runs_history: Vec<Instant>,
        max_runs: u64,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            runs_history,
            max_runs,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(runs_history: Vec<Instant>, max_runs: u64) -> Self {
        Self::_new(
            Uuid::new_v4(),
            "Test title",
            "Test description",
            runs_history,
            max_runs,
        )
    }
}
