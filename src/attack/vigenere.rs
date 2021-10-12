/// Module to attack Vigenere cipher texts.
///
/// This module uses two approaches: a dictionary brute force method to guess probable word key used to cipher
/// a text using Vigenere algorithm and a frequency analysis attack.
///
/// You should be aware that to be successful charset used for attack should be the
/// same used to cipher. Besides, this module tries to guess if deciphered text is
/// the good one comparing it with words from a language dictionary. If original
/// message was in a language you don't have a dictionary for, then correct key
/// won't be detected.
use crate::Result;


/// Get Vigenere ciphered text key.
///
/// Uses a brute force technique trying the entire dictionary space until finding a text
/// that can be identified with any of our languages.
///
/// **You should not use this function. Use *brute_force_mp* instead.** This
/// function is slower than *mp* one because is sequential while the other uses a
/// multiprocessing approach. This function only stay here to allow comparisons
/// between sequential and multiprocessing approaches.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * charset: Charset used for Vigenere method substitution. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
/// * testing: Vigenere takes to long time to be tested against real dictionaries. So,
///      to keep tests short if _testing is set to True a mocked key generator is used so only
///      a controlled handful of words are tested to find a valid key. In simple terms: don't
///      mess with this parameter and keep it to False, it is only used for testing.
///
/// # Returns:
/// * Most probable Vigenere key found.
pub fn brute_force<T, U>(ciphered_text: T, charset: U, testing: bool)-> Result<String>
    where T: AsRef<str>,
          U: AsRef<str> {
    // let mut parameters = create_parameters(ciphered_text, charset);
    // simple_brute_force(assess_affine_key, &mut parameters)
    unimplemented!()
}

// pub fn brute_force_mp<T, U>(ciphered_text: T, charset: U, testing: bool)-> Result<String>
//     where T: AsRef<str>,
//           U: AsRef<str> {
pub fn brute_force_mp(ciphered_text: &str, charset: &str, testing: bool)-> Result<String> {
    // let mut parameters = create_parameters(ciphered_text, charset);
    // simple_brute_force(assess_affine_key, &mut parameters)
    unimplemented!()
}

// TODO: Implement this module.