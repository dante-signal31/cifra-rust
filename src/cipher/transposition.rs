/// Library to cipher and decipher texts using transposition method.

use crate::attack::simple_attacks::Parameters;

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
    let ciphered_text = transpose_text(&text, key, true);
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
    let deciphered_text = transpose_text(&ciphered_text, key, false);
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
///
/// # Returns:
/// * Deciphered text.
pub fn decipher_par(parameters: &Parameters)-> String {
    let ciphered_text = parameters.get_str("ciphered_text");
    let key = parameters.get_int("key");
    decipher(ciphered_text, key)
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
    let matrix = create_transposition_matrix(key, &text, ciphering);
    let populated_matrix = populate_transposition_matrix(key,
                                                         &text,
                                                         matrix,
                                                         ciphering);
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
    let (total_rows, total_columns) = get_matrix_dimensions(key, &text, ciphering);
    let mut matrix = create_matrix(total_rows, total_columns);
    matrix = set_remainder_cells(ciphering, matrix, &text);
    matrix
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

/// Create an empty transposition matrix with given dimensions.
///
/// # Parameters:
/// * rows: Amount of rows created matrix needs to have.
/// * columns: Amount of columns created matrix needs to have.
///
/// # Returns:
/// * An empty transposition matrix.
fn create_matrix(rows: usize, columns: usize) -> TranspositionMatrix {
    let mut matrix: TranspositionMatrix = Vec::with_capacity(rows);
    let blank_row = vec![Some(' '); columns];
    for _i in 0..rows {
        matrix.push(blank_row.clone());
    };
    matrix
}

/// Mark not usable cells in transposition matrix with None.
///
/// Usually, transposition matrix has more cells that those actually needed for
/// text characters. Exceeding cells should be marked as None. Be aware that
/// transposition algorithm appends exceeding cells in last row tail for
/// ciphering matrix whereas uses last column tail for deciphering matrix.
///
/// # Parameters:
/// * ciphering: If true then we are populating a transposition matrix
///      for ciphering purposes. If false then we are using this function to
///      populate a transposition matrix fro deciphering.
/// * matrix: Transposition matrix to modify. This matrix is consumed by this function.
/// * text: Text to transpose.
///
/// # Returns:
/// * A new transposition matrix with remainder cells set.
fn set_remainder_cells<T>(ciphering: bool, mut matrix: TranspositionMatrix, text: T) -> TranspositionMatrix
    where T: AsRef<str> {
    let text_length = text.as_ref().len();
    let total_rows = matrix.len();
    let total_columns = matrix[0].len();
    let remainder = (total_columns * total_rows) - text_length;
    for i in 0..remainder {
        if ciphering {
            matrix[total_rows-1][total_columns-1-i] = None;
        } else {
            matrix[total_rows-1-i][total_columns-1] = None;
        }
    }
    matrix
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
                                    mut transposition_matrix: TranspositionMatrix,
                                    ciphering: bool) -> TranspositionMatrix
    where T: AsRef<str> {
    let total_columns = if ciphering {
            key
        } else {
            (text.as_ref().len() as f64 / key as f64).ceil() as usize
        };
    let mut offset: usize = 0;
    for (index, char) in text.as_ref().chars().enumerate() {
        let (mut row, mut column) = calculate_position(index+offset, total_columns);
        if transposition_matrix[row][column] == None {
            // Actually we only get here on deciphering cases. When ciphering you
            // exhaust text characters before touching None cells, but when
            // deciphering you get can touch those cells when still distributing
            // chars through matrix. When you come across a cell marked as None
            // You should get over it and use next available cell (not marked
            // as None).
            offset += 1;
            // It's a pity but I cannot reuse row and column to get directly calculate_position()
            // return. If I do it with let I only recreate a local scope pair of variables that get
            // lost when we leave "if" scope. And I get an error if I don't use "let". So, my
            // only and dirty option is taking calculate_position() return in two auxiliary
            // variables and assign to outer variables just afterwards.
            let (_row, _column) = calculate_position(index+offset, total_columns);
            row = _row;
            column = _column;
        }
        transposition_matrix[row][column] = Some(char);
    }
    transposition_matrix
}


/// Get matrix coordinates of a given index, based on columns table.
///
/// # Parameters:
/// * index: Searched index.
/// * total_columns: How many columns per row this matrix has.
///
/// # Returns:
/// * (row, column) for given index.
fn calculate_position(index: usize, total_columns: usize) -> (usize, usize) {
    let row = (index as f64 / total_columns as f64).floor() as usize;
    let column = index % total_columns;
    (row, column)
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
    let total_columns = populated_transposition_matrix[0].len();
    let mut transposed_text = String::new();
    for i in 0..total_columns {
        for row in populated_transposition_matrix {
            if let Some(char) = row[i]{
                transposed_text.push(char);
            }
        }
    }
    transposed_text
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
                                                                           transposition_matrix,
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

    #[test]
    fn test_create_matrix() {
        let expected_matrix = vec![
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')]];
        let recovered_matrix = create_matrix(4, 8);
        assert_eq!(expected_matrix, recovered_matrix,
                   "Default created matrix is not what we were expecting.")
    }

    #[test]
    fn test_set_remainder_cells() {
        let input_matrix_ciphering = vec![
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')]];
        let expected_matrix_ciphering = vec![
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), Some(' '), None, None]];
        let recovered_matrix_ciphering = set_remainder_cells(true,
                                                             input_matrix_ciphering,
                                                             ORIGINAL_MESSAGE);
        assert_eq!(expected_matrix_ciphering, recovered_matrix_ciphering,
                   "Remainder cells were not correctly set for ciphering case.");
        let input_matrix_deciphering = vec![
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')]];
        let expected_matrix_deciphering = vec![
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), Some(' ')],
            vec![Some(' '), Some(' '), Some(' '), None],
            vec![Some(' '), Some(' '), Some(' '), None]];
        let recovered_matrix_deciphering = set_remainder_cells(false,
                                                               input_matrix_deciphering,
                                                               CIPHERED_MESSAGE_KEY_8);
        assert_eq!(expected_matrix_deciphering, recovered_matrix_deciphering,
                   "Remainder cells were not correctly set for deciphering case.");
    }

    #[test]
    fn test_calculated_position() {
        let expected_position = (1,2);
        let recovered_position = calculate_position(10, 8);
        assert_eq!(expected_position, recovered_position,
                   "Recovered position was not what we were expecting.")
    }
}

