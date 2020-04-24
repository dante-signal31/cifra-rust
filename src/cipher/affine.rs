/// Library to cipher and decipher texts using Affine method.

#[derive(Debug, Copy, Clone)]
enum WrongKeyCauses {
    MultiplyingKeyBelowZero,
    MultiplyingKeyZero,
    AddingKeyBelowZero,
    AddingKeyTooLong,
    KeysNotRelativelyPrime
}

struct WrongKey {
    key: usize,
    multiplying_key: usize,
    adding_key: usize,
    cause: WrongKeyCauses
}

impl WrongKey {
    /// Get because keys are wrong and a written explanation
    fn get_cause(&mut self)-> (WrongKeyCauses, &'static str){
        match self.cause {
            WrongKeyCauses::MultiplyingKeyBelowZero=> (self.cause, "Wrong key used: Multiplying key must be greater than 0."),
            WrongKeyCauses::MultiplyingKeyZero=> (self.cause, "Wrong key used: Multiplying key must be greater than 0."),
            WrongKeyCauses::AddingKeyBelowZero=> (self.cause, "Wrong key used: Multiplying key must be greater than 0."),
            WrongKeyCauses::AddingKeyTooLong=> (self.cause, "Wrong key used: Multiplying key must be greater than 0."),
            WrongKeyCauses::KeysNotRelativelyPrime=> (self.cause, "Wrong key used: Multiplying key must be greater than 0.")
        }
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
fn cipher<T, U>(text: T, key: usize, charset: U)-> String
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
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
fn decipher<T, U>(ciphered_text: T, key: usize, charset: U)-> String
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
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
fn validate_key(key: usize, charset_length: usize)-> bool {
    unimplemented!()
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
        let ciphered_text = cipher(ORIGINAL_MESSAGE, TEST_KEY, DEFAULT_CHARSET);
        assert_eq!(CIPHERED_MESSAGE_KEY_2894, ciphered_text);
    }

    #[test]
    fn test_decipher() {
        let deciphered_text = decipher(CIPHERED_MESSAGE_KEY_2894, TEST_KEY, DEFAULT_CHARSET);
        assert_eq!(ORIGINAL_MESSAGE, deciphered_text);
    }

    #[test]
    fn test_get_random_key() {
        let test_string = random_string(10);
        let key = get_random_key(DEFAULT_CHARSET);
        assert!(validate_key(key, DEFAULT_CHARSET.len()));
        let ciphered_test_string = cipher(&test_string, key, DEFAULT_CHARSET);
        let recovered_string = decipher(ciphered_test_string, key, DEFAULT_CHARSET);
        assert_eq!(test_string, recovered_string);
    }
}