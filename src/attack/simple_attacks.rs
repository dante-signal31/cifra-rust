/// Module to keep common functions used by caesar and transposition attacks.
use std::collections::HashMap;
use rayon::prelude::*;

use crate::{ErrorKind, Result, ResultExt};
use crate::attack::dictionaries::{IdentifiedLanguage, identify_language, get_best_result, Dictionary};
// use diesel::sql_types::Integer;


#[derive(Clone)]
pub enum ParameterValue {
    Str(String),
    Int(usize)
}

/// Whereas Python offers variable arguments for its functions, Rust does not. The only way I've found
/// to use variable arguments with function pointers is pass parameters stored in a single HashMap.
///
/// This type use a HashMap to store both usize parameters and string ones.
#[derive(Clone)]
pub struct Parameters {
    parameters: HashMap<&'static str, ParameterValue>
}

impl Parameters {
    /// Create a new Parameters type.
    pub fn new()-> Self{
        Parameters{
            parameters: HashMap::new()
        }
    }

    /// Get value at given key assuming it is an usize value.
    ///
    /// Error if given key does not exist or its value is not an usize.
    ///
    /// # Parameters:
    /// * key: Key to search its value.
    ///
    /// # Returns
    /// * Key's value.
    pub fn get_int(&self, key: &'static str)-> Result<usize> {
        if let ParameterValue::Int(value) = self.parameters.get(key)
            .chain_err(|| ErrorKind::KeyError(key.to_string(), format!("Key {} was not found", key)))? {
            Ok(*value)
        } else {
            bail!(ErrorKind::KeyError(key.to_string(), format!("Value for key {} was not an integer.", key)))
        }
    }

    /// Get value at given key assuming it is an String value.
    ///
    /// Error if given key does not exist or its value is not an String.
    ///
    /// # Parameters:
    /// * key: Key to search its value.
    ///
    /// # Returns
    /// * Key's value.
    pub fn get_str(&self, key: &'static str)-> Result<String> {
        if let ParameterValue::Str(value) = self.parameters.get(key)
            .chain_err(|| ErrorKind::KeyError(key.to_string(), format!("Key {} was not found", key)))? {
            Ok(value.clone())
        } else {
            bail!(ErrorKind::KeyError(key.to_string(), format!("Value for key {} was not an integer.", key)))
        }
    }

    /// Insert an usize type at given key.
    ///
    /// # Parameters:
    /// * key: Key to insert given value.
    pub fn insert_int(&mut self, key: &'static str, value: usize) {
        self.parameters.insert(key, ParameterValue::Int(value));
    }

    /// Insert an String type at given key.
    ///
    /// # Parameters:
    /// * key: Key to insert given value.
    pub fn insert_str<T>(&mut self, key: &'static str, value: T)
        where T: AsRef<str> {
        self.parameters.insert(key, ParameterValue::Str(value.as_ref().to_string()));
    }
}

/// Iterator through a range from 1 to maximum_key.
struct IntegerKeyIterator {
    start: usize,
    current: usize,
    end: usize
}

impl IntegerKeyIterator {
    fn new(start: usize, end: usize) -> Self {
        IntegerKeyIterator {
            start,
            current: start,
            end
        }
    }
}

impl Iterator for IntegerKeyIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let returned_value = self.current;
            self.current += 1;
            Some(returned_value)
        } else {
            None
        }
    }
}

/// Iterate through every word in our dictionaries.
struct DictionaryWordKeyIterator {
    available_languages: Vec<String>,
    languages_length: usize,
    current_language_index: usize,
    words: Vec<String>,
    words_length: usize,
    current_word_index: usize
}

impl DictionaryWordKeyIterator {
    pub fn new() -> Result<Self> {
        let available_languages = Dictionary::get_dictionaries_names()?;
        let languages_length = available_languages.len();
        let current_language = &available_languages[0];
        let dictionary = Dictionary::new(current_language, false)?;
        let words  = dictionary.get_all_words()?;
        let words_length = words.len();
        Ok(DictionaryWordKeyIterator {
            available_languages,
            languages_length,
            current_language_index: 0,
            words,
            words_length,
            current_word_index:0
        })
    }

    fn get_next_word(&mut self) -> Option<String> {
        let word = self.words[self.current_word_index].clone();
        self.current_word_index += 1;
        return Some(word);
    }
}

