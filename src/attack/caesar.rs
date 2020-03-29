use crate::attack::dictionaries::IdentifiedLanguage;

/// Get Caesar ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
/// **You should not use this function. Use *brute_force_caesar_mp* instead.** This
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
pub fn brute_force_caesar<T, U>(ciphered_text: T, charset: U)-> usize
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
}

/// Get Caesar ciphered text key.
///
/// Uses a brute force technique trying the entire key space until finding a text
/// that can be identified with any of our languages.
///
/// **You should use this function instead of *brute_force_caesar*.**
///
/// Whereas *brute_force_caesar* uses a sequential approach, this function uses
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
pub fn brute_force_caesar_mp<T,U>(ciphered_text: T, charset: U)-> usize
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
}

/// Decipher text with given key and try to find out if returned text can be identified with any
/// language in our dictionaries.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Key to decipher *ciphered_text*.
/// * charset: Charset used for Caesar method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * A tuple with used key ans An *IdentifiedLanguage* object with assessment result.
fn assess_caesar_key<T,U>(ciphered_text: T, key: usize, charset: U)-> (usize, IdentifiedLanguage)
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
}

/// Assess a list of IdentifiedLanguage objects and select the most likely.
///
/// # Parameters:
/// * identified_languages: A list of tuples with a Caesar key and its corresponding IdentifiedLanguage object.
///
/// # Returns:
/// * Caesar key whose IdentifiedLanguage object got the highest probability.
fn get_best_result(identified_language: Vec<(usize, IdentifiedLanguage)>)-> usize {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use crate::attack::dictionaries::tests::LoadedDictionaries;
    use crate::cipher::caesar::{DEFAULT_CHARSET, decipher};
    use diesel::result::Error::DatabaseError;

    const ORIGINAL_MESSAGE: &'static str = "This is my secret message.";
    const CIPHERED_MESSAGE_KEY_13: &'static str = "Guvf vf zl frperg zrffntr.";
    const TEST_KEY: usize = 13;
    
    #[test]
    fn test_brute_force_caesar() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force_caesar(CIPHERED_MESSAGE_KEY_13, DEFAULT_CHARSET);
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_caesar: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    #[test]
    fn test_brute_force_caesar_mp() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let timer = Instant::now();
        let found_key = brute_force_caesar_mp(CIPHERED_MESSAGE_KEY_13, DEFAULT_CHARSET);
        assert_found_key(found_key);
        println!("{}", format!("\n\nElapsed time with test_brute_force_caesar_mp: {:.2} seconds.", timer.elapsed().as_secs_f64()));
    }

    fn assert_found_key(found_key: usize){
        assert_eq!(TEST_KEY, found_key);
        let deciphered_key = decipher(CIPHERED_MESSAGE_KEY_13, found_key, DEFAULT_CHARSET);
        assert_eq!(ORIGINAL_MESSAGE, deciphered_key);
    }
}