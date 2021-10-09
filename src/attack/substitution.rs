/// Module to attack substitution cipher texts.
///
/// This module uses a word patter matching method to guess probable key used to cipher
/// a text using substitution algorithm.
///
/// You should be aware that to be successful charset used for attack should be the
/// same used to cipher. Besides, this module tries to guess if deciphered text is
/// the good one comparing it with words from a language dictionary. If original
/// message was in a language you don't have a dictionary for, then correct key
/// won/'t be detected.
use crate::{ErrorKind, Result, ResultExt, Error};
use crate::attack::dictionaries::{get_words_from_text, Dictionary, get_word_pattern};
use crate::FromStr;
use crate::cipher::substitution::decipher;
use std::collections::{HashMap, HashSet};
// use std::convert::From;
use std::fmt::Debug;
// use std::fmt;
// use std::iter::FromIterator;
use rayon::prelude::*;

/// Creates a mapping instance using a content description similar to python dicts.
///
/// For instance:
/// ```rust
///     let mut current_mapping = mapping!(TEST_CHARSET,
///                                        {"1" : {"a", "b"},
///                                         "2" : {"c"},
///                                         "3" : {"d"},
///                                         "4" : {"d", "f"},
///                                         "5" : {"c", "h"}});
/// ```
///
/// # Parameters:
/// * charset: Charset used for substitution method. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
/// * content: Python dict like structure whose keys are cipherletters and values are python
///     set like lists with letter candidates.
///
/// # Returns:
/// * A Mapping instance loaded with mapping dict content.
macro_rules! mapping {

        (
            $charset:expr ,
            {
                $($key:tt : {$($value:tt), +}), +
            }
        ) => {
                {
                    let mut mapping_content: HashMap<char, Option<HashSet<char>>> = HashMap::new();
                    $(
                        let values_list = vec![$(char::fromStr($value)), +];
                        let values_iter = values_list.into_iter();
                        mapping_content.insert(char::fromStr($key), Some(HashSet::from_iter(values_iter)));
                      )+
                    let mapping = Mapping::new(&mapping_content, $charset);
                    mapping
                }
        };
        (
            $charset:expr ,
            {
                $($key:tt : {}), +
            }
        ) => {
                {
                    let mut mapping_content: HashMap<char, Option<HashSet<char>>> = HashMap::new();
                    $(
                        mapping_content.insert(char::fromStr($key), None);
                      )+
                    let mapping = Mapping::new(&mapping_content, $charset);
                    mapping
                }
        };
    }

// trait FromStr<T> {
//     fn fromStr(s: T) -> Self;
// }
//
// impl FromStr<&str> for char {
//     fn fromStr(s: &str) -> Self {
//         s.chars().next().expect(format!("Could not create char from given string: {}", s).as_str())
//     }
// }

/// Get substitution ciphered text key.
///
/// Uses a word pattern matching technique to identify used language.
///
/// **You should not use this function. Use *hack_substitution_mp* instead.** This
/// function is slower than *mp* one because is sequential while the other uses a
/// multiprocessing approach. This function only stay here to allow comparisons
/// between sequential and multiprocessing approaches.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for substitution method. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
/// * database_path: Absolute pathname to database file. Usually you don't
///      set this parameter, but it is useful for tests.
///
/// # Returns:
/// * A tuple with substitution key found and success probability.
pub fn hack_substitution<T, U>(ciphered_text: T, charset: U) -> Result<(String, f64)>
    where T: AsRef<str>,
          U: AsRef<str> {
    let ciphered_words = get_words_from_text(&ciphered_text);
    let available_languages = Dictionary::get_dictionaries_names()
        .chain_err(|| ErrorKind::DatabaseError("We could not get dictionaries names."))?;
    let mut keys_found: HashMap<String, f64> = HashMap::new();
    for language in available_languages {
        let language_probabilities = get_keys_probabilities(&ciphered_text, &charset, &ciphered_words, &language)?;
        language_probabilities.iter().for_each(|(key, value)| {
            match keys_found.get(key) {
                Some(previous_value) => {
                    if value > previous_value {
                        keys_found.insert(key.clone(), *value);
                    }
                },
                None => { keys_found.insert(key.clone(), *value); }
            }
        });
    }
    let (best_key, best_probability) = get_best_key(&keys_found);
    Ok((best_key, best_probability))
}

fn get_keys_probabilities<T, U>(ciphered_text: &T, charset: &U, ciphered_words: &HashSet<String>, language: &String) -> Result<HashMap<String, f64>>
    where T: AsRef<str>, U: AsRef<str> {
    let (possible_mappings, _) = get_possible_mappings(&language, &ciphered_words, &charset)?;
    let language_keys = assess_candidate_keys(&ciphered_text, &language,
                                              &possible_mappings, &charset)?;
    Ok(language_keys)
}

