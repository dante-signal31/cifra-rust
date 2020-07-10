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
use crate::attack::dictionaries::{get_words_from_text, Dictionary};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::hash::Hash;

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
        let (possible_mappings, _) = get_possible_mapping(&language, &ciphered_words, &charset)?;
        let language_keys = assess_candidate_keys(&ciphered_text, &language,
                                                  &possible_mappings, &charset);
        keys_found.extend(language_keys);
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
fn get_possible_mapping<T, U, V>(language: T, ciphered_words: &HashSet<U>, charset: V) -> Result<(Vec<Mapping>, String)>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    unimplemented!()
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
                                  possible_mappings: &Vec<Mapping>, charset: V) -> HashMap<String, f64>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    unimplemented!()
}

/// Get key with maximum probability
///
/// # Parameters:
/// * keys_found: Dict with cipher keys as dict keys and their corresponding probabilities as float values.
///
/// # Returns:
/// * Tuple with best key and its corresponding probability.
fn get_best_key(keys_found: &HashMap<String, f64>)-> (String, f64){
    unimplemented!()
}

/// Type to manage possible candidates to substitute every cipherletter in charset.
///
/// You can use it as a dict whose keys are letters and values are sets with substitution
/// letters candidates.
#[derive(Debug)]
struct Mapping {
    mapping: HashMap<String, Option<HashSet<String>>>,
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
            self.mapping.insert(char.to_string(), None);
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
    pub fn new<T, U, V>(mapping_dict: &HashMap<T, Option<HashSet<U>>>, charset: V)-> Self
        where T: AsRef<str>,
              U: AsRef<str>,
              V: AsRef<str> {
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
    fn load_content<T, U>(&mut self, mapping_dict: &HashMap<T, Option<HashSet<U>>>)
        where T: AsRef<str>,
              U: AsRef<str> {
        for (key, value) in mapping_dict.iter() {
            match value {
                Some(mapping_set) => {
                    self.mapping.insert(key.as_ref().to_string(), Some(HashSet::new()));
                    for mapping in mapping_set {
                        if let Some(Some(value)) = self.mapping.get_mut(key.as_ref()) {
                            value.insert(mapping.as_ref().to_string());
                        }
                    }
                },
                None =>  {self.mapping.insert(key.as_ref().to_string(), None); }
            }
        }
    }

    /// Get current mapping content.
    ///
    /// # Returns:
    /// * Dict's keys are cipherletters and values are sets of mapping char candidates.
    fn get_current_content(&self)-> &HashMap<String, Option<HashSet<String>>> {
        &self.mapping
    }

    /// Get this mapping cipherletters.
    ///
    /// # Returns:
    /// * A list with cipherletters registered in this mapping.
    fn cipherletters(&self)-> Vec<String>{
        let cipherletters_list: Vec<String> = self.mapping.keys().cloned().collect();
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
        unimplemented!()
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
    fn get_possible_mappings(&self, mapping: Option<&Mapping>)-> Vec<Mapping> {
        unimplemented!()
    }

    /// Apply given word mapping to reduce this mapping.
    ///
    /// # Parameters:
    /// * word_mapping: Partial mapping for an individual word.
    fn reduce_mapping(&mut self, world_mapping: &Mapping) {
        unimplemented!()
    }

    /// Remove redundancies from mapping.
    ///
    /// If any cipherletter has been reduced to just one candidate, then that
    /// candidate should not be in any other cipherletter. Leaving it would produce
    /// an inconsistent deciphering key with repeated characters.
    fn clean_redundancies(&mut self){
        let candidates_to_remove: Vec<String> = self.mapping.values()
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
                let set: &HashSet<String> = x.as_ref().unwrap();
                let mut string: String = String::new();
                for element in set.iter() {
                    // Actually I just want first element from set.
                    string = element.clone();
                    break;
                }
                string
            })
            .collect();
        let keys_to_check: Vec<String> = self.mapping.keys().cloned()
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

// impl Debug for Mapping {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         unimplemented!()
//     }
// }

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
                        let mut mapping_content = HashMap::new();
                        $(
                            let values_list = vec![$($value), +];
                            let values_iter = values_list.iter();
                            mapping_content.insert($key, Some(HashSet::from_iter(values_iter)));
                          )+
                        let mapping = Mapping::new(&mapping_content, $charset);
                        mapping
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

    // #[test]
    // fn test_get_possible_mappings() {
    //     let mut mapping_content: HashMap<String, Vec<String>> = HashMap::new();
    //     mapping_content.insert("1".to_string(), vec![""])
    // }

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

    // #[test]
    // fn test_generate_key_string() {
    //     let mut mapping_content = HashMap::new();
    //     mapping_content.insert("f", Some(HashSet::from_iter(vec!["a"].iter())));
    //     mapping_content.insert("g", Some(HashSet::from_iter(vec!["b"].iter())));
    //     mapping_content.insert("h", Some(HashSet::from_iter(vec!["c"].iter())));
    //     mapping_content.insert("i", Some(HashSet::from_iter(vec!["d"].iter())));
    //     mapping_content.insert("j", Some(HashSet::from_iter(vec!["e"].iter())));
    //     let expected_keystring = "ABCDEFGHIJKLMNOPQRSTUVWXYZfghijfghijklmnopqrstuvwxyz";
    //     let test_charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    //     let mapping = Mapping::new(&mapping_content, &test_charset);
    //     let returned_keystring = mapping.generate_key_string();
    //     assert_eq!(expected_keystring, returned_keystring)
    // }
    //
    // #[test]
    // fn test_get_possible_mappings() {
    //     //
    //     let mut mapping_content = HashMap::new();
    //     mapping_content.insert("1", Some(HashSet::from_iter(vec!["a", "b"].iter())));
    //     mapping_content.insert("2", Some(HashSet::from_iter(vec!["c"].iter())));
    //     mapping_content.insert("3", Some(HashSet::from_iter(vec!["d"].iter())));
    //     mapping_content.insert("4", Some(HashSet::from_iter(vec!["e", "f"].iter())));
    //     mapping_content.insert("5", Some(HashSet::from_iter(vec!["g", "h"].iter())));
    //     let mut mapping = Mapping::new_empty(TEST_CHARSET);
    //     mapping.load_content(&mapping_content);

    // }

}