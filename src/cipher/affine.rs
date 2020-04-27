/// Library to cipher and decipher texts using Affine method.
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::{Result, ErrorKind, ResultExt};
use crate::cipher::common::{offset_text, Ciphers, DEFAULT_CHARSET, get_key_parts};
use crate::cipher::cryptomath::gcd;
use std::convert::TryInto;


#[derive(Debug, Copy, Clone)]
enum WrongAffineKeyCauses {
    MultiplyingKeyBelowZero,
    MultiplyingKeyZero,
    AddingKeyBelowZero,
    AddingKeyTooLong,
    KeysNotRelativelyPrime
}

impl Display for WrongAffineKeyCauses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            WrongAffineKeyCauses::MultiplyingKeyBelowZero=> "Multiplying key must be greater than 0.",
            WrongAffineKeyCauses::MultiplyingKeyZero=> "Multiplying key must not be 0.",
            WrongAffineKeyCauses::AddingKeyBelowZero=> "Adding key must be greater than 0.",
            WrongAffineKeyCauses::AddingKeyTooLong=> "Adding key must be smaller than charset length.",
            WrongAffineKeyCauses::KeysNotRelativelyPrime=> "Keys are not relatively prime."
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub struct WrongAffineKey {
    key: usize,
    multiplying_key: usize,
    adding_key: usize,
    cause: WrongAffineKeyCauses
}

impl WrongAffineKey {

    fn new(key: usize, cause: WrongAffineKeyCauses, charset_length: usize) -> Self {
        let (multiplying_key, adding_key) = get_key_parts(key, charset_length);
        WrongAffineKey {
            key,
            multiplying_key,
            adding_key,
            cause
        }
    }

    // /// Get because keys are wrong and a written explanation
    // fn get_cause(&mut self)-> (WrongAffineKeyCauses, &'static str){
    //     match self.cause {
    //         WrongAffineKeyCauses::MultiplyingKeyBelowZero=> (self.cause, "Multiplying key must be greater than 0."),
    //         WrongAffineKeyCauses::MultiplyingKeyZero=> (self.cause, "Multiplying key must be greater than 0."),
    //         WrongAffineKeyCauses::AddingKeyBelowZero=> (self.cause, "Multiplying key must be greater than 0."),
    //         WrongAffineKeyCauses::AddingKeyTooLong=> (self.cause, "Multiplying key must be greater than 0."),
    //         WrongAffineKeyCauses::KeysNotRelativelyPrime=> (self.cause, "Multiplying key must be greater than 0.")
    //     }
    // }
}

impl Display for WrongAffineKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wrong key: {} key decomposes to {} as multiplicative key and {} as adding key, \
        but problem is {}", self.key, self.multiplying_key, self.adding_key, self.cause)
    }
}

/// Cipher given text using Affine method.
///
///  Be aware that different languages use different charsets. Default charset
/// is for english language, if you are using any other you should use a proper
/// dataset. For instance, if you are ciphering an spanish text, you should use
/// a charset with "ñ" character.
///
/// Not every key is good to cipher using Affine with a given charset. It must
/// meet a set of rules. So we must check given key meets them.
///
/// If given key does not meet any of the rules them a WrongKey exception is raised.
///
/// # Parameters:
/// * text: Text to be ciphered.
/// * key: Secret key. Both ends should know this and use the same one.
/// * charset: Charset used for Affine method substitution. Both ends, ciphering
///     and deciphering, should use the same charset or original text won't be properly
///     recovered.
///
/// # Returns:
/// * Ciphered text.
fn cipher<T, U>(text: T, key: usize, charset: U)-> Result<String>
    where T: AsRef<str>,
          U: AsRef<str> {
    validate_key(key, charset.as_ref().len());
    let ciphered_text = offset_text(text, key, true, &Ciphers::AFFINE, DEFAULT_CHARSET);
    ciphered_text
}

/// Decipher given text using Affine method.
///
/// Note you should use the same charset that ciphering end did.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Secret key. Both ends should know this and use the same one.
/// * charset: Charset used for Affine method substitutions. Both end should
///     use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * Deciphered text.
fn decipher<T, U>(ciphered_text: T, key: usize, charset: U)-> Result<String>
    where T: AsRef<str>,
          U: AsRef<str> {
    validate_key(key, charset.as_ref().len())?;
    let deciphered_text = offset_text(ciphered_text, key, false, &Ciphers::AFFINE, charset);
    deciphered_text
}