/// Get substitution ciphered text key.
///
/// Uses a word pattern matching technique to identify used language.
///
///  **You should use this function instead of *hack_substitution*.**
///
///  Whereas *hack_substitution* uses a sequential approach, this function uses
///  multiprocessing to improve performance.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for substitution method. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
/// * database_path: Absolute pathname to database file. Usually you don't
///      set this parameter, but it is useful for tests.
///
/// # Returns:
/// * A tuple with substitution key found and success probability.
pub fn hack_substitution_mp<T, U>(ciphered_text: T, charset: U) -> Result<(String, f64)>
    where T: AsRef<str> + std::marker::Sync,
          U: AsRef<str> + std::marker::Sync {
    let ciphered_words = get_words_from_text(&ciphered_text);
    let available_languages = Dictionary::get_dictionaries_names()
        .chain_err(|| ErrorKind::DatabaseError("We could not get dictionaries names."))?;
    let mut keys_found: HashMap<String, f64> = HashMap::new();
    let languages_probabilities: Vec<Result<HashMap<String, f64>>> = available_languages.par_iter().map(|language| get_keys_probabilities(&ciphered_text, &charset, &ciphered_words, &language)).collect();
    for language_probability in languages_probabilities {
        let probabilities = language_probability?;
        keys_found.extend(probabilities.into_iter());
    }
    let (best_key, best_probability) = get_best_key(&keys_found);
    Ok((best_key, best_probability))
}

/// Get every possible mapping for given ciphered words in given language.
///
/// # Parameters:
/// * language: Language to compare with ciphered words.
/// * ciphered_words: Words whose patterns needs to be compared with those from language dictionary.
/// * charset: Charset used for substitution method. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Tuple with a Vec of possible mapping found and a string with language name where those
///     mappings where found.
fn get_possible_mappings<T, U, V>(language: T, ciphered_words: &HashSet<U>, charset: V) -> Result<(Vec<Mapping>, String)>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    let mut global_mapping = generate_language_mapping(&language, ciphered_words, &charset)
        .chain_err(|| "Error generating language mapping.")?;
    global_mapping.clean_redundancies();
    let possible_mappings = global_mapping.get_possible_mappings();
    Ok((possible_mappings, language.as_ref().to_string()))
}

/// Generate a mapping with all letter candidates in given language for every cipherletter.
///
/// # Parameters:
/// * language: Language to look letter candidates into.
/// * ciphered_words: Every cipherword in message.
/// * charset: Charset used for substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Mapping loaded with all candidates in given language.
fn generate_language_mapping<T, U, V>(language: T, ciphered_words: &HashSet<U>, charset: V) -> Result<Mapping>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    let mut language_mapping = Mapping::new_empty(&charset);
    let dictionary = Dictionary::new(&language, false)?;
    for ciphered_word in ciphered_words {
        let word_mapping = get_word_mapping(&charset, ciphered_word, &dictionary)?;
        language_mapping.reduce_mapping(&word_mapping);
    }
    Ok(language_mapping)
}

/// Create a mapping with characters candidates for given ciphered word.
///
/// # Parameters:
/// * charset: Charset used for substitution method. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
/// * ciphered_word: Ciphered word used to find words with similar patterns.
/// * dictionary: Dictionary to extract from words with the same pattern than ciphered word.
///
/// # Returns:
/// * A Mapping class instance.
fn get_word_mapping<T, U>(charset: T, ciphered_word: U, dictionary: &Dictionary) -> Result<Mapping>
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut word_mapping = Mapping::new_empty(&charset);
    let ciphered_word_pattern: String = get_word_pattern(&ciphered_word);
    let word_candidates = dictionary.get_words_with_pattern(&ciphered_word_pattern)
        .chain_err(|| ErrorKind::NoMappingAvailable(ciphered_word.as_ref().to_string(), dictionary.language.clone()))?;
    for (index, char) in ciphered_word.as_ref().chars().enumerate() {
        // let char_string = char.to_string();
        for word_candidate in word_candidates.iter() {
            if let Some(selected_char) = word_candidate.chars().nth(index) {
                word_mapping.add(char, selected_char);
                // word_mapping.add(&char.to_string(), selected_char.to_string());
            }

        }
    }
    Ok(word_mapping)
}

/// Assess every possible mapping and get how many recovered words are identifiable
/// in any language dictionary.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * language: Language to compare with recovered words.
/// * possible_mappings: Possible cipherletter mappings for given text.
/// * charset: Charset used for substitution method. Both ends, ciphering
///    and deciphering, should use the same charset or original text won't be properly
///    recovered.
///
/// # Returns:
/// * A HashMap whose keys are tested keys and values are a 0 to 1 float with
///   comparison sucess for given language. 1 means every deciphered word using
///   tested key can be found in given language dictionary.
fn assess_candidate_keys<T, U, V>(ciphered_text: T, language: U,
                                  possible_mappings: &Vec<Mapping>, charset: V) -> Result<HashMap<String, f64>>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    let mut keys_found: HashMap<String, f64> = HashMap::new();
    for possible_mapping in possible_mappings {
        match assess_possible_mapping(possible_mapping, &language, &ciphered_text, &charset) {
            Ok((key, probability)) => { keys_found.insert(key, probability); },
            Err(e) => match e {
                Error(ErrorKind::WrongKeyLength(_, _), _) => continue,
                Error(ErrorKind::WrongKeyRepeatedCharacters(_), _) => continue,
                error => bail!(error)
            }
        };
    }
    Ok(keys_found)
}

