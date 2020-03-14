/// Cifra database definition.
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::env::VarError;
use std::path::PathBuf;

embed_migrations!();


const DATABASE_ENV_VAR: &'static str = "DATABASE_URL";
const DATABASE_STANDARD_PATH: &'static str = "~/.cifra/cifra_database.sqlite";

/// Check if DATABASE_URL environment variable actually exists and create it if not.
///
/// At tests a .env file is used to shadow default DATABASE_URL var. But at production
/// that environment variable must be set to let cifra find its database. If this
/// function does not find DATABASE_URL then it creates that var and populates it
/// with default value stored at *DATABASE_STANDARD_PATH*, but notifies that situations
/// returning a VarError.
///
/// Returns:
/// Ok(()) is DEFAULT_URL exists and a VarError if not.
fn check_database_url_env_var_exists()-> Result<(), VarError>{
    return match env::var(DATABASE_ENV_VAR) {
        Ok(_var_value) => Ok(()),
        Err(e) => {
            env::set_var(DATABASE_ENV_VAR, DATABASE_STANDARD_PATH);
            Err(e)
        }
    };
}

/// Create and populate database wit its default tables.
pub fn create_database(){
    let database = Database::new();
    embedded_migrations::run(&database.session);
}

pub struct Database {
    session: SqliteConnection
}

impl Database {

    pub fn new() -> Self {
        Database {
           session: Self::open_session()
        }
    }

    /// Connect to current dictionaries database.
    ///
    /// Returns:
    /// A connection to underlying database.
    fn open_session() -> SqliteConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
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
        assert!(!Path::new(absolute_pathname_to_database).exists());
        let test_env = TemporalEnvironmentVariable::new("DATABASE_URL", absolute_pathname_to_database);
        create_database();
        // Database now exists.
        assert!(Path::new(absolute_pathname_to_database).exists());
    }


}