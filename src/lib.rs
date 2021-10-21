#![feature(trace_macros)]
#![feature(iter_advance_by)]
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
            DatabaseError(message: String) {
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
            FolderError(folder: String){
                description("Error creating folder.")
                display("Error creating folder: {}", folder)
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
            WrongKeyLength(wrong_key: String, charset: String){
                description("Wrong key used: Length is not the same than key one")
                display("Key length is {} and charset length is {}", wrong_key.len(), charset.len())
            }
            WrongKeyRepeatedCharacters(wrong_key: String){
                description("Wrong key used: Key uses repeated characters")
                display("{}", wrong_key)
            }
            CharacterMappingError(wrong_char: String){
                description("Error trying to substitute char.")
                display("Char tried to substitute {}", wrong_char)
            }
            EmptyMapping {
                description("Mapping has no more cipherletters.")
                display("Empty mapping.")
            }
            NoMappingAvailable(word: String, dictionary: String){
                description("No candidate mapping was found for word.")
                display("Word was {} and tried dictionary was {}", word, dictionary)
            }
    }
}

/// Trait to use one-letter strings as chars.
pub trait FromStr<T> {
    /// Create a char from a one letter string.
    fn fromStr(s: T) -> Self;
}

impl FromStr<&str> for char {
    fn fromStr(s: &str) -> Self {
        s.chars().next().expect(format!("Could not create char from given string: {}", s).as_str())
    }
}

/// Trait to use to find substrings searching from a given index.
pub trait FindFromIndex<T, U> {

    /// Find text_to_find in text using index as start search position.
    ///
    /// # Parameters:
    /// * text: Text to search into.
    /// * text_to_find: text to look for.
    /// * index: Start search position.
    ///
    /// # Returns:
    /// * Index where text_to_find_was found, counted from text start.
    fn findFromIndex(text: &T, text_to_find: U, index: usize) -> Option<usize>;
}

impl <U: AsRef<str>> FindFromIndex<String, U> for String {
    fn findFromIndex(text: &String, text_to_find: U, index: usize) -> Option<usize> {
        let mut text_iter = text.chars();
        if let Ok(()) = text_iter.advance_by(index) {
            let remaining_text: String = text_iter.collect();
            match remaining_text.find(text_to_find.as_ref()) {
                Some(current_index) => return Some(current_index + index),
                None => None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_from_index() {
        let text = "This is a text where I want to find another text.".to_string();
        let text_to_find = "text";
        let expected_index: usize = 44;
        if let Some(found_index) = String::findFromIndex(&text, text_to_find, 14) {
            assert_eq!(found_index, expected_index);
        } else {
            assert!(false)
        }
    }

}