/// Convert mapping to a substitution key and check if that key deciphers messages in words
/// from any know dictionary.
///
/// # Parameters:
/// * possible_mapping: Mapping reduced to maximum.
/// * language: Language to compare with recovered words.
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for substitution method. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
///
/// # Returns:
/// * A tuple with key generated from given mapping and a 0 to 1 float with
///     comparison success for given language. 1 means every deciphered word using
///     tested key can be found in given language dictionary.
fn assess_possible_mapping<T, U, V>(possible_mapping: &Mapping, language: T, ciphered_text: U,
                                    charset: V) -> Result<(String, f64)>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    let key = possible_mapping.generate_key_string();
    let success = assess_substitution_key(&ciphered_text, &key, &language, &charset)?;
    Ok((key, success))
}

/// Decipher text with given key and try to find out if returned text can be identified with given
/// language.
///
/// If given key does not comply with coherence rules then it is silently discarded
/// returning 0.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Key to decipher *ciphered_text*.
/// * language: Language to compare got text.
/// * charset: Charset used for substitution. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
/// # Returns:
/// * Float from 0 to 1. The higher the frequency of presence of words in language
///      the higher of this probability.
fn assess_substitution_key<T, U, V, W>(ciphered_text: T, key: U, language: V, charset: W) -> Result<f64>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str>,
          W: AsRef<str> {
    let recovered_text = decipher(&ciphered_text, &key, &charset)?;
    let words = get_words_from_text(&recovered_text);
    let frequency = get_candidates_frequency_at_language(&words, &language);
    frequency
}

/// Get frequency of presence of words in given language.
///
/// # Parameters:
/// * words: Text words.
/// * language: Language you want to look into.
///
/// # Returns:
/// * Float from 0 to 1. The higher the frequency of presence of words in language
///     the higher of this probability.
fn get_candidates_frequency_at_language<T>(words: &HashSet<String>, language: T) -> Result<f64>
    where T: AsRef<str> {
    let dictionary = Dictionary::new(language.as_ref(), false)?;
    let frequency = dictionary.get_words_presence(&words);
    Ok(frequency)
}


/// Get key with maximum probability
///
/// # Parameters:
/// * keys_found: Dict with cipher keys as dict keys and their corresponding probabilities as float values.
///
/// # Returns:
/// * Tuple with best key and its corresponding probability.
fn get_best_key(keys_found: &HashMap<String, f64>)-> (String, f64){
    let mut best_probability: f64 = 0.0;
    let mut best_key = String::new();
    for (key, value) in keys_found {
        if *value > best_probability {
            best_probability = *value;
            best_key = key.clone();
        }
    }
    (best_key, best_probability)
}

/// Type to manage possible candidates to substitute every cipherletter in charset.
///
/// You can use it as a dict whose keys are letters and values are sets with substitution
/// letters candidates.
#[derive(Debug)]
struct Mapping {
    mapping: HashMap<char, Option<HashSet<char>>>,
    charset: String
}

impl Mapping {

    /// Create empty mapping for cipher letters
    ///
    /// # Parameters:
    /// * charset: Charset used for substitution method. Both ends, ciphering
    ///     and deciphering, should use the same charset or original text won't be properly
    ///     recovered.
    fn init_mapping(&mut self){
        for char in self.charset.chars() {
            self.mapping.insert(char, None);
        }
    }

    /// Create a mapping with all character mappings empty.
    ///
    /// # Parameter:
    /// * charset: Charset used for substitution method. Both ends, ciphering
    ///     and deciphering, should use the same charset or original text won't be properly
    ///     recovered.
    ///
    /// # Returns:
    /// * An empty Mapping instance.
    pub fn new_empty<T>(charset: T) -> Self
        where T: AsRef<str> {
        let mut mapping = Self {
            mapping: HashMap::new(),
            charset: charset.as_ref().to_string()
        };
        mapping.init_mapping();
        mapping
    }

    /// Create a mapping loaded with given mapping dict.
    ///
    /// # Parameters:
    /// * mapping_dict: Content to load.
    /// * charset: Charset used for substitution method. Both ends, ciphering
    ///      and deciphering, should use the same charset or original text won't be properly
    ///      recovered.
    ///
    /// # Returns:
    /// * A Mapping instance loaded with mapping dict content.
    pub fn new<T>(mapping_dict: &HashMap<char, Option<HashSet<char>>>, charset: T)-> Self
        where T: AsRef<str> {
        let mut mapping = Self::new_empty(charset);
        mapping.load_content(mapping_dict);
        mapping
    }

    /// Populates this mapping using a HashMap.
    ///
    /// HashMaps's keys are cipherletters and values are sets of mapping char candidates.
    ///
    /// Given mapping should use the same charset as this one. Differing cipherletters
    /// will be discarded.
    ///
    /// # Parameters:
    /// * mapping_dict: Content to load.
    fn load_content(&mut self, mapping_dict: &HashMap<char, Option<HashSet<char>>>) {
        let keys_list: Vec<&char> = mapping_dict.keys().map(|x| x).collect();
        for (key, value) in mapping_dict.iter() {
            if keys_list.contains(&key){
                match value {
                    Some(mapping_set) => {
                        let mapping_set_clone: HashSet<char> = mapping_set.iter().map(|x| *x).collect();
                        self.mapping.insert(*key, Some(mapping_set_clone));
                    },
                    None =>  {  }
                }
            }
        }
    }

