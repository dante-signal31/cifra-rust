use std::ops::Add;

use crate::attack::simple_attacks::{Parameters, ParameterValue};

/// Library to cipher and decipher texts using Caesar method.
pub const DEFAULT_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";


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
pub fn cipher<T, U>(text: T, key: usize, charset: U)-> String
    where T: AsRef<str>,
          U: AsRef<str> {
    let ciphered_text = offset_text(text, key, true, charset);
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
pub fn decipher<T, U>(ciphered_text: T, key: usize, charset: U)-> String
    where T: AsRef<str>,
          U: AsRef<str> {
// pub fn decipher(parameters: &Parameters)-> String {
//     let ciphered_text = parameters.get_str("ciphered_text");
//     let charset = parameters.get_str("charset");
//     let key = parameters.get_int("key");
    let deciphered_text = offset_text(ciphered_text, key, false, charset);
    deciphered_text
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
pub fn decipher_par(parameters: &Parameters)-> String {
    let ciphered_text = parameters.get_str("ciphered_text");
    let charset = parameters.get_str("charset");
    let key = parameters.get_int("key");
    decipher(ciphered_text, key, charset)
}

/// Generic function to offset text characters frontwards and backwards.
///
/// # Parameters:
/// * text: Text to offset.
/// * key: Number of positions to offset characters.
/// * advance: If True offset characters frontwards.
/// * charset: Charset to use for substitution.
///
/// Returns:
/// *  Offset text.
fn offset_text<T, U>(text: T, key: usize, advance: bool, charset: U)-> String
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut offset_text = String::new();
    for character in text.as_ref().chars() {
        let normalized_char = character.to_lowercase().to_string();
        let new_character = match get_new_char_position(&normalized_char, key, advance, &charset) {
                Some(new_char_position) => charset.as_ref().chars().nth(new_char_position).unwrap(),
                _ => character.clone()
            };
        offset_text = if character.is_lowercase() {
                offset_text.add(new_character.to_string().as_str())
            } else {
                offset_text.add(new_character.to_uppercase().to_string().as_str())
            };
    }
    offset_text
}


/// Get position for offset char.
///
/// Not all character are subject to offset, only those present at charset.
///
/// # Parameters:
/// * char: Actual character with no offset. It should be normalized to be
///    sure it is present at charset.
/// * key: Number of positions to offset characters.
/// * advance: If True offset characters frontwards.
/// * charset: Charset to use for substitution.
///
/// Returns:
/// * If char is present at charset returns its index for offset char. If not returns None.
fn get_new_char_position<T, U>(character: T, key: usize, advance: bool, charset: U)-> Option<usize>
    where T: AsRef<str>,
          U: AsRef<str> {
    let charset_length = charset.as_ref().len();
    let character_to_find = character.as_ref().chars().nth(0)?;
    let char_position = match charset.as_ref().find(character_to_find) {
            Some(index) => index,
            _ => return None
        };
    let offset_position = if advance {
        (char_position + key) as isize
        } else {
            char_position as isize - key as isize
        };
    let new_char_position = if advance {
            offset_position.abs() as usize % charset_length
        } else {
            if offset_position >= 0 {
                offset_position.abs() as usize
            } else {
               charset_length - offset_position.abs() as usize % charset_length
            }
        };
    Some(new_char_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_MESSAGE: &str = "This is my secret message.";
    const CIPHERED_MESSAGE_KEY_13: &str = "Guvf vf zl frperg zrffntr.";

    #[test]
    fn test_cipher() {
        let ciphered_text = cipher(ORIGINAL_MESSAGE, 13, DEFAULT_CHARSET);
        assert_eq!(CIPHERED_MESSAGE_KEY_13, ciphered_text,
                   "Expected message was:\n\t{}\nBut ciphered was:\n\t{}\n",
                   CIPHERED_MESSAGE_KEY_13, ciphered_text)
    }

    #[test]
    fn test_decipher() {
        let deciphered_text = decipher(CIPHERED_MESSAGE_KEY_13, 13, DEFAULT_CHARSET);
        assert_eq!(ORIGINAL_MESSAGE, deciphered_text,
                   "Expected message was:\n\t{}\nBut deciphered was:\n\t{}\n",
                   ORIGINAL_MESSAGE, deciphered_text)
    }
}