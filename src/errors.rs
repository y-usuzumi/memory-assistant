use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("DB error: '{0}")]
    DBError(String),
}