/// Get a valid random Affine key for given charset.
///
/// Get manually a valid Affine key can be hardsome because all rules it must meet.
/// This function automates that task, so you can use it and run.
///
/// # Parameters:
/// * charset: Charset you are going to use to cipher.
///
/// # Returns:
/// * An random Affine key valid for given charset.
fn get_random_key<T>(charset: T)-> usize
    where T: AsRef<str>{
    unimplemented!()
}

/// Check if given key is good for Affine cipher using this charset.
///
/// Not every key is good to cipher using Affine with a given charset. It must
/// meet a set of rules. So we must check given key meets them.
///
/// If given key does not meet any of the rules them a WrongKey exception is raised.
///
/// # Parameters:
/// * key: Secret key. Both ends should know this and use the same one.
/// * charset_length: Charset used for Affine method substitutions. Both end should
///     use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * True if validation was right. You won't receive a False, an exception will be raised before.
fn validate_key(key: usize, charset_length: usize)-> Result<bool> {
    let multiplying_key = key / charset_length;
    let adding_key = key % charset_length;
    if multiplying_key < 0 {
        bail!(ErrorKind::WrongAffineKeyError(
            WrongAffineKey::new(key, WrongAffineKeyCauses::MultiplyingKeyBelowZero, charset_length)
            ));
    } else if multiplying_key == 0 {
        bail!(ErrorKind::WrongAffineKeyError(
            WrongAffineKey::new(key, WrongAffineKeyCauses::MultiplyingKeyZero, charset_length)
            ));
    } else if adding_key < 0 {
        bail!(ErrorKind::WrongAffineKeyError(
            WrongAffineKey::new(key, WrongAffineKeyCauses::AddingKeyBelowZero, charset_length)
            ));
    } else if adding_key > charset_length -1 {
        bail!(ErrorKind::WrongAffineKeyError(
            WrongAffineKey::new(key, WrongAffineKeyCauses::AddingKeyTooLong, charset_length)
            ));
    } else if gcd(multiplying_key.try_into().chain_err(|| ErrorKind::ConversionError("multiplying_key", "usize", "isize"))?,
                  charset_length.try_into().chain_err(|| ErrorKind::ConversionError("charset_length", "usize", "isize"))?) != 1 {
        bail!(ErrorKind::WrongAffineKeyError(
            WrongAffineKey::new(key, WrongAffineKeyCauses::KeysNotRelativelyPrime, charset_length)
            ));
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_common::random::strings::random_string;

    use crate::cipher::common::DEFAULT_CHARSET;

    const ORIGINAL_MESSAGE: &'static str = "A computer would deserve to be called intelligent if it could deceive a human into believing that it was human.\" Alan Turing";
    const CIPHERED_MESSAGE_KEY_2894: &'static str = "5QG9ol3La6QI93!xQxaia6faQL9QdaQG1!!axQARLa!!AuaRLQADQALQG93!xQxaGaAfaQ1QX3o1RQARL9Qda!AafARuQLX1LQALQI1iQX3o1RN\"Q5!1RQP36ARu";
    const TEST_KEY: usize = 2894;

    #[test]
    fn test_cipher() {
        let ciphered_text = cipher(ORIGINAL_MESSAGE, TEST_KEY, DEFAULT_CHARSET).expect("Error getting ciphered text.");
        assert_eq!(CIPHERED_MESSAGE_KEY_2894, ciphered_text);
    }

    #[test]
    fn test_decipher() {
        let deciphered_text = decipher(CIPHERED_MESSAGE_KEY_2894, TEST_KEY, DEFAULT_CHARSET).unwrap();
        assert_eq!(ORIGINAL_MESSAGE, deciphered_text);
    }

    #[test]
    fn test_get_random_key() {
        let test_string = random_string(10);
        let key = get_random_key(DEFAULT_CHARSET);
        assert!(validate_key(key, DEFAULT_CHARSET.len()).unwrap());
        let ciphered_test_string = cipher(&test_string, key, DEFAULT_CHARSET).expect("Error getting ciphered text.");
        let recovered_string = decipher(ciphered_test_string, key, DEFAULT_CHARSET).unwrap();
        assert_eq!(test_string, recovered_string);
    }
}