    /// Get current mapping content.
    ///
    /// # Returns:
    /// * Dict's keys are cipherletters and values are sets of mapping char candidates.
    fn get_current_content(&self)-> &HashMap<char, Option<HashSet<char>>> {
        &self.mapping
    }

    /// Get this mapping cipherletters.
    ///
    /// # Returns:
    /// * A list with cipherletters registered in this mapping.
    fn cipherletters(&self)-> Vec<char>{
        let cipherletters_list: Vec<char> = self.mapping.keys().cloned().collect();
        cipherletters_list
    }

    /// Generate an string to be used as a substitution key.
    ///
    /// If any cipherletter has no substitutions alternative then the same cipherletter
    /// is used for substitution. Also, be aware that first candidate for every
    /// cipherletter will be chosen so use this method when mapping is completely
    /// reduced.
    ///
    /// # Returns:
    /// * Generated key string.
    fn generate_key_string(&self)-> String {
        let mut key_list: Vec<char> = Vec::new();
        for clear_char in self.charset.chars() {
            let mut char_found = false;
            for (&key, value_set) in self.mapping.iter() {
                match value_set {
                    Some(set) => {
                        // Use this method with already reduced mappings because only
                        // first element of every set will be taken.
                        let value = set.get_first_element().unwrap();
                        if value == clear_char {
                            char_found = true;
                            key_list.push(key);
                            break;
                        }
                    },
                    None => continue
                }
            }
            if !char_found {
                key_list.push(clear_char);
            }
        }
        let mut string_to_return = String::new();
        key_list.iter().for_each(|x| string_to_return.push_str(x.to_string().as_str()));
        string_to_return
    }

    /// Return every possible mapping from an unresolved mapping.
    ///
    /// An unresolved mapping is one that has more than one possibility in any of
    /// its chars.
    ///
    /// # Parameters:
    /// * mapping: A character mapping.
    ///
    /// # Returns:
    /// * A list of mapping candidates.
    fn get_possible_mappings(&self)-> Vec<Mapping> {
        self._get_possible_mappings(None)
    }

    /// Utility recursive method used by get_possible_mappings().
    ///
    /// # Parameters:
    /// * mapping: A character mapping.
    ///
    /// # Returns:
    /// * A list of mapping candidates.
    fn _get_possible_mappings(&self, mapping: Option<&Mapping>)-> Vec<Mapping> {
        let mut mapping_list: Vec<Mapping> = Vec::new();
        let mut step_mapping = match mapping {
            None => Mapping::new(self.get_current_content(), &self.charset),
            Some(start_mapping) => start_mapping.clone()
        };
        if let Ok((char, candidates)) = step_mapping.pop_item() {
            let partial_mappings = self._get_possible_mappings(Some(&mut step_mapping));
            match candidates {
                Some(set) => {
                    for candidate in set.iter() {
                        for partial_mapping in partial_mappings.iter() {
                            let mut current_mapping = Mapping::new_empty(&self.charset);
                            current_mapping.add(char, *candidate);
                            current_mapping.load_content(partial_mapping.get_current_content());
                            mapping_list.push(current_mapping);
                        }
                    }
                },
                None => {
                    for partial_mapping in partial_mappings.iter() {
                        let char_string = char.to_string();
                        let char_str = char_string.as_str();
                        let mut current_mapping = mapping!(&self.charset, {char_str : {}});
                        current_mapping.load_content(partial_mapping.get_current_content());
                        mapping_list.push(current_mapping);
                    }
                }
            };
            return mapping_list
        } else {
            return vec![Mapping::new_empty(&self.charset),];
        }
    }

    /// Apply given word mapping to reduce this mapping.
    ///
    /// # Parameters:
    /// * word_mapping: Partial mapping for an individual word.
    fn reduce_mapping(&mut self, word_mapping: &Mapping) {
        for cipherletter in self.cipherletters()  {
            // Unwrap here is safe because we are using cipherletters.
            if let Some(set) = self.get(cipherletter).unwrap() {
                // Previous candidates present for cipherletter so reducing needed.
                if let Some(word_cipherletters_mapping_option) = word_mapping.get(cipherletter) {
                    match word_cipherletters_mapping_option {
                        Some(word_cipherletter_mapping) => {
                            let new_candidates_set: HashSet<char> = set.intersection(word_cipherletter_mapping).map(|x| *x).collect();
                            self.set(cipherletter, Some(new_candidates_set));
                        },
                        None => {}
                    };
                }
            } else {
                // No previous candidates present for cipherletter so just copy word mapping.
                if let Some(word_cipherletters_mapping_option) = word_mapping.get(cipherletter) {
                    match word_cipherletters_mapping_option {
                        Some(word_cipherletter_mapping) => {
                            self.set(cipherletter, Some(word_cipherletter_mapping.clone()));
                        },
                        None => {}
                    };
                }
            }
        }
    }

