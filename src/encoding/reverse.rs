use std::ops::Add;

/// Reverse encoding functions.
///
/// Reverse encoding is the simplest of encoding methods. It just reverses order of
/// text characters.


/// Reverse order of given text characters.
///
/// # Parameters:
/// * text: Text to reverse.
///
/// # Returns:
/// * Reversed text.
fn encode<T>(text: T)-> String
    where T: AsRef<str> {
    let mut reversed_text = String::new();
    for character in (text.as_ref() as &str).chars().rev(){
        reversed_text = reversed_text.add(character.to_string().as_str());
    }
    reversed_text
}

/// Obtain original text from a reversed text.
///
/// # Parameters:
/// * text: Reversed text.
///
/// # Returns:
/// * Original text.
fn decode<T>(text: T)-> String
    where T: AsRef<str> {
    encode(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_MESSAGE: &str = "Three can keep a secret, if two of them are dead.";
    const REVERSED_MESSAGE: &str = ".daed era meht fo owt fi ,terces a peek nac eerhT";

    #[test]
    fn test_reverse_encode() {
        let encoded_text = encode(ORIGINAL_MESSAGE);
        assert_eq!(REVERSED_MESSAGE, encoded_text,
                   "Expected text:\n\t{}\nBut recovered was:\n\t{}",
                   REVERSED_MESSAGE, encoded_text);
    }

    #[test]
    fn test_reverse_decode() {
        let decoded_text = decode(REVERSED_MESSAGE);
        assert_eq!(ORIGINAL_MESSAGE, decoded_text,
                "Expected text:\n\t{}\nBut recovered was:\n\t{}",
                ORIGINAL_MESSAGE, decoded_text);
    }
}
