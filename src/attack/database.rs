/// Cifra database definition.
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::env::VarError;

#[macro_use]
extern crate diesel_migrations;

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
        Ok(var_value) => Ok(()),
        Err(e) => {
            env::set_var(DATABASE_ENV_VAR, DATABASE_STANDARD_PATH);
            Err(e)
        }
    };
}

/// Connect to current dictionaries database.
///
/// Returns:
/// A connection to underlying database.
pub fn establish_connection()-> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