    /// Remove redundancies from mapping.
    ///
    /// If any cipherletter has been reduced to just one candidate, then that
    /// candidate should not be in any other cipherletter. Leaving it would produce
    /// an inconsistent deciphering key with repeated characters.
    pub fn clean_redundancies(&mut self){
        let candidates_to_remove: Vec<char> = self.mapping.values()
            .filter(|&x|
                if let Some(set) = x {
                    if set.len() == 1 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                })
            .map(|x| {
                let set: &HashSet<char> = x.as_ref().unwrap();
                // Unwrap is not dangerous here because we filtered to be sure set has at least 1 element.
                set.get_first_element().unwrap()
            })
            .collect();
        let keys_to_check: Vec<char> = self.mapping.keys().cloned()
            .filter(|x|
                if let Some(Some(set)) = self.mapping.get(x) {
                    if set.len() > 1 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                })
            .collect();
        for key_to_check in keys_to_check {
            let set_option = self.mapping.get_mut(&key_to_check).unwrap();
            let set = set_option.as_mut().unwrap();
            set.retain(|x| !candidates_to_remove.contains(x))
        }
    }

    /// Get candidates for given cipherletter.
    ///
    /// If the mapping did not have this cipherletter present, [`None`] is returned.
    ///
    /// # Parameters:
    /// * key: Cipherletter to get candidates from.
    ///
    /// # Returns:
    /// * Current candidates set or None if cipherletter is not present.
    fn get(&self, key: char) -> Option<&Option<HashSet<char>>> {
        self.mapping.get(&key)
    }

    /// Inserts a cipherletter-candidates pair into the mappping.
    ///
    /// If the mapping did not have this cipherletter present, [`None`] is returned, but key and
    /// value are inserted.
    ///
    /// If the mappping did have this cipherletter present, the value is updated, and the old
    /// value is returned. The key is not updated, though.
    ///
    /// # Parameters:
    /// * key: Cipherletter to update.
    /// * value: New value to insert.
    ///
    /// # Returns:
    /// * Old value or None if key was not found.
    fn set(&mut self, key: char, value: Option<HashSet<char>>) -> Option<Option<HashSet<char>>> {
        self.mapping.insert(key, value)
    }

    /// Remove and return a cipherletter and its candidates from current mapping.
    ///
    /// # Returns:
    /// * A tuple with selected cipherletter and its candidates.
    fn pop_item(&mut self) -> Result<(char, Option<HashSet<char>>)> {
        if self.mapping.keys().len() >= 1 {
            let cipherletters: Vec<char> = self.mapping.keys().cloned().take(1).collect();
            if let Some(cipherletter) = cipherletters.get(0) {
                let set = self.mapping.remove(&cipherletter).unwrap();
                Ok((*cipherletter, set))
            } else {
                Err(ErrorKind::EmptyMapping.into())
            }
        } else {
            Err(ErrorKind::EmptyMapping.into())
        }
    }

    /// Insert a new candidate into an existing mapping.
    ///
    /// Whereas set() assigns an entire HashSet to cipherletter, this method only adds a new candidate
    /// to existing cipherletter.
    ///
    /// # Parameters:
    /// * key: Cipherletter to update.
    /// * value: Candidate to insert.
    // fn add<T, U>(&mut self, key: T, value: U)
    //     where T: AsRef<str>,
    //           U: AsRef<str> {
    fn add(&mut self, key: char, value: char) {
        let entry = self.mapping.entry(key).or_insert(None);
        match entry {
            Some(content) => {
                content.insert(value);
            },
            None => {
                let mut new_content: HashSet<char> = HashSet::new();
                new_content.insert(value);
                *entry = Some(new_content);
            }
        };
    }

    /// Create a new set at given cipherletter just with one candidate.
    ///
    /// # Parameters:
    /// * key: Cipherletter to update.
    /// * value: Candidate to insert.
    fn create_new_single_entry(&mut self, key: char, value: char) {
        let mut new_candidates_set: HashSet<char> = HashSet::new();
        new_candidates_set.insert(value);
        self.mapping.insert(key, Some(new_candidates_set));
    }
}

impl PartialEq for Mapping {
    fn eq(&self, other: &Self) -> bool {
        if self.charset == other.charset && self.mapping == other.mapping {
            true
        } else {
            false
        }
    }
}

impl Clone for Mapping {
    fn clone(&self) -> Self {
        Self {
            mapping: self.mapping.clone(),
            charset: self.charset.clone()
        }
    }
}

trait Extractor {
    type Item;

    /// Get first N elements from collections.
    ///
    /// # Parameters:
    /// * n: How many elements to return.
    ///
    /// # Returns:
    /// * A list of elements.
    fn get_n_elements(&self, n: usize) -> Option<Vec<Self::Item>>;

    /// Get first element from collections.
    ///
    /// # Returns:
    /// * An element.
    fn get_first_element(&self) -> Option<Self::Item>;
}

impl Extractor for HashSet<char> {

    type Item = char;

    fn get_n_elements(&self, n: usize) -> Option<Vec<char>> {
        let mut returned_elements: Vec<char> = Vec::new();
        for element in self.iter() {
            returned_elements.push(*element);
            if returned_elements.len() >= n {
                return Some(returned_elements);
            }
        }
        None
    }

