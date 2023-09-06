use chrono::NaiveDateTime;
use derive_getters::Getters;
use diesel::{
    prelude::{Identifiable, Queryable},
    Selectable,
};

use crate::{app_context::AppContext, schema::subjects, types::UUID};

use super::subject_run::SubjectRun;

#[derive(Identifiable, Queryable, Selectable, Getters)]
#[diesel(table_name = subjects)]
pub struct Subject {
    id: UUID,
    title: String,
    description: String,
    max_runs: i32, // 0 represents infinite re-learnings
}

impl Subject {
    pub fn new(appctx: &AppContext, title: &str, description: &str, max_runs: i32) -> Self {
        Self::_new(UUID::random(), title, description, max_runs)
    }

    pub(crate) fn _new(id: UUID, title: &str, description: &str, max_runs: i32) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            max_runs,
        }
    }
}

#[derive(Getters)]
pub struct CompositeSubject {
    subject: Subject,
    subject_runs: Vec<SubjectRun>,
}

impl CompositeSubject {
    pub fn new(appctx: &AppContext, title: &str, description: &str, max_runs: i32) -> Self {
        Self::_new(
            UUID::random(),
            title,
            description,
            max_runs,
            &[*appctx.datetime()],
        )
    }

    pub(crate) fn _new(
        id: UUID,
        title: &str,
        description: &str,
        max_runs: i32,
        subject_run_times: &[NaiveDateTime],
    ) -> Self {
        let subject = Subject::_new(id, title, description, max_runs);
        let subject_runs = subject_run_times
            .iter()
            .map(|datetime| SubjectRun::from_datetime(id, *datetime))
            .collect();
        Self {
            subject,
            subject_runs,
        }
    }
}
