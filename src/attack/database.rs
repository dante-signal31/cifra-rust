/// Cifra database definition.
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::env::VarError;

use crate::schema::languages;
use crate::schema::languages::dsl::*;
use crate::schema::words;
use crate::schema::words::dsl::*;

use crate::{ErrorKind, Result, ResultExt};
use std::fmt::Error;

embed_migrations!();

pub type DatabaseSession = SqliteConnection;

pub const DATABASE_ENV_VAR: &'static str = "DATABASE_URL";
const DATABASE_STANDARD_PATH: &'static str = "~/.cifra/cifra_database.sqlite";

/// Check if DATABASE_URL environment variable actually exists and create it if not.
///
/// At tests a .env file is used to shadow default DATABASE_URL var. But at production
/// that environment variable must be set to let cifra find its database. If this
/// function does not find DATABASE_URL then it creates that var and populates it
/// with default value stored at *DATABASE_STANDARD_PATH*, but notifies that situations
/// returning a VarError.
///
/// # Returns:
/// * Environment value if DEFAULT_URL exists and a VarError if not.
fn check_database_url_env_var_exists()-> Result<String>{
    return env::var(DATABASE_ENV_VAR)
        .chain_err (|| {
            env::set_var(DATABASE_ENV_VAR, DATABASE_STANDARD_PATH);
            ErrorKind::DatabaseError("Error finding out if database env var existed previously.")
        })
}

/// Create and populate database with its default tables.
pub fn create_database()-> Result<Database> {
    let database = Database::new()?;
    embedded_migrations::run(&database.session)
        .chain_err(|| ErrorKind::DatabaseError("Error running database migrations."))?;
    Ok(database)
}

pub struct Database {
    pub session: DatabaseSession,
    database_path: String
}

impl Database {

    pub fn new() -> Result<Self> {
        let database_path = check_database_url_env_var_exists()
            .chain_err(|| ErrorKind::DatabaseError("Error checking if DATABASE_URL environment variable exists."))?;
        Ok(Database {
           session: Self::open_session()?,
           database_path
        })
    }

    /// Connect to current dictionaries database.
    ///
    /// Returns:
    /// A connection to underlying database.
    fn open_session() -> Result<DatabaseSession> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .chain_err(|| ErrorKind::DatabaseError("DATABASE_URL must be set"))?;
        SqliteConnection::establish(&database_url)
            .chain_err(|| ErrorKind::DatabaseError("Error connecting to DATABASE_URL"))
    }
}

/// Model for Languages database table.
#[derive(Queryable, Identifiable, Associations, Debug, PartialEq)]
#[table_name="languages"]
// #[has_many(words)]
pub struct Language {
    pub id: i32,
    pub language: String,
}

#[derive(Insertable)]
#[table_name="languages"]
pub struct NewLanguage<'a> {
    pub language: &'a str,
}


/// Model for Words database table.
#[derive(Queryable, Identifiable, Associations, Debug, PartialEq)]
#[table_name="words"]
#[belongs_to(Language)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub word_pattern: String,
    pub language_id: i32
}

#[derive(Insertable)]
#[table_name="words"]
pub struct NewWord<'a> {
    pub word: &'a str,
    pub word_pattern: String,
    pub language_id: i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::path::Path;
    use test_common::fs::tmp::TestEnvironment;
    use test_common::system::env::TemporalEnvironmentVariable;

    #[test]
    fn test_create_database() {
        let test_folder = TestEnvironment::new();
        let absolute_path_to_database = test_folder.path().join("cifra_database.sqlite");
        let absolute_pathname_to_database = match absolute_path_to_database.to_str() {
            Some(path)=> path,
            None=> panic!("Path uses non valid characters.")
        };
        // Database does not exists yet.
        let database_path = Path::new(absolute_pathname_to_database);
        assert!(!database_path.exists());
        let test_env = TemporalEnvironmentVariable::new("DATABASE_URL", absolute_pathname_to_database);
        create_database();
        // Database now exists.
        assert!(database_path.exists());
    }


}