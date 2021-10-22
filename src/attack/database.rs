/// Cifra database definition.
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
// use std::env::VarError;

use crate::{ErrorKind, Result, ResultExt, Error};
use crate::schema::languages;
// use crate::schema::languages::dsl::*;
use crate::schema::words;
// use crate::schema::words::dsl::*;
// use std::fmt::Error;

embed_migrations!("migrations/");

pub type DatabaseSession = SqliteConnection;

pub const DATABASE_ENV_VAR: &'static str = "DATABASE_URL";
// const DATABASE_STANDARD_PATH: &'static str = ".cifra/cifra_database.sqlite";
// Database is going to be stored in this path that is relative to user home folder.
const DATABASE_STANDARD_RELATIVE_PATH: &'static str = ".cifra/cifra_database.sqlite";

/// This type provides a dynamically built path to database folder.
///
/// Just create an instance of this type and use it whenever a PathBuf, String or OsString with
/// database path is expected. This type will automatically convert to those three types.
struct DATABASE_STANDARD_PATH;

impl DATABASE_STANDARD_PATH{
    fn get_database_standard_path()-> PathBuf{
        let mut path: PathBuf = dirs::home_dir().expect("No home user found to place database file.");
        path.push(PathBuf::from(DATABASE_STANDARD_RELATIVE_PATH));
        path
    }
}

impl From<DATABASE_STANDARD_PATH> for String {
    fn from(_: DATABASE_STANDARD_PATH) -> Self {
        let path = DATABASE_STANDARD_PATH::get_database_standard_path();
        String::from(path.to_str().unwrap())
    }
}

impl From<DATABASE_STANDARD_PATH> for PathBuf {
    fn from(_: DATABASE_STANDARD_PATH) -> Self {
        DATABASE_STANDARD_PATH::get_database_standard_path()
    }
}

impl From<DATABASE_STANDARD_PATH> for OsString {
    fn from(_: DATABASE_STANDARD_PATH) -> Self {
        let path = DATABASE_STANDARD_PATH::get_database_standard_path();
        path.into_os_string()
    }
}

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
            ErrorKind::DatabaseError(String::from("Error finding out if database env var existed previously."))
        })
}

/// Create and populate database with its default tables.
pub fn create_database()-> Result<Database> {
    let database = Database::new()?;
    embedded_migrations::run(&database.session)
        .chain_err(|| ErrorKind::DatabaseError(String::from("Error running database migrations.")))?;
    Ok(database)
}

/// Take a path and create all folders that don't actually exists yet.
fn create_folder_path(path: &Path) -> Result<()>{
    if let Ok(()) = fs::create_dir_all(path) {
        Ok(())
    } else {
        let path_string = path.as_os_str();
        bail!(String::from(path_string.to_str().unwrap()))
    }
}

pub struct Database {
    pub session: DatabaseSession,
    database_path: String
}

impl Database {

    /// Create a new Database type.
    ///
    /// At creation it checks if DATABASE_URL environment variable actually exists and
    /// create it if not.
    ///
    /// At tests a .env file is used to shadow default DATABASE_URL var. But at production
    /// that environment variable must be set to let cifra find its database. If this
    /// function does not find DATABASE_URL then it creates that var and populates it
    /// with default value stored at *DATABASE_STANDARD_PATH*
    ///
    /// If DATABASE_URL was not set that is a signal about database does not exist yet so
    /// its created too.
    pub fn new() -> Result<Self> {
        if let Ok(database_path) = check_database_url_env_var_exists() {
            // Database already exists.
            Ok(Database {
                session: Self::open_session()?,
                database_path
            })
        } else {
            // Database does not exists yet. So we must create it.
            let database_path: OsString = DATABASE_STANDARD_PATH.into();
            env::set_var(DATABASE_ENV_VAR, database_path.as_os_str());
            let database_path = PathBuf::from(DATABASE_STANDARD_PATH);
            let database_folder = database_path.parent()
                .chain_err(|| ErrorKind::FolderError(String::from(database_path.as_os_str().to_str().unwrap())))?;
            create_folder_path(database_folder);
            create_database()
        }
    }

    /// Connect to current dictionaries database.
    ///
    /// Returns:
    /// A connection to underlying database.
    fn open_session() -> Result<DatabaseSession> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .chain_err(|| ErrorKind::DatabaseError(String::from("DATABASE_URL must be set")))?;
        SqliteConnection::establish(&database_url)
            .chain_err(|| ErrorKind::DatabaseError(format!("Error connecting to DATABASE_URL: {}", database_url)))
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

    #[test]
    fn test_create_database_path() {
        let test_folder = TestEnvironment::new();
        let absolute_path_to_database = test_folder.path().join(".cifra/parent1/parent2/cifra_database.sqlite");
        let database_folder = absolute_path_to_database.parent().unwrap();
        assert!(!database_folder.exists());
        create_folder_path(database_folder);
        assert!(database_folder.exists());
    }

    #[test]
    fn test_database_standard_path() {
        let mut expected_database_folder = dirs::home_dir().unwrap();
        expected_database_folder.push(DATABASE_STANDARD_RELATIVE_PATH);
        assert_eq!(expected_database_folder.into_os_string(), OsString::from(DATABASE_STANDARD_PATH));
    }


}