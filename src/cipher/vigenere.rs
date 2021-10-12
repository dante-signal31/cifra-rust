/// Library to cipher and decipher texts using Vigenere method.
use crate::cipher::common::{offset_text, Ciphers};

use crate::{ErrorKind, Result, ResultExt};

// To keep along with book examples I'm going to work with an only lowercase
// charset.
pub const DEFAULT_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";

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
// pub fn cipher<T, U, V>(text: T, key: U, charset: V) -> Result<String>
//     where T: AsRef<str>,
//           U: AsRef<str>,
//           V: AsRef<str> {
pub fn cipher(text: &str, key: &str, charset: &str) -> Result<String> {
    let ciphered_text = vigenere_offset(text, key, Vigenere::CIPHER, charset);
    ciphered_text
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
// pub fn decipher<T, U, V>(ciphered_text: T, key: U, charset: V) -> Result<String>
//     where T: AsRef<str>,
//           U: AsRef<str>,
//           V: AsRef<str> {
pub fn decipher(ciphered_text: &str, key: &str, charset: &str) -> Result<String> {
    let deciphered_text = vigenere_offset(ciphered_text, key, Vigenere::DECIPHER, charset);
    deciphered_text
}

/// Utility function to reduce code redundancy with Vigenere operations.
///
/// Don't use this function directly.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Secret key. Both ends should know this and
///     use the same one. The longer key you use the harder to break ciphered text.
/// * charset: Charset used for Vigenere method. Both end should
///     use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * Offset text.
fn vigenere_offset<T, U, V>(text: T, key: U, operation: Vigenere, charset: V) -> Result<String>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    let advance = match operation {
        Vigenere::CIPHER => true,
        Vigenere::DECIPHER => false
    };
    let key_length = key.as_ref().len();
    let mut offset_chars: Vec<String> = Vec::new();
    let mut skip_accumulator: usize = 0;
    for (index, char) in text.as_ref().chars().enumerate() {
        let char_to_find: String = char.to_lowercase().collect();
        if !(charset.as_ref().contains( &char_to_find)) {
            offset_chars.push(char.to_string());
            skip_accumulator += 1;
            continue;
        }
        let subkey_char= key.as_ref().chars().nth((index - skip_accumulator) % key_length)
            .chain_err(|| ErrorKind::KeyError(key.as_ref().to_string(), "Error getting subkey.".to_string()))?;
        let subkey_offset = charset.as_ref().find(|x: char| x == subkey_char)
            .chain_err(|| ErrorKind::KeyError(key.as_ref().to_string(), "Error finding subkey index.".to_string()))?;
        let mut offset_char = String::new();
        if char.is_lowercase() {
            offset_char = offset_text(char.to_string(), subkey_offset, advance, &Ciphers::VIGENERE, &charset)?;
        } else {
            offset_char = offset_text(char.to_lowercase().to_string(), subkey_offset, advance, &Ciphers::VIGENERE, &charset)?;
            offset_char = offset_char.to_uppercase();
        }
        offset_chars.push(offset_char);
    }
    let offset_text = offset_chars.join("");
    Ok(offset_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_MESSAGE: &'static str = "Common sense is not so common.";
    const CIPHERED_MESSAGE: &'static str = "Rwlloc admst qr moi an bobunm.";
    const TEST_KEY: &'static str = "pizza";

    #[test]
    fn test_cipher() {
        let ciphered_text = cipher(ORIGINAL_MESSAGE, TEST_KEY, DEFAULT_CHARSET)
            .expect("Error ciphering text with Vigenere ciphering.");
        assert_eq!(ciphered_text, CIPHERED_MESSAGE,
                   "Message {} was not what we were expecting {}",
                   ciphered_text, CIPHERED_MESSAGE);
    }

    #[test]
    fn test_decipher() {
        let deciphered_text = decipher(CIPHERED_MESSAGE, TEST_KEY, DEFAULT_CHARSET)
            .expect("Error deciphering text with Vigenere deciphering.");
        assert_eq!(deciphered_text, ORIGINAL_MESSAGE,
                "Message {} was not what we were expecting {}",
                deciphered_text, ORIGINAL_MESSAGE);
    }
}
