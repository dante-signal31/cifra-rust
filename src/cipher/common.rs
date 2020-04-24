use std::convert::{TryFrom, TryInto};
use std::ops::Add;
use crate::cipher::cryptomath::{modulus, find_mod_inverse};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::{ErrorKind, Result, ResultExt};

/// Common functions to be used across cipher modules.

const DEFAULT_CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

#[derive(Debug)]
pub enum Ciphers {
    CAESAR,
    TRANSPOSITION,
    AFFINE
}

/// Generic function to offset text character frontwards and backwards.
///
/// # Parameters:
/// * text: Text to offset.
/// * key: Number of positions to offset characters.
/// * advance: If True offset characters frontwards.
/// * cipher_used: Kind of cipher we are using for this message.
/// * charset: Charset to use for substitution.
///
/// # Returns:
/// * Offset text.
pub fn offset_text<T, U>(text: T, key: usize, advance: bool, cipher_used: &Ciphers, charset: U) -> Result<String>
    where T: AsRef<str>,
          U: AsRef<str> {
    let mut offset_text = String::new();
    for character in text.as_ref().chars() {
        let normalized_char = character.to_lowercase().to_string();
        let new_character = match get_new_char_position(&normalized_char, key, advance, cipher_used, &charset)? {
            Some(new_char_position) => charset.as_ref().chars().nth(new_char_position).unwrap(),
            _ => character
        };
        offset_text = if character.is_lowercase() {
            offset_text.add(new_character.to_string().as_str())
        } else {
            offset_text.add(new_character.to_uppercase().to_string().as_str())
        };
    }
    Ok(offset_text)
}

/// Get position for offset char.
///
/// # Parameters:
/// * char: Actual character with no offset. It should be normalized to be
///      sure it is present at charset.
/// * key: Offset to apply.
/// * advance: If True offset is going to be applied frontwards.
/// * cipher_used: Kind of cipher we are using for this message.
/// * charset: Charset to use for substitution.
///
/// # Returns:
/// * Index in charset for offset char
fn get_new_char_position<T, U>(char: T, key: usize, advance: bool, cipher_used: &Ciphers, charset: U) -> Result<Option<usize>>
    where T: AsRef<str>,
          U: AsRef<str> {
    let charset_length = charset.as_ref().len();
    let character_to_find = match char.as_ref().chars().nth(0) {
        Some(c)=> c,
        None=> bail!(ErrorKind::StringIndexError(char.as_ref().to_string(), "Error at function get_new_char_position()"))
    };
    let char_position = match charset.as_ref().find(character_to_find) {
        Some(index) => index,
        _ => return Ok(None)
    };
    let offset_position = get_offset_position(char_position, key, advance, cipher_used, charset_length)?;
    let new_char_position = modulus(offset_position, charset_length as isize);
    // Positive operands at modulus give positive modulus result, so it can be casted to usize.
    Ok(Some(new_char_position as usize))
}

/// Get new offset depending on ciphering being used.
///
/// # Parameters:
/// * current_position: Charset index of current char we are calculating offset to.
/// * key: Key value used for this message.
/// * advance: If True offset is going to be applied frontwards, that is when you cipher.
/// * cipher_used: Kind of cipher we are using for this message.
/// * charset_length: Length of charset to use for substitution.
///
/// # Returns:
/// * New offset position for this char.
fn get_offset_position(current_position: usize, key: usize, advance: bool, cipher_used: &Ciphers, charset_length: usize)-> Result<isize> {
    let i_current_position: isize = current_position.try_into()
        .chain_err(|| ErrorKind::ConversionError("current_position", "usize", "isize"))?;
    let i_key: isize = key.try_into()
        .chain_err(|| ErrorKind::ConversionError("key", "usize", "isize"))?;
    match cipher_used {
        Ciphers::CAESAR=> if advance {Ok(i_current_position + i_key)} else {Ok(i_current_position - i_key)},
        Ciphers::AFFINE=> {
            let (multiplying_key, adding_key) = get_key_parts(key, charset_length);
            let i_multiplying_key: isize = multiplying_key.try_into()
                .chain_err(|| ErrorKind::ConversionError("multiplying_key", "usize", "isize"))?;
            let i_adding_key: isize = adding_key.try_into()
                .chain_err(|| ErrorKind::ConversionError("adding_key", "usize", "isize"))?;
            if advance {
                Ok((i_current_position * i_multiplying_key) + i_adding_key)
            } else {
                let i_multiplying_key: isize = multiplying_key.try_into()
                    .chain_err(|| ErrorKind::ConversionError("multiplying_key", "usize", "isize"))?;
                let i_charset_length = charset_length.try_into()
                    .chain_err(|| ErrorKind::ConversionError("charset_length", "usize", "isize"))?;
                Ok((i_current_position - i_adding_key) * find_mod_inverse(i_multiplying_key, i_charset_length)
                    .expect(format!("Modular inverse could not be found for {} and {}", i_multiplying_key, i_charset_length).as_ref()))
            }
        },
        _=> bail!("get_offset_position has been unexpectedly called for {:?} cipher", cipher_used)
    }
}

/// Split given key in two parts to be used by Affine cipher.
///
/// # Parameters:
/// * key: Key used for ciphering and deciphering.
/// * charset_length: Length of charset used for Affine method substitutions. Both end should
///     use the same charset or original text won't be properly recovered.
///
/// # Returns:
/// * A tuple whose first component is key used for multiplying while ciphering and second component is used for
///     adding.
fn get_key_parts(key: usize, charset_length: usize)-> (usize, usize){
    let multiplying_key = key / charset_length;
    // Operands for this modulus operation are going to be positive always, so no need
    // to use modulus function.
    let adding_key = key % charset_length;
    (multiplying_key, adding_key)
}
