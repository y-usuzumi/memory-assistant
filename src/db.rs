use std::env;

use diesel::Connection;
use dotenvy::dotenv;

const DATABASE_URL: &str = "DATABASE_URL";

use crate::types::MAResult;

pub fn establish_connection<Conn: Connection>() -> MAResult<Conn> {
    dotenv()?;
    let database_url = env::var(DATABASE_URL)?;
    Ok(Conn::establish(&database_url)?)
}

#[cfg(test)]
mod tests {}
