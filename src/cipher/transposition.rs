

/// Library to cipher and decipher texts using transposition method.

type TranspositionMatrix = Vec<Vec<Option<char>>>;

/// Cipher given text using transposition method.
///
/// # Parameters:
/// * text: Text to be ciphered.
/// * key: Secret key.
///
/// # Returns:
/// * Ciphered text.
pub fn cipher<T>(text: T, key: usize)-> String
    where T: AsRef<str> {
    let ciphered_text = transpose_text(text, key, true);
    ciphered_text
}

/// Decipher given text using transposition method.
///
/// # Parameters:
/// * ciphered_text: Text to be deciphered.
/// * key: Secret key.
///
/// # Returns:
/// * Deciphered text.
pub fn decipher<T>(ciphered_text: T, key: usize)-> String
    where T: AsRef<str> {
    let deciphered_text = transpose_text(ciphered_text, key, false);
    deciphered_text
}

/// Transpose given text.
///
/// # Parameters:
/// * text: Text to transpose.
/// * key: Key for transposition.
/// * ciphering: True if we are using transposition for ciphering. False if we are using it for
///    deciphering.
///
/// # Returns:
/// * Transposed text.
fn transpose_text<T>(text: T, key: usize, ciphering: bool)-> String
    where T: AsRef<str> {
    let mut matrix = create_transposition_matrix(key, &text, ciphering);
    let populated_matrix = populate_transposition_matrix(key, &text, &mut matrix, ciphering);
    let recovered_text = get_transposed_text(&populated_matrix);
    recovered_text
}

/// Create matrix used to store characters and perform transposition operations.
///
/// # Parameters:
/// * key: Secret key used for transposition.
/// * text: Text to transpose.
/// * ciphering: If true then we are populating a transposition matrix
///    for ciphering purposes. If false then we are using this function to
///    populate a transposition matrix from deciphering.
///
/// # Returns:
/// * Transposition matrix in its default state.
fn create_transposition_matrix<T>(key: usize, text: T, ciphering: bool) -> TranspositionMatrix
    where T: AsRef<str> {
    unimplemented!()
}


/// Get transposition matrix dimensions needed for given text and key.
///
/// # Parameters:
/// * key: Secret key used for transposition.
/// * text: Text to transpose.
/// * ciphering: If true then we are populating a transposition matrix
///      for ciphering purposes. If false then we are using this function to
///      populate a transposition matrix from deciphering.
/// # Returns:
/// * A tuple with matrix dimensions with format (rows, columns).
fn get_matrix_dimensions<T>(key: usize, text: T, ciphering: bool)-> (usize, usize)
    where T: AsRef<str> {
    let text_length = text.as_ref().len();
    let total_rows = if ciphering {
            (text_length as f64 / key as f64).ceil() as usize
        } else {
            key
        };
    let total_columns = if ciphering {
            key
        } else {
            (text_length as f64 / key as f64).ceil() as usize
        };
    (total_rows, total_columns)
}

/// Store text to cipher in transposition matrix.
///
/// # Parameters:
/// * key: Text to be ciphered.
/// * text: Secret key.
/// * transposition_matrix: Transposition matrix in its default state.
/// * ciphering: If true then we are populating a transposition matrix
///    for ciphering purposes. If false then we are using this function to
///    populate a transposition matrix fro deciphering.
///
/// # Returns:
/// * transposition_matrix with text to cipher stored inside it.
fn populate_transposition_matrix<T>(key: usize, text: T,
                                    transposition_matrix: &mut TranspositionMatrix,
                                    ciphering: bool) -> TranspositionMatrix
    where T: AsRef<str> {
    unimplemented!()
}

