use crate::attack::database::{Database, DatabaseSession};
use std::collections::{HashSet, HashMap};
use std::path::PathBuf;

/// Module to deal with words dictionaries.
///
/// A dictionary is a repository of distinct words present in an actual language.

/// Cifra stores word dictionaries in a local database. This class
/// is a wrapper to not to deal directly with that database.
pub struct Dictionary {
    pub language: String,
    database: Database
}

impl Dictionary {
    /// Remove given language from database.
    ///
    /// Be aware that all its words will be removed too.
    ///
    /// # Parameters:
    /// * language: Language to remove from database.
    fn remove_dictionary<T>(language: T)
        where T: AsRef<str> {
        unimplemented!();
    }

    /// Get languages dictionaries present at database.
    ///
    /// # Returns:
    /// * A list with names of dictionaries present at database.
    fn get_dictionaries_names()-> Vec<String> {
        unimplemented!();
    }

    fn new<T>(language: T)-> Self
        where T: AsRef<str> {
        Dictionary {
            language: String::from(language),
            database
        }
    }

    /// Get open session for current dictionary database.
    fn session(&self) -> &DatabaseSession {
        &self.database.session
    }

    /// Add given word to dictionary.
    ///
    /// If word is already present at dictionary, do nothing.
    ///
    /// # Parameters:
    /// * word: word to add to dictionary.
    fn add_word<T>(&mut self, word: T)
        where T: AsRef<str> {
        unimplemented!()
    }

    /// Add given words to dictionary.
    ///
    /// # Parameters:
    /// * words: Set of words to add to dictionary.
    fn add_multiple_words(&mut self, words: HashSet<String>){
        unimplemented!()
    }

    /// Remove given word from dictionary.
    ///
    /// If word is not already present at dictionary, do nothing.
    ///
    /// # Parameters:
    /// * word: word to remove from dictionary.
    fn remove_word<T>(&mut self, word: T)
        where T: AsRef<str> {
        unimplemented!()
    }

    /// Check if given word exists at this dictionary.
    ///
    /// # Parameters:
    /// * word: word to check.
    ///
    /// # Returns:
    /// True if word is already present at dictionary, False otherwise.
    fn word_exists<T>(&self, word: T) -> bool
        where T: AsRef<str> {
        unimplemented!()
    }

    /// Read a file's words and stores them at this language database.
    ///
    /// # Parameters:
    /// * file_pathname: Absolute path to file with text to analyze.
    fn populate<T>(&mut self, file_pathname: T)
        where T: AsRef<PathBuf> {
        unimplemented!()
    }

    /// Check if a table for this instance language already exists at database or not.
    ///
    /// # Returns:
    /// True if a table exists for this instance language, otherwise False.
    fn already_created(&self)-> bool {
        unimplemented!()
    }

    /// Create this instance language table in database.
    fn create_dictionary(&mut self) {

    }
}

/// Extract words from given file.
///
/// # Parameters:
/// * param file_pathname: Absolute filename to file to be read.
///
/// # Returns:
/// A set of words normalized to lowercase and without any punctuation mark.
fn get_words_from_text_file<T>(file_pathname: T) -> HashSet<String>
    where T: AsRef<PathBuf> {
    unimplemented!()
}

/// Language selected as more likely to be the one the message is written into.
///
/// # Members:
/// * winner: Name of language more likely. If None the no proper language was found.
/// * winner_probability: Probability this language is actually de right one. If None the no proper language was found.
/// * candidates: Dict with all languages probabilities. Probabilities are floats from 0 to 1.
struct IdentifiedLanguage {
    winner: Option<String>,
    winner_probability: Option<String>,
    candidates: HashMap<String, String>
}

/// Identify language used to write text.
///
/// It check each word present at text to find out if is present in any language.
/// The language that has more words is select as winner.
///
/// # Parameters:
/// * Text: Text to analyze.
///
/// # Returns:
/// * Language selected as more likely to be the one used to write text.
fn identify_language<T>(text: T)-> IdentifiedLanguage
    where T: AsRef<str> {
    unimplemented!()
}