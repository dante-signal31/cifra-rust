/// Module to attack Transposition cipher texts.
///
/// This module uses a brute force method to guess probable key used to cipher
/// a text using Transposition algorithm.

use crate::Result;
use crate::attack::dictionaries::IdentifiedLanguage;
use crate::attack::simple_attacks::{Parameters, assess_key};
use crate::attack::simple_attacks::brute_force as simple_brute_force;
use crate::attack::simple_attacks::brute_force_mp as simple_brute_force_mp;
use crate::cipher::transposition::decipher_par;

/// Get Transposition ciphered text key.
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
/// * ciphered_text: Text to be deciphered.
///
/// # Returns:
/// * Transposition key found.
pub fn brute_force<T>(ciphered_text: T)-> Result<usize>
    where T: AsRef<str> {
    let mut parameters = create_parameters(ciphered_text);
    simple_brute_force(assess_transposition_key, &mut parameters)
}

/// Get Transposition ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
///
/// **You should use this function instead of *brute_force*.**
///
/// Whereas *brute_force* uses a sequential approach, this function uses
/// multiprocessing to improve performance.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
///
/// # Returns:
/// * Transposition key found.
pub fn brute_force_mp<T>(ciphered_text: T)-> Result<usize>
    where T: AsRef<str> + std::marker::Sync {
    let mut parameters = create_parameters(ciphered_text);
    simple_brute_force_mp(assess_transposition_key, &mut parameters)
}

/// Get a Parameters type with given arguments.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
///
/// # Returns:
/// * A Parameters type with next key-values:
///     * ciphered_text: Text to be deciphered.
///     * key_space_length: Key space length of cipher to crack.
fn create_parameters<T>(ciphered_text: T)-> Parameters
    where T: AsRef<str> {
    let key_space_length = ciphered_text.as_ref().len();
    let mut parameters = Parameters::new();
    parameters.insert_str("ciphered_text", ciphered_text.as_ref());
    parameters. insert_int("key_space_length", key_space_length);
    parameters
}

/// Decipher text with given key and try to find out if returned text can be identified with any
/// language in our dictionaries.
///
/// # Parameters:
/// * parameters: A Parameters type with at least next key-values.
///     * ciphered_text (str): Text to be deciphered.
///     * key: Key to decipher *ciphered_text*.
///
/// # Returns:
/// * A tuple with used key ans An *IdentifiedLanguage* object with assessment result.
fn assess_transposition_key(parameters: &Parameters)-> Result<(usize, IdentifiedLanguage)>{
    assess_key(decipher_par, parameters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::time::Instant;
    use crate::attack::dictionaries::tests::LoadedDictionaries;
    use crate::cipher::transposition::decipher;

    const ORIGINAL_MESSAGE: &str = "Common sense is not so common.";
    const CIPHERED_MESSAGE_KEY_8: &str = "Cenoonommstmme oo snnio. s s c";
    const TEST_KEY: usize = 8;

    #[test]
    fn test_brute_force_transposition() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force(CIPHERED_MESSAGE_KEY_8);
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_transposition: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    #[test]
    fn test_brute_force_transposition_mp() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force_mp(CIPHERED_MESSAGE_KEY_8);
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_transposition_mp: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    fn assert_found_key(found_key: Result<usize>) {
        if let Ok(key) = found_key {
            assert_eq!(key, TEST_KEY);
            let deciphered = decipher(CIPHERED_MESSAGE_KEY_8, key);
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


