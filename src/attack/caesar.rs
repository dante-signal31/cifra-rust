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
pub fn brute_force_caesar<T, U>(ciphered_text: T, charset: U)-> u8
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
pub fn brute_force_caesar_mp<T,U>(ciphered_text: T, charset: U)-> u8
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
fn assess_caesar_key<T,U>(ciphered_text: T, key: u8, charset: U)-> (u8, IdentifiedLanguage)
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
fn get_best_result(identified_language: Vec<(u8, IdentifiedLanguage)>)-> u8 {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_MESSAGE: &'static str = "This is my secret message.";
    const CIPHERED_MESSAGE_KEY_13: &'static str = "Guvf vf zl frperg zrffntr.";
    const TEST_KEY: u8 = 13;
    
    #[test]
    fn test_brute_force_caesar() {
        unimplemented!()
    }

    #[test]
    fn test_brute_force_caesar_mp() {
        unimplemented!()
    }
}