impl Iterator for DictionaryWordKeyIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_word_index < self.words_length {
            self.get_next_word()
        } else if self.current_language_index < self.languages_length {
            let current_language = &self.available_languages[self.current_language_index];
            self.current_language_index += 1;
            if let Ok(dictionary) = Dictionary::new(current_language, false) {
                self.words  = match dictionary.get_all_words(){
                    Ok(_words) => _words,
                    Err(E) => return None
                };
                self.words_length = self.words.len();
                self.current_word_index = 0;
                self.get_next_word()
            } else {
                return None
            }
        } else {
            None
        }
    }
}




type GetIdentifiedLanguageTuple = fn(&Parameters) -> Result<(usize, IdentifiedLanguage)>;
type GetString = fn(&Parameters)-> Result<String>;

/// Get ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
///
/// **You should not use this function. Use *brute_force_mp* instead.** This
/// function is slower than *mp* one because is sequential while the other uses a
/// multiprocessing approach. This function only stay here to allow comparisons
/// between sequential and multiprocessing approaches.
///
/// # Parameters:
/// * assess_function: Analysis function to be used.
/// * assess_function_args: Arguments to be used with given *assess_function*. This parameter should
///     have all keys-values needed by assess_function **and** next key-value:
///     * key_space_length: Key space length of cipher to crack.
///
/// # Returns:
/// * Found key.
pub fn brute_force(assess_function: GetIdentifiedLanguageTuple, assess_function_args: &mut Parameters) -> Result<usize> {
    let key_space_length = assess_function_args.get_int("key_space_length")?;
    let mut results: Vec<Result<(usize, IdentifiedLanguage)>> = Vec::new();
    for key in 1..key_space_length {
        assess_function_args.insert_int("key", key);
        results.push(assess_function(&assess_function_args));
    }
    let best_key = get_best_result(&results);
    Ok(best_key)
}

/// Get ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
///
/// **You should use this function instead of *brute_force*.**
///
///  Whereas *brute_force* uses a sequential approach, this function uses
/// multiprocessing to improve performance.
///
///  # Parameters:
/// * assess_function: Analysis function to be used.
/// * assess_function_args: Arguments to be used with given *assess_function*. This parameter should
///     have all keys-values needed by assess_function **and** next key-value:
///     * key_space_length: Key space length of cipher to crack.
///
/// # Returns:
/// * Found key.
pub fn brute_force_mp(assess_function: GetIdentifiedLanguageTuple, assess_function_args: &Parameters) -> Result<usize> {
    let key_space_length = assess_function_args.get_int("key_space_length")?;
    let keys_to_try: Vec<usize> = (1..key_space_length).collect();
    let results: Vec<Result<(usize, IdentifiedLanguage)>> = keys_to_try.par_iter()
        .map(|&key| {
            let mut process_parameters = assess_function_args.clone();
            process_parameters.insert_int("key", key);
            assess_function(&process_parameters)
        })
        .collect();
    let best_key = get_best_result(&results);
    Ok(best_key)
}

/// Decipher text with given key and try to find out if returned text can be identified with any
/// language in our dictionaries.
///
/// # Parameters:
/// * decipher_function: Function to decipher given text.
/// * decipher_function_args: Key to decipher *ciphered_text*.
///
/// # Returns:
/// * A tuple with used key and an *IdentifiedLanguage* object with assessment result.
pub fn assess_key(decipher_function: GetString, decipher_function_args: &Parameters) -> Result<(usize, IdentifiedLanguage)> {
    let deciphered_text = decipher_function(decipher_function_args)?;
    let identified_language = identify_language(deciphered_text)?;
    let used_key = decipher_function_args.get_int("key")?;
    Ok((used_key, identified_language))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::attack::dictionaries::tests::{loaded_dictionary_temp_dir, MicroDictionaries};
    use test_common::fs::tmp::TestEnvironment;
    use test_common::system::env::TemporalEnvironmentVariable;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_integer_key_generator() {
        let expected_result: Vec<usize> = vec![0, 1, 2, 3, 4];
        let generator = IntegerKeyIterator::new(0, 5);
        let returned_result: Vec<usize> = generator.collect();
        assert_eq!(returned_result, expected_result);
    }

    #[rstest]
    fn test_dictionary_word_key_generator(loaded_dictionary_temp_dir: (TestEnvironment, TemporalEnvironmentVariable)) {
        let micro_dictionaries = MicroDictionaries::new();
        let expected_words: Vec<String> = micro_dictionaries._languages
            .keys()
            .flat_map(|key| micro_dictionaries._languages[key].iter())
            .cloned()
            .collect();
        let expected_words_set: HashSet<String> = HashSet::from_iter(expected_words);
        let word_iterator = DictionaryWordKeyIterator::new().unwrap();
        let recovered_words_set: HashSet<String> = HashSet::from_iter(word_iterator);
        assert_eq!(recovered_words_set, expected_words_set);
    }
}