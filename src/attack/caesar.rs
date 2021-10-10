// use std::collections::HashMap;

// use rayon::prelude::*;

use crate::Result;
use crate::attack::dictionaries::IdentifiedLanguage;
use crate::attack::simple_attacks::brute_force as simple_brute_force;
use crate::attack::simple_attacks::brute_force_mp as simple_brute_force_mp;
use crate::attack::simple_attacks::{assess_key, Parameters};
use crate::cipher::caesar::decipher_par;
// use crate::cipher::common::DEFAULT_CHARSET;


/// Get Caesar ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
/// **You should not use this function. Use *brute_force_mp* instead.** This
/// function is slower than *mp* one because is sequential while the other uses a
/// multiprocessing approach. This function only stay here to allow comparisons
/// between sequential and multiprocessing approaches.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for Caesar method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Caesar key found.
pub fn brute_force<T, U>(ciphered_text: T, charset: U) -> Result<usize>
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut parameters = create_parameters(ciphered_text, charset);
    simple_brute_force(assess_caesar_key, &mut parameters)
}

/// Get Caesar ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
///
/// **You should use this function instead of *brute_caesar*.**
///
/// Whereas *brute_caesar* uses a sequential approach, this function uses
/// multiprocessing to improve performance.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for Caesar method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Caesar key found.
pub fn brute_force_mp<T,U>(ciphered_text: T, charset: U) -> Result<usize>
    where T: AsRef<str> + std::marker::Sync,
          U: AsRef<str> + std::marker::Sync {
    let mut parameters = create_parameters(ciphered_text, charset);
    simple_brute_force_mp(assess_caesar_key, &mut parameters)
}

/// Get a Parameters type with given arguments.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for Caesar method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * A Parameters type with next key-values:
///     * ciphered_text: Text to be deciphered.
///     * charset: Charset used for Caesar method substitution. Both ends, ciphering
///         and deciphering, should use the same charset or original text won't be properly
///         recovered.
///     * key_space_length: Key space length of cipher to crack.
fn create_parameters<T,U>(ciphered_text: T, charset: U) -> Parameters
    where T: AsRef<str>,
          U: AsRef<str> {
    let key_space_length = charset.as_ref().len();
    let mut parameters: Parameters = Parameters::new();
    parameters.insert_str("ciphered_text", ciphered_text);
    parameters.insert_str("charset", charset);
    parameters.insert_int("key_space_length", key_space_length);
    parameters
}

/// Decipher text with given key and try to find out if returned text can be identified with any
/// language in our dictionaries.
///
/// # Parameters:
/// * A Parameters type with these keys-values.
///     - ciphered_text (str) : Text to be deciphered.
///     - key (usize): Key to decipher *ciphered_text*.
///     - charset (str): Charset used for Caesar method substitution. Both ends, ciphering
///         and deciphering, should use the same charset or original text won't be properly
///         recovered.
///
/// # Returns:
/// * A tuple with used key and an *IdentifiedLanguage* object with assessment result.
fn assess_caesar_key(parameters: &Parameters)-> Result<(usize, IdentifiedLanguage)> {
    assess_key(decipher_par, parameters)
}

/// Assess a list of IdentifiedLanguage objects and select the most likely.
///
/// # Parameters:
/// * identified_languages: A list of tuples with a Caesar key and its corresponding IdentifiedLanguage object.
///
/// # Returns:
/// * Caesar key whose IdentifiedLanguage object got the highest probability.
fn get_best_result(identified_languages: &Vec<(usize, IdentifiedLanguage)>)-> usize {
    let mut current_best_key: usize = 0;
    let mut current_best_probability: f64 = 0.0;
    for (caesar_key, identified_language) in identified_languages {
        if identified_language.winner == None {
            continue;
        } else {
            if let Some(winner_probability) = identified_language.winner_probability {
                if winner_probability > current_best_probability {
                    current_best_key = *caesar_key;
                    current_best_probability = winner_probability;
                }
            }
        }
    };
    current_best_key
}


#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::time::Instant;
    use crate::attack::dictionaries::tests::LoadedDictionaries;
    use crate::cipher::caesar::decipher;
    use crate::cipher::common::DEFAULT_CHARSET;
    use diesel::result::Error::DatabaseError;

    const ORIGINAL_MESSAGE: &str = "This is my secret message.";
    const CIPHERED_MESSAGE_KEY_13: &str = "guv6Jv6Jz!J6rp5r7Jzr66ntrM";
    const TEST_KEY: usize = 13;
    
    #[test]
    fn test_brute_force_caesar() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force(CIPHERED_MESSAGE_KEY_13, DEFAULT_CHARSET);
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_caesar: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    #[test]
    fn test_brute_force_caesar_mp() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force_mp(CIPHERED_MESSAGE_KEY_13, DEFAULT_CHARSET);
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_caesar_mp: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    fn assert_found_key(found_key: Result<usize>){
        if let Ok(key) = found_key {
            assert_eq!(key, TEST_KEY);
            let deciphered = decipher(CIPHERED_MESSAGE_KEY_13, key, DEFAULT_CHARSET);
            if let Ok(deciphered_text) = deciphered {
                assert_eq!(deciphered_text, ORIGINAL_MESSAGE);
            } else {
                assert!(false, "At deciphering we only could get an error.")
            }
        } else {
            assert!(false, "Test result was an error.")
        }
    }
}