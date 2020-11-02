use std::collections::HashMap;



// /// Module for frequency attacks.
//
// struct LetterHistogram {
//     charset: &'static str,
//     total_letters: u64,
//     ordered_dict: HashMap<String, u64>,
//     top_matching_letters: Vec<char>,
//     bottom_matching_letters: Vec<char>
// }
//
// impl LetterHistogram {
//
//     fn new<T>(text: T, letters: Option<HashMap<char, u64>>,
//               matching_width: usize, charset: &'static str) -> Self
//     where T: AsRef<str> {
//         let mut total_letters: u64 = 0;
//         let mut letter_counter: u64 = 0;
//         if let Some(_letters) = letters {
//             total_letters = _letters.values().sum();
//             // TODO: Create a Counter class. Rust has no one built-in.
//             letter_counter = Counter::new(_letters);
//         } else {
//             // TODO: Implement normalize_text().
//             let normalized_words = normalize_text(text);
//             let letter_sequence =
//
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