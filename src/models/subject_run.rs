use chrono::NaiveDateTime;
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
    #[diesel(sql_type = Integer)]
    datetime: NaiveDateTime,
}

impl SubjectRun {
    pub fn from_current_datetime(appctx: &AppContext, subject_id: UUID) -> Self {
        Self::from_datetime(subject_id, *appctx.datetime())
    }

    pub fn from_datetime(subject_id: UUID, datetime: NaiveDateTime) -> Self {
        Self::_new(UUID::random(), subject_id, datetime)
    }

    pub(crate) fn _new(id: UUID, subject_id: UUID, datetime: NaiveDateTime) -> Self {
        Self {
            id,
            subject_id,
            datetime,
        }
    }
}
