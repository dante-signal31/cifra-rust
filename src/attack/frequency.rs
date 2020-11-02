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

// impl LetterHistogram {
//
//     fn new<T>(text: T, letters: Option<HashMap<char, u64>>,
//               matching_width: usize, charset: &'static str) -> Self
//     where T: AsRef<str> {
//         let mut total_letters: u64 = 0;
//         let mut letter_counter: Counter<char>;
//         if let Some(_letters) = letters {
//             total_letters = _letters.values().sum();
//             letter_counter = Counter::from_iter(_letters.iter());
//         } else {
//             let normalized_words = normalize_text(text);
//             let letter_sequence = String::from_iter(normalized_words);
//             letter_counter = Counter::from_iter(letter_sequence.chars())
//         }
//         LetterHistogram{
//             charset,
//             total_letters: 0,
//             ordered_dict: Default::default(),
//             top_matching_letters: vec![],
//             bottom_matching_letters: vec![]
//         }
//     }
// }

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