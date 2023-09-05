use std::time::Instant;

use derive_getters::Getters;
use diesel::{
    prelude::{Identifiable, Queryable},
    Selectable,
};

use crate::{app_context::AppContext, schema::subject_runs, types::UUID};

#[derive(Identifiable, Queryable, Selectable, Getters)]
#[diesel(table_name = subject_runs)]
#[diesel(belongs_to(Subject))]
pub struct SubjectRun {
    id: UUID,
    subject_id: UUID,
    time: Instant,
}

impl SubjectRun {
    pub fn from_current_time(appctx: &AppContext, subject_id: UUID) -> Self {
        Self::from_time(subject_id, *appctx.time())
    }

    pub fn from_time(subject_id: UUID, time: Instant) -> Self {
        Self::_new(UUID::random(), subject_id, time)
    }

    pub(crate) fn _new(id: UUID, subject_id: UUID, time: Instant) -> Self {
        Self {
            id,
            subject_id,
            time,
        }
    }
}
