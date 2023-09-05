use std::env;

use crate::models::subject_run::SubjectRun;
use crate::types::UUID;
use crate::{models::subject::CompositeSubject, types::MAResult};
use crate::{
    models::subject::Subject,
    schema::{subject_runs::dsl::subject_runs, subjects::dsl::subjects},
};
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;

const DATABASE_URL: &str = "DATABASE_URL";

impl FromSql<Binary, Sqlite> for UUID {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = <Vec<u8>>::from_sql(bytes)?;
        uuid::Uuid::from_slice(&value)
            .map(UUID)
            .map_err(|e| e.into())
    }
}

pub fn establish_connection() -> MAResult<SqliteConnection> {
    dotenv()?;
    let database_url = env::var(DATABASE_URL)?;
    Ok(SqliteConnection::establish(&database_url)?)
}

pub struct DB<'a> {
    connection: &'a mut SqliteConnection,
}

impl<'a> DB<'a> {
    pub fn get_subjects(&self) -> MAResult<Vec<Subject>> {
        let results = subjects.load(self.connection)?;
        Ok(results)
    }

    pub fn get_subject_runs(&self) -> MAResult<Vec<SubjectRun>> {
        let results = subject_runs.load(self.connection)?;
        Ok(results)
    }

    pub fn get_composite_subjects() -> Vec<CompositeSubject> {
        todo!();
    }
}

#[cfg(test)]
mod tests {}
