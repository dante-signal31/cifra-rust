/// Module to attack Transposition cipher texts.
///
/// This module uses a brute force method to guess probable key used to cipher
/// a text using Transposition algorithm.

use crate::attack::dictionaries::IdentifiedLanguage;
use crate::attack::simple_attacks::Parameters;

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
pub fn brute_force<T>(ciphered_text: T)-> usize
    where T: AsRef<str> {
    unimplemented!()
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
pub fn brute_force_mp<T>(ciphered_text: T)-> usize
    where T: AsRef<str> {
    unimplemented!()
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
fn assess_transposition_key(parameters: &Parameters)-> (usize, IdentifiedLanguage){
    unimplemented!()
}