    fn get_first_element(&self) -> Option<Self::Item> {
        if let Some(elements_list) = self.get_n_elements(1) {
            if let Some(first_element) = elements_list.get(0) {
                return Some(*first_element);
            } else {
                return None;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::time::Instant;
    use crate::attack::dictionaries::tests::LoadedDictionaries;
    use crate::cipher::substitution::{cipher, decipher};
    use std::io::Read;
    use std::path::PathBuf;
    use std::iter::FromIterator;


    const TEST_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const TEST_KEY: &'static str =     "lfwoayuisvkmnxpbdcrjtqeghz";
    const TEST_CHARSET_SPANISH: &'static str = "abcdefghijklmnopqrstuvwxyzáéíóúñ";
    const TEST_KEY_SPANISH: &'static str =     "lfwoayuisvkmnxpbdcrjtqeghzñúóíéá";
    const ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS: &'static str = "resources/english_book_c1.txt";
    const SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS: &'static str = "resources/spanish_book_c1.txt";

    struct TestSet {
        text_file: &'static str,
        language: &'static str,
        key: &'static str,
        charset: &'static str
    }

    impl TestSet {
        fn new(text_file: &'static str, language: &'static str,
               key: &'static str, charset: &'static str)-> Self {
            Self {
                text_file,
                language,
                key,
                charset
            }
        }
    }

    /// Creates a candidates set valid to assigned to a Mapping key.
    ///
    /// For instance:
    /// ```rust
    ///     let mut current_mapping = mapping!(TEST_CHARSET,
    ///                                        {"1" : {"a", "b"},
    ///                                         "2" : {"c"},
    ///                                         "3" : {"d"},
    ///                                         "4" : {"d", "f"},
    ///                                         "5" : {"c", "h"}});
    ///     current_mapping["4"] = candidates!("r", "p", "x");
    /// ```
    ///
    /// # Parameters:
    /// * A list of &str chars to be included as candidates.
    ///
    /// # Returns:
    /// * A HashSet ready to be assigned to a Mapping key.
    macro_rules! candidates {
        (
            $($value:tt), +
        ) => {
            {
                let mut candidates_set = HashSet::new();
                $(
                    candidates_set.insert(char::fromStr($value));
                 )+
                Some(candidates_set)
            }
        };
    }

    #[test]
    fn test_hack_substitution() {
        let test_sets = vec![
            TestSet::new(ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS, "english", TEST_KEY, TEST_CHARSET),
            TestSet::new(SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS, "spanish", TEST_KEY_SPANISH, TEST_CHARSET_SPANISH)
        ];
        let loaded_dictionaries = LoadedDictionaries::new();
        for set in test_sets {
            let text = get_text_to_cipher(&set);
            let ciphered_text = match cipher(&text, &set.key, &set.charset) {
                Ok(text) => text,
                Err(E) => {assert!(false, E); String::new()}
            };
            let timer = Instant::now();
            let found_key = hack_substitution(&ciphered_text, &set.charset)
                .expect("Error running hacking_substitution().");
            assert_found_key(&found_key, &set.key, &ciphered_text,
                             &text, &set.charset);
            println!("{}", format!("\n\nElapsed time with hack_substitution: {:.2} seconds.", timer.elapsed().as_secs_f64()));
        }
    }

    #[test]
    fn test_hack_substitution_mp() {
        let test_sets = vec![
            TestSet::new(ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS, "english", TEST_KEY, TEST_CHARSET),
            TestSet::new(SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS, "spanish", TEST_KEY_SPANISH, TEST_CHARSET_SPANISH)
        ];
        let loaded_dictionaries = LoadedDictionaries::new();
        for set in test_sets {
            let text = get_text_to_cipher(&set);
            let ciphered_text = match cipher(&text, &set.key, &set.charset) {
                Ok(text) => text,
                Err(E) => {assert!(false, E); String::new()}
            };
            let timer = Instant::now();
            let found_key = hack_substitution_mp(&ciphered_text, &set.charset)
                .expect("Error running hacking_substitution().");
            assert_found_key(&found_key, &set.key, &ciphered_text,
                             &text, &set.charset);
            println!("{}", format!("\n\nElapsed time with hack_substitution: {:.2} seconds.", timer.elapsed().as_secs_f64()));
        }
    }

    fn get_text_to_cipher(set: &TestSet) -> String {
        let mut text_file_pathname = match env::current_dir() {
            Ok(cwd) => cwd,
            Err(E) => {assert!(false, E); PathBuf::new()}
        };
        text_file_pathname.push(set.text_file);
        let mut text_file = match File::open(&text_file_pathname) {
            Ok(file) => file,
            Err(E) => {assert!(false, E); File::create("/tmp").unwrap()}
        };
        let mut text = String::new();
        match text_file.read_to_string(&mut text) {
            Ok(_) => (),
            Err(E) => {assert!(false, E); ()}
        }
        text
    }


    fn assert_found_key<U, V, W, X>(found_key: &(String, f64), tested_key: U, ciphered_text: V,
                              original_text: W, charset: X)
        where U: AsRef<str>,
              V: AsRef<str>,
              W: AsRef<str>,
              X: AsRef<str> {
        assert_eq!(found_key.0, tested_key.as_ref());
        let deciphered_text = match decipher(ciphered_text.as_ref(), &found_key.0,
                                             charset.as_ref()) {
            Ok(text) => text,
            Err(E) => {assert!(false, E); String::new()}
        };
        assert_eq!(deciphered_text, original_text.as_ref());
    }

    #[test]
    fn test_clean_redundancies() {
        let mut current_mapping = mapping!(TEST_CHARSET,
                                                    {"1" : {"a", "b"},
                                                     "2" : {"c"},
                                                     "3" : {"d"},
                                                     "4" : {"d", "f"},
                                                     "5" : {"c", "h"}});
        let expected_mapping = mapping!(TEST_CHARSET,
                                        {"1" : {"a", "b"},
                                         "2" : {"c"},
                                         "3" : {"d"},
                                         "4" : {"f"},
                                         "5" : {"h"}});
        current_mapping.clean_redundancies();
        assert_eq!(expected_mapping, current_mapping)
    }

    #[test]
    fn test_generate_key_string() {
        let expected_keystring = "ABCDEFGHIJKLMNOPQRSTUVWXYZfghijfghijklmnopqrstuvwxyz";
        let test_charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mapping = mapping!(test_charset,
                                                {"f": {"a"},
                                                   "g": {"b"},
                                                   "h": {"c"},
                                                   "i": {"d"},
                                                   "j": {"e"}});
        let returned_keystring = mapping.generate_key_string();
        assert_eq!(expected_keystring, returned_keystring)
    }

    #[test]
    fn test_get_n_elements() {
        let mut set: HashSet<char> = HashSet::new();
        set.insert(char::fromStr("a"));
        set.insert(char::fromStr("b"));
        set.insert(char::fromStr("c"));
        match set.get_n_elements(2) {
            Some(list) => {
                assert_eq!(list.len(), 2);
            },
            None => {
                assert!(false, "No element was extracted.");
            }
        }
    }

    #[test]
    fn test_get_first_element() {
        let mut set: HashSet<char> = HashSet::new();
        set.insert(char::fromStr("a"));
        match set.get_first_element() {
            Some(element) => {
                assert_eq!(element, char::fromStr("a"));
            },
            None => {
                assert!(false, "No element was extracted.");
            }
        }
    }


    #[test]
    fn test_get_possible_mappings() {
        let mapping = mapping!(TEST_CHARSET,
                                {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f"},
                                               "5": {"g", "h"}});
        let expected_list = vec![
            mapping!(TEST_CHARSET, {"1": {"a"},
                              "2": {"c"},
                              "3": {"d"},
                              "4": {"e"},
                              "5": {"g"}}),
            mapping!(TEST_CHARSET, {"1": {"a"},
                                    "2": {"c"},
                                    "3": {"d"},
                                    "4": {"f"},
                                    "5": {"g"}}),

            mapping!(TEST_CHARSET, {"1": {"b"},
                                      "2": {"c"},
                                      "3": {"d"},
                                      "4": {"e"},
                                      "5": {"g"}}),
            mapping!(TEST_CHARSET, {"1": {"b"},
                                        "2": {"c"},
                                        "3": {"d"},
                                        "4": {"f"},
                                        "5": {"g"}}),
            mapping!(TEST_CHARSET, {"1": {"a"},
                              "2": {"c"},
                              "3": {"d"},
                              "4": {"e"},
                              "5": {"h"}}),
            mapping!(TEST_CHARSET, {"1": {"a"},
                                    "2": {"c"},
                                    "3": {"d"},
                                    "4": {"f"},
                                    "5": {"h"}}),
            mapping!(TEST_CHARSET, {"1": {"b"},
                                      "2": {"c"},
                                      "3": {"d"},
                                      "4": {"e"},
                                      "5": {"h"}}),
            mapping!(TEST_CHARSET, {"1": {"b"},
                                        "2": {"c"},
                                        "3": {"d"},
                                        "4": {"f"},
                                        "5": {"h"}}),
        ];
        let recovered_mappings = mapping.get_possible_mappings();
        assert_eq!(expected_list.len(), recovered_mappings.len());
        assert!(expected_list.iter().all(|_mapping| recovered_mappings.contains(&_mapping)));
    }

    #[test]
    fn test_get_possible_mappings_with_empties() {
        let THIS_TEST_CHARSET = "12345";
        let mut mapping = mapping!(THIS_TEST_CHARSET,
                                {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f"},
                                               "5": {"g", "h"}});
        mapping.set(char::fromStr("0"), None);
        let mut expected_mapping_1 = mapping!(THIS_TEST_CHARSET, {"1": {"a"},
                              "2": {"c"},
                              "3": {"d"},
                              "4": {"e"},
                              "5": {"g"}});
        let mut expected_mapping_2 =  mapping!(THIS_TEST_CHARSET, {"1": {"a"},
                                    "2": {"c"},
                                    "3": {"d"},
                                    "4": {"f"},
                                    "5": {"g"}});
        let mut expected_mapping_3 = mapping!(THIS_TEST_CHARSET, {"1": {"b"},
                                      "2": {"c"},
                                      "3": {"d"},
                                      "4": {"e"},
                                      "5": {"g"}});
        let mut expected_mapping_4 =  mapping!(THIS_TEST_CHARSET, {"1": {"b"},
                                        "2": {"c"},
                                        "3": {"d"},
                                        "4": {"f"},
                                        "5": {"g"}});
        let mut expected_mapping_5 = mapping!(THIS_TEST_CHARSET, {"1": {"a"},
                              "2": {"c"},
                              "3": {"d"},
                              "4": {"e"},
                              "5": {"h"}});
        let mut expected_mapping_6 = mapping!(THIS_TEST_CHARSET, {"1": {"a"},
                                    "2": {"c"},
                                    "3": {"d"},
                                    "4": {"f"},
                                    "5": {"h"}});
        let mut expected_mapping_7 = mapping!(THIS_TEST_CHARSET, {"1": {"b"},
                                      "2": {"c"},
                                      "3": {"d"},
                                      "4": {"e"},
                                      "5": {"h"}});
        let mut expected_mapping_8 =  mapping!(THIS_TEST_CHARSET, {"1": {"b"},
                                        "2": {"c"},
                                        "3": {"d"},
                                        "4": {"f"},
                                        "5": {"h"}});
        let mut expected_list = vec![
            expected_mapping_1,
            expected_mapping_2,
            expected_mapping_3,
            expected_mapping_4,
            expected_mapping_5,
            expected_mapping_6,
            expected_mapping_7,
            expected_mapping_8,
        ];
        let recovered_mappings = mapping.get_possible_mappings();
        assert_eq!(expected_list.len(), recovered_mappings.len());
        let missing: Vec<Mapping> = expected_list.iter().cloned().filter(|_mapping| !recovered_mappings.contains(&_mapping)).collect();
        assert!(expected_list.iter().all(|_mapping| recovered_mappings.contains(&_mapping)));
    }

    #[test]
    fn test_reduce_mapping() {
        let mut mapping = mapping!(TEST_CHARSET,
                                            {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f", "g"},
                                               "5": {"h"}});
        let mapping_2 = mapping!(TEST_CHARSET,
                                            {"1": {"a"},
                                                 "2": {"c"},
                                                 "4": {"e", "g"},
                                                 "5": {"h"}});
        let expected_reduced_mapping = mapping!(TEST_CHARSET,
                                                {"1": {"a"},
                                                    "2": {"c"},
                                                    "3": {"d"},
                                                    "4": {"e", "g"},
                                                    "5": {"h"}});
        mapping.reduce_mapping(&mapping_2);
        assert_eq!(mapping, expected_reduced_mapping,
                   "Mapping was not reduced as expected.");
    }

    #[test]
    fn test_mapping_get() {
        let mut mapping = mapping!(TEST_CHARSET,
                                            {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f", "g"},
                                               "5": {"h"}});
        let content = mapping.get(char::fromStr("2")).unwrap().as_ref().expect("Error retrieving key.");
        let content_string = content.get_first_element().expect("Error retrieving content.");
        assert_eq!(char::fromStr("c"), content_string);
    }

    #[test]
    fn test_mapping_set() {
        let mut mapping = mapping!(TEST_CHARSET,
                                            {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f", "g"},
                                               "5": {"h"}});
        mapping.set(char::fromStr("4"), candidates!("r", "t"));
        let content = mapping.get(char::fromStr("4")).unwrap().as_ref().expect("Error retrieving key.");
        let content_list = content.get_n_elements(2).expect("Error retrieving content.");
        assert!(vec!["r", "t"].iter().all(|candidate| content_list.contains(&char::fromStr(candidate))));
    }

    #[test]
    fn test_mapping_add() {
        let mut mapping = mapping!(TEST_CHARSET,
                                            {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f", "g"},
                                               "5": {"h"}});
        mapping.add(char::fromStr("4"), char::fromStr("x"));
        let content = mapping.get(char::fromStr("4")).unwrap().as_ref().expect("Error retrieving key.");
        let expected_length: usize = 4;
        assert_eq!(expected_length, content.len(),
                   "Content has {} while we were expecting {}.",
                   content.len(), expected_length);
        let content_list = content.get_n_elements(expected_length).expect("Error retrieving content.");
        assert!(vec!["e", "f", "g", "x"].iter().all(|candidate| content_list.contains(&char::fromStr(candidate))));

    }

    #[test]
    fn test_popitem() {
        let mut mapping = mapping!(TEST_CHARSET,
                                            {"1": {"a", "b"},
                                               "2": {"c"},
                                               "3": {"d"},
                                               "4": {"e", "f", "g"},
                                               "5": {"h"}});
        // Test correct item extraction.
        let original_content = mapping.get_current_content().clone();
        let original_keys: Vec<&char> = original_content.keys().collect();
        let (extracted_cipherletter, extracted_candidates) = mapping.pop_item()
            .expect("Error extracting item.");
        assert!(original_keys.contains(&&extracted_cipherletter),
                format!("Extracted key {} was not among original ones.", &extracted_cipherletter));
        // Test extraction reduces length.
        let resulting_keys = mapping.cipherletters();
        let original_keys_length = original_keys.len().to_string();
        let resulting_keys_length = resulting_keys.len().to_string();
        assert_eq!(resulting_keys.len(), original_keys.len() - 1,
                   "Original keys length of {} is {} after pop",
                   original_keys_length.as_str(),
                   resulting_keys_length.as_str());
        assert!(!resulting_keys.contains(&extracted_cipherletter),
                format!("Extracted cipherletter {} was not removed from mapping", &extracted_cipherletter));
        // Test extraction from empty mapping generates an error.
        mapping = Mapping {
            mapping: HashMap::new(),
            charset: "".to_string()
        };
        if let Err(E) = mapping.pop_item() {
            match Error::from(E) {
                Error(ErrorKind::EmptyMapping, _) => assert!(true),
                error => assert!(false, format!("Raised error was not the one \
                                          we were expecting but {} instead", error))
            }
        } else { assert!(false, "No error was raised when extracting from empty mapping.") }
    }
}