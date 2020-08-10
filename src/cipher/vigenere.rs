/// Library to cipher and decipher texts using Vigenere method.
use crate::cipher::common::{offset_text, Ciphers};

// To keep along with book examples I'm going to work with an only lowercase
// charset.
const DEFAULT_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";

enum Vigenere {
    CIPHER,
    DECIPHER
}

/// Cipher given text using Vigenere method.
///
/// Be aware that different languages use different charsets. Default charset
/// is for english language, if you are using any other you should use a proper
/// dataset. For instance, if you are ciphering an spanish text, you should use
/// a charset with "ñ" character.
///
/// This module uses only lowercase charsets. That means that caps will be kept
/// but lowercase and uppercase will follow ths same substitutions.
///
/// # Parameters:
/// * text: Text to be ciphered.
/// * key: Secret key. Both ends should know this and
///     use the same one. The longer key you use the harder to break ciphered text.
/// * charset: Charset used for Vigenere method. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Ciphered text.
pub fn cipher<T, U, V>(text: T, key: U, charset: V) -> String
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    unimplemented!()
}

/// Decipher given text using Vigenere method.
///
/// Note you should use the same charset that ciphering end did.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Secret key. Both ends should know this and
///     use the same one. The longer key you use the harder to break ciphered text.
/// * charset: Charset used for Vigenere method. Both end should
///     use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * Deciphered text.
pub fn decipher<T, U, V>(text: T, key: U, charset: V) -> String
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_MESSAGE: &'static str = "Common sense is not so common.";
    const CIPHERED_MESSAGE: &'static str = "Rwlloc admst qr moi an bobunm.";
    const TEST_KEY: &'static str = "pizza";

    #[test]
    fn test_cipher() {
        let ciphered_text = cipher(ORIGINAL_MESSAGE, TEST_KEY, DEFAULT_CHARSET);
        assert_eq!(ciphered_text, CIPHERED_MESSAGE);
    }

    #[test]
    fn test_decipher() {
        let deciphered_text = decipher(CIPHERED_MESSAGE, TEST_KEY, DEFAULT_CHARSET);
        assert_eq!(deciphered_text, ORIGINAL_MESSAGE);
    }
}
