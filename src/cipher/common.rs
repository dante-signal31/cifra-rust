/// Common functions to be used across cipher modules.

const DEFAULT_CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

enum Ciphers {
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
fn offset_text<T, U>(text: T, key: usize, advance: bool, cipher_used: Ciphers, charset: U) -> String
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
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
fn get_new_char_position<T>(char: T, key: usize, advance: bool, cipher_used: Ciphers, charset: U) -> usize
    where T: AsRef<str>,
          U: AsRef<str> {
    unimplemented!()
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
fn get_offset_position(current_position: usize, key: usize, advance: bool, cipher_used: Ciphers, charset_length: usize)-> usize{
    unimplemented!()
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
    unimplemented!()
}