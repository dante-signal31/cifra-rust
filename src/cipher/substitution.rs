use crate::{ErrorKind, Result, ResultExt};
use std::collections::HashSet;

/// Library to cipher and decipher texts using substitution method.

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
    let key = key.as_ref();
    let charset = charset.as_ref();
    let charset_set: HashSet<char> = charset.chars().collect();
    if key.len() != charset.len() {
        bail!(ErrorKind::WrongKeyLength(key, charset))
    } else if key.len() != charset_set.len() {
        bail!(ErrorKind::WrongKeyRepeatedCharacters(key))
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
fn cipher(text: T, key: U, charset: V) -> String
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str>{
    unimplemented!()
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
fn decipher(ciphered_text: T, key: U, charset: V) -> String
    where T: AsRef<str>,
          U: AsRef<str>,
          V: AsRef<str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const TEST_KEY: &'static str =     "lfwoayuisvkmnxpbdcrjtqeghz";
    const ORIGINAL_MESSAGE: &'static str  = "If a man is offered a fact which goes against his " /
                                    "instincts, he will scrutinize it closely, and unless " /
                                    "the evidence is overwhelming, he will refuse to believe " /
                                    "it. If, on the other hand, he is offered something which " /
                                    "affords a reason for acting in accordance to his " /
                                    "instincts, he will accept it even on the slightest " /
                                    "evidence. The origin of myths is explained in this way. " /
                                    "-Bertrand Russell";
    const CIPHERED_MESSAGE: &'static str = "Sy l nlx sr pyyacao l ylwj eiswi upar lulsxrj isr " /
                                    "sxrjsxwjr, ia esmm rwctjsxsza sj wmpramh, lxo txmarr " /
                                    "jia aqsoaxwa sr pqaceiamnsxu, ia esmm caytra " /
                                    "jp famsaqa sj. Sy, px jia pjiac ilxo, ia sr " /
                                    "pyyacao rpnajisxu eiswi lyypcor l calrpx ypc " /
                                    "lwjsxu sx lwwpcolxwa jp isr sxrjsxwjr, ia esmm " /
                                    "lwwabj sj aqax px jia rmsuijarj aqsoaxwa. Jia pcsusx " /
                                    "py nhjir sr agbmlsxao sx jisr elh. -Facjclxo Ctrramm";


    #[test]
    fn test_cipher() {
        let ciphered_text =
    }
}