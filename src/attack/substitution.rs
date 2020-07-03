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
fn hack_substitution<T, U, V>(ciphered_text: T, charset: U, database_path: Option<V>) -> (String, f64)
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    unimplemented!()
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
            let found_key = hack_substitution(&ciphered_text, &set.charset,
                                              Some(loaded_dictionaries.temp_dir.to_str().unwrap()));
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


}