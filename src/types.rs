use std::error::Error;

pub type MAResult<T> = Result<T, Box<dyn Error>>;
