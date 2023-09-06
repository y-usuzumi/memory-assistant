use std::error::Error;

use diesel::sql_types::Binary;
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::SqlType};

pub type MAResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Hash, Eq, PartialEq, SqlType)]
#[diesel(sql_type = Binary)]
#[diesel(sqlite_type(name = "Binary"))]
pub struct UUID(pub uuid::Uuid);

impl UUID {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<UUID> for uuid::Uuid {
    fn from(s: UUID) -> Self {
        s.0
    }
}
