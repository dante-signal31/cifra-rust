/// Module to attack Affine cipher texts.
///
/// This module uses a brute force method to guess probable key used to cipher
/// a text using Affine algorithm.
///
/// You should be aware that to be successful charset used for attack should be the
/// same used to cipher. Besides, this module tries to guess if deciphered text is
/// the good one comparing it with words from a language dictionary. If original
/// message was in a language you don't have a dictionary for, then correct key
/// won't be detected.

use crate::Result;
use crate::attack::dictionaries::IdentifiedLanguage;
use crate::attack::simple_attacks::brute_force as simple_brute_force;
use crate::attack::simple_attacks::brute_force_mp as simple_brute_force_mp;
use crate::attack::simple_attacks::{assess_key, Parameters};
use crate::cipher::affine::{decipher_par, validate_key};


/// Get Affine ciphered text key.
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
/// * charset: Charset used for Affine method substitution. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
///
/// # Returns:
/// * Affine key found.
fn brute_force<T, U>(ciphered_text: T, charset: U)-> Result<usize>
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut parameters = create_parameters(ciphered_text, charset);
    simple_brute_force(assess_affine_key, &mut parameters)
}

/// Get Affine ciphered text key.
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
/// * charset: Charset used for Affine method substitution. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
///
/// # Returns:
/// * Affine key found.
fn brute_force_mp<T, U>(ciphered_text: T, charset: U)-> Result<usize>
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut parameters = create_parameters(ciphered_text, charset);
    simple_brute_force_mp(assess_affine_key, &mut parameters)
}

/// Decipher text with given key and try to find out if returned text can be identified with any
/// language in our dictionaries.
///
/// # Parameters:
/// * A Parameters type with these keys-values.
///     - ciphered_text (str) : Text to be deciphered.
///     - key (usize): Key to decipher *ciphered_text*.
///     - charset (str): Charset used for Affine method substitution. Both ends, ciphering
///         and deciphering, should use the same charset or original text won't be properly
///         recovered.
///
/// # Returns:
/// * A tuple with used key and an *IdentifiedLanguage* object with assessment result.
fn assess_affine_key(parameters: &Parameters)-> Result<(usize, IdentifiedLanguage)> {
    let key = parameters.get_int("key")?;
    let charset = parameters.get_str("charset")?;
    let charset_length = charset.len();
    validate_key(key, charset_length)?;
    assess_key(decipher_par, parameters)
}


/// Get a Parameters type with given arguments.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for Affine method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * A Parameters type with next key-values:
///     * ciphered_text: Text to be deciphered.
///     * charset: Charset used for Affine method substitution. Both ends, ciphering
///         and deciphering, should use the same charset or original text won't be properly
///         recovered.
///     * key_space_length: Key space length of cipher to crack.
fn create_parameters<T,U>(ciphered_text: T, charset: U) -> Parameters
    where T: AsRef<str>,
          U: AsRef<str> {
    let key_space_length = charset.as_ref().len().pow(2);
    let mut parameters: Parameters = Parameters::new();
    parameters.insert_str("ciphered_text", ciphered_text);
    parameters.insert_str("charset", charset);
    parameters.insert_int("key_space_length", key_space_length);
    parameters
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Instant;
    use crate::attack::dictionaries::tests::LoadedDictionaries;
    use crate::cipher::affine::decipher;
    use crate::cipher::common::DEFAULT_CHARSET;

    const ORIGINAL_MESSAGE: &'static str = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
    const CIPHERED_MESSAGE_KEY_331: &'static str = "eiTven8TXvqH/u.?/CqqlvLi.?JT33DSvD?vESn?xvDYvXTJD?OvE.n3DhcvYDSvE.?xX";
    const TEST_KEY: usize = 331;

    fn assert_found_key(found_key: usize) {
        assert_eq!(TEST_KEY, found_key,
                   "Expected key was:\n\t{}\nBut found was:\n\t{}\n",
                   TEST_KEY, found_key);
        let deciphered_text = decipher(&CIPHERED_MESSAGE_KEY_331, found_key, &DEFAULT_CHARSET).unwrap();
        assert_eq!(ORIGINAL_MESSAGE, deciphered_text,
        "Expected message was:\n\t{}\nBut found was:\n\t{}\n",
        ORIGINAL_MESSAGE, deciphered_text);
    }

    #[test]
    fn test_brute_force_affine() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force(CIPHERED_MESSAGE_KEY_331, DEFAULT_CHARSET).unwrap();
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_affine: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    #[test]
    fn test_brute_force_affine_mp() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force_mp(CIPHERED_MESSAGE_KEY_331, DEFAULT_CHARSET).unwrap();
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_caesar_mp: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }
}