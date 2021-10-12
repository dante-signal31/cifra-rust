// use std::ops::Add;

use crate::attack::simple_attacks::Parameters;
use crate::cipher::common::{offset_text, Ciphers};
use crate::Result;

/// Library to cipher and decipher texts using Caesar method.
// pub const DEFAULT_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";


/// Cipher given text using Caesar method.
///
/// Be aware that different languages use different charsets. Default charset
/// is for english language, if you are using any other you should use a proper
/// dataset. For instance, if you are ciphering an spanish text, you should use
/// a charset with "ñ" character.
///
/// # Parameters:
/// * text: Text to be ciphered.
/// * key: Secret key. In Caesar method it corresponds with how many position
///     advance in the charset. Both ends should know this and use the same one.
/// * charset: Charset used for Caesar method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Ciphered text.
// pub fn cipher<T, U>(text: T, key: usize, charset: U)-> Result<String>
//     where T: AsRef<str>,
//           U: AsRef<str> {
pub fn cipher(text: &str, key: usize, charset: &str)-> Result<String> {
    let ciphered_text = offset_text(text, key, true, &Ciphers::CAESAR, charset);
    ciphered_text
}

/// Decipher given text using Caesar method.
///
/// Note you should use the same charset that ciphering end did.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Secret key. In Caesar method, and for deciphering end, it correspond
///     with how many position get bat in the charset. Both ends should know this and
///     use the same one.
/// * charset: Charset used for Caesar method substitutions. Both end should
///    use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * Deciphered text.
// pub fn decipher<T, U>(ciphered_text: T, key: usize, charset: U)-> Result<String>
//     where T: AsRef<str>,
//           U: AsRef<str> {
pub fn decipher(ciphered_text: &str, key: usize, charset: &str)-> Result<String> {
    let deciphered_text = offset_text(ciphered_text, key, false, &Ciphers::CAESAR, charset)?;
    Ok(deciphered_text)
}

/// Call decipher function using a Parameters type.
///
/// You probably wont use this function. It's used by brute force attacks instead.
///
/// # Parameters:
/// * parameters: Parameters stored in a Parameters type. It should include next keys-values:
///     * ciphered_text (str): Text to be deciphered.
///     * key (usize): Secret key. In Caesar method, and for deciphering end, it correspond
///         with how many position get bat in the charset. Both ends should know this and
///         use the same one.
///     * charset (str): Charset used for Caesar method substitutions. Both end should
///         use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * Deciphered text.
pub fn decipher_par(parameters: &Parameters)-> Result<String> {
    let ciphered_text = parameters.get_str("ciphered_text")?;
    let charset = parameters.get_str("charset")?;
    let key = parameters.get_int("key")?;
    decipher(ciphered_text.as_str(), key, charset.as_str())
}


#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::cipher::common::DEFAULT_CHARSET;

    pub const ORIGINAL_MESSAGE: &str = "This is my secret message.";
    pub const CIPHERED_MESSAGE_KEY_13: &str = "guv6Jv6Jz!J6rp5r7Jzr66ntrM";
    pub const TEST_KEY: usize = 13;

    #[test]
    fn test_cipher() {
        let ciphered = cipher(ORIGINAL_MESSAGE, TEST_KEY, DEFAULT_CHARSET);
        if let Ok(ciphered_text) = ciphered {
            assert_eq!(CIPHERED_MESSAGE_KEY_13, ciphered_text,
                       "Expected message was:\n\t{}\nBut ciphered was:\n\t{}\n",
                       CIPHERED_MESSAGE_KEY_13, ciphered_text)
        } else {
            assert!(false, "Ciphering operation returned an error.")
        }
    }

    #[test]
    fn test_decipher() {
        let deciphered = decipher(CIPHERED_MESSAGE_KEY_13, TEST_KEY, DEFAULT_CHARSET);
        if let Ok(deciphered_text) = deciphered {
            assert_eq!(ORIGINAL_MESSAGE, deciphered_text,
                       "Expected message was:\n\t{}\nBut deciphered was:\n\t{}\n",
                       ORIGINAL_MESSAGE, deciphered_text)
        } else {
            assert!(false, "Ciphering operation returned an error.")
        }
    }
}