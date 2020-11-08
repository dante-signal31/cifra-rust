use std::collections::HashMap;
use crate::cipher::common::{normalize_text, Counter};
use std::iter::FromIterator;


/// Module for frequency attacks.

struct LetterHistogram {
    charset: &'static str,
    total_letters: u64,
    ordered_dict: HashMap<String, u64>,
    top_matching_letters: Vec<char>,
    bottom_matching_letters: Vec<char>
}

impl LetterHistogram {

    /// Create a LetterHistogram instance.
    ///
    /// If letter stays as None instance is created reading a text. But it is not
    /// None then histogram instance is created using a dict.
    ///
    /// # Parameters:
    /// * text: Text to read.
    /// * letters: A dict with letters as keys and occurrences for values.
    /// * matching_width: Desired length for top and bottom matching list.
    /// * charset: Minimum charset expected in given text.
    ///
    /// # Returns:
    /// * A dict whose keys are detected letters and values are float ranging
    ///     from 0 to 1, being 1 as this letter is the only one in text and 0 as this
    ///     letter does not happen in this text (actually that value is
    ///     impossible because it would not exist that key). Keys are ordered from higher
    ///     value to lesser.
    fn new<T>(text: T, letters: Option<HashMap<char, u64>>,
              matching_width: usize, charset: &'static str) -> Self
    where T: AsRef<str> {
        let mut total_letters: u64 = 0;
        let mut letter_counter: Counter<char>;
        if let Some(_letters) = letters {
            letter_counter = Counter::from(&_letters);
            total_letters = _letters.values().sum();
        } else {
            let normalized_words = normalize_text(text);
            let letter_sequence = String::from_iter(normalized_words);
            letter_counter = Counter::from_iter(letter_sequence.chars());
            total_letters = letter_counter.values().sum();
        }
        let mut new_histogram = LetterHistogram {
                                charset,
                                total_letters,
                                ordered_dict: Default::default(),
                                top_matching_letters: vec![],
                                bottom_matching_letters: vec![]
                            };
        new_histogram.create_ordered_dict(letter_counter);
        new_histogram.set_matching_width(matching_width);
        new_histogram
    }

    /// Create an ordered dict ordering by values.
    ///
    /// Equal values are sorted by keys alphabetically.
    fn create_ordered_dict(&mut self, letter_counter: Counter<char>) {
        unimplemented!()
    }

    /// Set top and bottom matching to have desired length.
    ///
    /// By default top and bottom matching lists have 6 letters length, but
    /// with this method you can change that.
    fn set_matching_width(&mut self, width: usize) {
        unimplemented!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use rstest::*;
//
//     #[fixture]
//     fn language_histogram() -> LetterHistogram {
//
//     }
// }