/// Get transposed characters from populated transposition matrix.
///
/// # Parameters:
/// * populated_transposition_matrix: Transposition matrix with text to
///    cipher stored inside it.
///
/// # Returns:
/// * Text cohered by transposition method.
fn get_transposed_text(populated_transposition_matrix: &TranspositionMatrix) -> String {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const ORIGINAL_MESSAGE: &str = "Common sense is not so common.";
    const CIPHERED_MESSAGE_KEY_8: &str = "Cenoonommstmme oo snnio. s s c";
    const TEST_KEY: usize = 8;

    #[test]
    fn test_cipher() {
        let ciphered_text = cipher(ORIGINAL_MESSAGE, TEST_KEY);
        assert_eq!(CIPHERED_MESSAGE_KEY_8, ciphered_text,
                   "Expected message was:\n\t{}\nBut ciphered was:\n\t{}\n",
                   CIPHERED_MESSAGE_KEY_8, ciphered_text)
    }

    #[test]
    fn test_decipher() {
        let deciphered_text = decipher(CIPHERED_MESSAGE_KEY_8, TEST_KEY);
        assert_eq!(ORIGINAL_MESSAGE, deciphered_text,
                   "Expected message was:\n\t{}\nBut deciphered was:\n\t{}\n",
                   ORIGINAL_MESSAGE, deciphered_text)

    }

    #[test]
    fn test_create_transposition_matrix_ciphering() {
        let expected_rows: usize = 4;
        let expected_columns: usize = 8;
        let transposition_matrix = create_transposition_matrix(TEST_KEY,
                                                               ORIGINAL_MESSAGE,
                                                               true);
        assert_eq!(expected_rows, transposition_matrix.len(),
                   "Expected rows:\n\t{}\nBut recovered matrix has:\n\t{}\n",
                   expected_rows, transposition_matrix.len());
        assert_eq!(expected_columns, transposition_matrix[0].len(),
                   "Expected columns:\n\t{}\nBut recovered matrix has:\n\t{}\n",
                   expected_columns, transposition_matrix[0].len());
        assert_eq!(None, transposition_matrix[3][6],
                   "Prior last position is not None as expected");
        assert_eq!(None, transposition_matrix[3][7],
                   "Last position is not None as expected");
    }

    #[test]
    fn test_create_transposition_matrix_deciphering() {
        let expected_columns: usize = 4;
        let expected_rows: usize = 8;
        let transposition_matrix = create_transposition_matrix(TEST_KEY,
                                                               ORIGINAL_MESSAGE,
                                                               false);
        assert_eq!(expected_rows, transposition_matrix.len(),
                   "Expected rows:\n\t{}\nBut recovered matrix has:\n\t{}\n",
                   expected_rows, transposition_matrix.len());
        assert_eq!(expected_columns, transposition_matrix[0].len(),
                   "Expected columns:\n\t{}\nBut recovered matrix has:\n\t{}\n",
                   expected_columns, transposition_matrix[0].len());
        assert_eq!(None, transposition_matrix[6][3],
                   "Prior last position is not None as expected");
        assert_eq!(None, transposition_matrix[7][3],
                   "Last position is not None as expected");
    }
    
    #[test]
    fn test_populate_transposition_matrix() {
        let mut transposition_matrix = vec![
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), None, None]];
        let expected_populated_transposition_matrix = vec![
            vec![Some('C'), Some('o'), Some('m'), Some('m'), Some('o'), Some('n'), Some(' '), Some('s')],
            vec![Some('e'), Some('n'), Some('s'), Some('e'), Some(' '), Some('i'), Some('s'), Some(' ')],
            vec![Some('n'), Some('o'), Some('t'), Some(' '), Some('s'), Some('o'), Some(' '), Some('c')],
            vec![Some('o'), Some('m'), Some('m'), Some('o'), Some('n'), Some('.'), None, None]];
        let recovered_transposition_matrix = populate_transposition_matrix(TEST_KEY,
                                                                           ORIGINAL_MESSAGE,
                                                                           &mut transposition_matrix,
                                                                           true);
        assert_eq!(expected_populated_transposition_matrix, recovered_transposition_matrix,
                   "Expected transposition matrix is not what we recovered");
    }
    
    #[test]
    fn test_get_transposed_text() {
        let transposition_matrix = vec![
            vec![Some('C'), Some('o'), Some('m'), Some('m'), Some('o'), Some('n'), Some(' '), Some('s')],
            vec![Some('e'), Some('n'), Some('s'), Some('e'), Some(' '), Some('i'), Some('s'), Some(' ')],
            vec![Some('n'), Some('o'), Some('t'), Some(' '), Some('s'), Some('o'), Some(' '), Some('c')],
            vec![Some('o'), Some('m'), Some('m'), Some('o'), Some('n'), Some('.'), None, None]];
        let recovered_text = get_transposed_text(&transposition_matrix);
        assert_eq!(CIPHERED_MESSAGE_KEY_8, recovered_text,
                   "Expected text:\n\t{}\nBut recovered text was:\n\t{}\n",
                   CIPHERED_MESSAGE_KEY_8, recovered_text);
    }

    #[test]
    fn test_get_matrix_dimensions() {
        let (rows, columns) = get_matrix_dimensions(TEST_KEY, ORIGINAL_MESSAGE, true);
        assert_eq!((4, 8), (rows, columns),
                    "Recovered dimensions were not as expected for ciphering case");
        let (rows, columns) = get_matrix_dimensions(TEST_KEY, CIPHERED_MESSAGE_KEY_8, false);
        assert_eq!((8, 4), (rows, columns),
                   "Recovered dimensions were not as expected for deciphering case");
    }
}

