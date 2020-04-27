#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod attack;
pub mod cipher;
// pub mod errors;
pub mod encoding;
mod schema;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate error_chain;

use crate::cipher::affine::WrongAffineKey;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    errors {
            ConversionError(var: &'static str, var_type: &'static str, tried_type: &'static str) {
                description("Conversion failed.")
                display("{} type variable '{}' could not converted to {}", var_type, var, tried_type)
            }
            DatabaseError(message: &'static str) {
                description("Database error")
                display("{}", message)
            }
            StringIndexError(searched_string: String, message: &'static str){
                description("Error looking for a string.")
                display("Error looking for {} text. Additional information: {}", searched_string, message)
            }
            IOError(file: String){
                description("Error reading/writing file.")
                display("Error reading/writing {} file.", file)
            }
            KeyError(key: String, message: String){
                description("Error with given key.")
                display("Problem with key {}:\n{}", key, message)
            }
            NotExistingLanguage(language_tried: String) {
                description("You have tried to operate with a language that does not exist yet at database.")
                display("Does not exist any dictionary for {} language", language_tried)
            }
            WrongAffineKeyError(wrong_key: WrongAffineKey){
                description("You selected a wrong Affine key.")
                display("{}", wrong_key)
            }
    }
}
