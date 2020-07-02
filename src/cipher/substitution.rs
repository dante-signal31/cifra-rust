/// Library to cipher and decipher texts using substitution method.
use crate::{ErrorKind, Result, ResultExt, Error};
use std::collections::HashSet;

const DEFAULT_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";

/// Check used key is a valid one for substitution method with this charset.
///
/// # Parameters:
/// * key: Secret key. In substitution method it corresponds with how to
///      substitute each character in the charset. Both ends should know this and
///      use the same one. Besides key should have the same length than charset and
///      no repeated characters.
/// * charset: Charset used for substitution method. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
///
/// # Raises:
/// * ErrorKind::WrongKeyLength: If given key has wrong length.
/// * ErrorKind::WrongKeyRepeatedCharacters: If given key has repeated characters.
fn check_substitution_key<T, U>(key: T, charset: U) -> Result<()>
    where T: AsRef<str>,
          U: AsRef<str> {
    let charset_set: HashSet<char> = charset.as_ref().chars().collect();
    let key_set: HashSet<char> = key.as_ref().chars().collect();
    if key.as_ref().len() != charset.as_ref().len() {
        bail!(ErrorKind::WrongKeyLength(key.as_ref().to_string(), charset.as_ref().to_string()))
    } else if key_set.len() != charset_set.len() {
        bail!(ErrorKind::WrongKeyRepeatedCharacters(key.as_ref().to_string()))
    }
    Ok(())
}

/// Cipher given text using substitution method.
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
/// * key: Secret key. In substitution method it corresponds with how to
///      substitute each character in the charset. Both ends should know this and
///      use the same one. Besides key should have the same length than charset and
///      no repeated characters.
/// * charset: Charset used for substitution method. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
///
/// # Returns:
/// * Ciphered text.
///
/// # Raises:
/// * CharacterMappingError: If there were an error mapping a char to its substitution.
fn cipher<T, U, V>(text: T, key: U, charset: V) -> Result<String>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    check_substitution_key(&key, &charset)?;
    let mut ciphered_message: String = String::new();
    let key_chars: Vec<char> = key.as_ref().chars().collect();
    for _char in text.as_ref().chars() {
        if charset.as_ref().contains(_char.to_lowercase().to_string().as_str()) {
            let charset_index = match charset.as_ref().find(_char.to_lowercase().to_string().as_str()){
                Some(index) => index,
                None => bail!(ErrorKind::CharacterMappingError(_char.to_lowercase().to_string()))
            };
            let mapped_char: char = key_chars[charset_index];
            let ciphered_chars: String = if _char.is_lowercase() {
                mapped_char.to_string()
            } else {
                mapped_char.to_uppercase().to_string()
            };
            ciphered_message.push_str(ciphered_chars.as_str());
        } else {
            ciphered_message.push_str(_char.to_string().as_str())
        }
    }
    Ok(ciphered_message)
}

/// Decipher given text using substitution method.
///
/// Note you should use the same charset that ciphering end did.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Secret key. In substitution method it corresponds with how to
///      substitute each character in the charset. Both ends should know this and
///      use the same one. Besides key should have the same length than charset and
///      no repeated characters.
/// * charset: Charset used for substitution method. Both ends, ciphering
///      and deciphering, should use the same charset or original text won't be properly
///      recovered.
///
/// # Returns:
/// * Deciphered text.
///
/// # Raises:
/// * CharacterMappingError: If there were an error mapping a char to its substitution.
fn decipher<T, U, V>(ciphered_text: T, key: U, charset: V) -> Result<String>
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    check_substitution_key(&key, &charset)?;
    let mut deciphered_message = String::new();
    let charset_chars: Vec<char> = charset.as_ref().chars().collect();
    for ciphered_char in ciphered_text.as_ref().chars() {
        if key.as_ref().contains(ciphered_char.to_lowercase().to_string().as_str()) {
            let key_index = match key.as_ref().find(ciphered_char.to_lowercase().to_string().as_str()) {
                Some(index) => index,
                None => bail!(ErrorKind::CharacterMappingError(ciphered_char.to_lowercase().to_string()))
            };
            let deciphered_char = charset_chars[key_index];
            let deciphered_chars = if ciphered_char.is_lowercase() {
                deciphered_char.to_string()
            } else {
                deciphered_char.to_string().to_uppercase()
            };
            deciphered_message.push_str(deciphered_chars.as_str());
        } else {
            deciphered_message.push_str(ciphered_char.to_string().as_str());
        }
    }
    Ok(deciphered_message)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const TEST_KEY: &'static str =     "lfwoayuisvkmnxpbdcrjtqeghz";
    const ORIGINAL_MESSAGE: &'static str  = "If a man is offered a fact which goes against his \
                                    instincts, he will scrutinize it closely, and unless \
                                    the evidence is overwhelming, he will refuse to believe \
                                    it. If, on the other hand, he is offered something which \
                                    affords a reason for acting in accordance to his \
                                    instincts, he will accept it even on the slightest \
                                    evidence. The origin of myths is explained in this way. \
                                    -Bertrand Russell";
    const CIPHERED_MESSAGE: &'static str = "Sy l nlx sr pyyacao l ylwj eiswi upar lulsxrj isr \
                                    sxrjsxwjr, ia esmm rwctjsxsza sj wmpramh, lxo txmarr \
                                    jia aqsoaxwa sr pqaceiamnsxu, ia esmm caytra \
                                    jp famsaqa sj. Sy, px jia pjiac ilxo, ia sr \
                                    pyyacao rpnajisxu eiswi lyypcor l calrpx ypc \
                                    lwjsxu sx lwwpcolxwa jp isr sxrjsxwjr, ia esmm \
                                    lwwabj sj aqax px jia rmsuijarj aqsoaxwa. Jia pcsusx \
                                    py nhjir sr agbmlsxao sx jisr elh. -Facjclxo Ctrramm";


    #[test]
    fn test_cipher() {
        match cipher(ORIGINAL_MESSAGE, TEST_KEY, TEST_CHARSET) {
            Ok(ciphered_text) => {
                assert_eq!(CIPHERED_MESSAGE, ciphered_text, "Message was not ciphered as we were expecting.")
            },
            Err(E) => {
                assert!(false, format!("Error happened: {}", E))
            }
        }
    }

    #[test]
    fn test_decipher() {
        match decipher(CIPHERED_MESSAGE, TEST_KEY, TEST_CHARSET) {
            Ok(deciphered_text) => {
                assert_eq!(ORIGINAL_MESSAGE, deciphered_text, "Deciphered message was not the one we were expecting")
            },
            Err(E) => {
                assert!(false, format!("Error happened: {}", E))
            }
        }
    }

    #[test]
    fn test_wrong_length_key_are_detected() {
        let test_charset = "123";
        let wrong_key = "1234";
        if let Err(E) = cipher("", wrong_key, test_charset) {
            match Error::from(E) {
                Error(ErrorKind::WrongKeyLength(_, _), _) => assert!(true),
                error => assert!(false, format!("Raised error was not the one \
                                          we were expecting but {} instead", error))
            }
        } else { assert!(false, "No error was raised when wrong key used.") }
    }

    #[test]
    fn test_repeated_character_keys_are_detected() {
        let test_charset = "123";
        let wrong_key = "122";
        if let Err(E) = cipher("", wrong_key, test_charset) {
            match Error::from(E) {
                Error(ErrorKind::WrongKeyRepeatedCharacters(_), _) => assert!(true),
                error => assert!(false, format!("Raised error was not the one \
                                          we were expecting but {} instead", error))
            }
        } else { assert!(false, "No error was raised when wrong key used.") }
    }
}