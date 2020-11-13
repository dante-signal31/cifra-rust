use linked_hash_map::LinkedHashMap;
use std::collections::{HashMap, HashSet};
use crate::cipher::common::{normalize_text, Counter};
use crate::cipher::vigenere::DEFAULT_CHARSET;
use std::iter::FromIterator;
use std::collections::hash_map::Keys;
use crate::FromStr;
use std::path::Prefix::Verbatim;


/// Module for frequency attacks.

struct LetterHistogram {
    charset: &'static str,
    total_letters: u64,
    ordered_dict: LinkedHashMap<char, u64>,
    top_matching_letters: Vec<char>,
    bottom_matching_letters: Vec<char>
}

impl LetterHistogram {

    /// Create a LetterHistogram instance.
    ///
    /// # Parameters:
    /// * text: Text to read.
    /// * matching_width: Desired length for top and bottom matching list.
    /// * charset: Minimum charset expected in given text.
    ///
    /// # Returns:
    /// * A dict whose keys are detected letters and values are float ranging
    ///     from 0 to 1, being 1 as this letter is the only one in text and 0 as this
    ///     letter does not happen in this text (actually that value is
    ///     impossible because it would not exist that key). Keys are ordered from higher
    ///     value to lesser.
    fn from_text<T>(text: T,
                    matching_width: usize, charset: &'static str) -> Self
        where T: AsRef<str> {
        let normalized_words = normalize_text(text);
        let letter_sequence = String::from_iter(normalized_words);
        let letter_counter = Counter::from_iter(letter_sequence.chars());
        let total_letters: u64 = letter_counter.values().sum();
        let new_histogram = LetterHistogram {
                                charset,
                                total_letters,
                                ordered_dict: Default::default(),
                                top_matching_letters: vec![],
                                bottom_matching_letters: vec![]
                            };
        new_histogram.setup_for_matching(letter_counter, matching_width)
    }

    /// Create a LetterHistogram instance.
    ///
    /// # Parameters:
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
    fn from_dict(letters: HashMap<char, u64>,
                 matching_width: usize, charset: &'static str) -> Self {
        let total_letters: u64 = letters.values().sum();
        let letter_counter = Counter::from(&letters);
        let new_histogram = LetterHistogram {
            charset,
            total_letters,
            ordered_dict: Default::default(),
            top_matching_letters: vec![],
            bottom_matching_letters: vec![]
        };
        new_histogram.setup_for_matching(letter_counter, matching_width)
    }

    /// Setup histogram inners to be ready to perform comparisons with other histograms.
    ///
    /// # Parameters:
    /// * counter: A Counter type with char ocurrences.
    /// * width: Desired length for top and bottom matching list.
    ///
    /// # Returns:
    /// * This histogram ready for comparisons.
    fn setup_for_matching(self, letter_counter: Counter<char>, width: usize) -> Self {
            self.create_ordered_dict(letter_counter)
                .set_matching_width(width)
    }

    /// Create an ordered dict ordering by values.
    ///
    /// Equal values are sorted by keys alphabetically.
    ///
    /// # Parameters:
    /// * counter: A Counter type with char ocurrences.
    ///
    /// # Returns:
    /// * This histogram with an ordered dict with ocurrences.
    fn create_ordered_dict(mut self, letter_counter: Counter<char>) -> Self {
        let most_common_letters = letter_counter.most_common();
        // Standard HshMaps don't keep insertion order so I must use LinkedHashMap.
        let mut ordered_dict_by_values: LinkedHashMap<char, u64> = LinkedHashMap::from_iter(
            most_common_letters.iter()
                .map(|(key, value)| (**key, **value))
                .collect::<Vec<(char, u64)>>()
        );
        let charset_letters_not_in_text: Vec<char> = self.charset
            .chars()
            .filter(|ch|
                !ordered_dict_by_values.contains_key(&char::fromStr(ch.to_lowercase().to_string().as_str()))
                    && ch.is_alphabetic())
            .map(|ch| char::fromStr(ch.to_lowercase().to_string().as_str())).collect();
        for letter in charset_letters_not_in_text {
            ordered_dict_by_values.insert(letter, 0);
        }
        let values_set: HashSet<&u64> = HashSet::from_iter(ordered_dict_by_values.values());
        let mut values_ordered: Vec<&u64> = values_set.into_iter().collect();
        values_ordered.sort_by(|&item_A, &item_B| item_B.cmp(item_A));
        let mut key_bins: Vec<Vec<&char>> = Vec::new();
        for value in values_ordered {
            let bin: Vec<&char> = ordered_dict_by_values.iter()
                .filter(|(&key, &_value)| _value == *value)
                .map(|(key, _)| key)
                .collect();
            key_bins.push(bin);
        }
        // Book orders bins using reverse order of every char in english histogram as key.
        // Problem is that I don't want to link text histogram to any specific language
        // histogram because I want to develop a language agnostic algorithm.
        // So I just order bins using default alphabetical order key.
        key_bins.iter_mut().for_each(|v| v.sort());
        let keys_ordered: Vec<&char> = key_bins.iter()
            .flat_map(|v| v.iter().map(|&ch| ch))
            .collect();
        keys_ordered.iter().for_each(|&&key| {
            let _ = self.ordered_dict.insert(key,ordered_dict_by_values[&key]);
        });
        self
    }

    /// Set top and bottom matching to have desired length.
    ///
    /// By default top and bottom matching lists have 6 letters length, but
    /// with this method you can change that.
    ///
    /// # Parameters:
    /// * width: Desired length for top and bottom matching list.
    ///
    /// # Returns:
    /// * This histogram with top and bottom matching lists ready for comparisons.
    fn set_matching_width(mut self, width: usize) -> Self{
        self.top_matching_letters = self.ordered_dict.iter()
            .map(|(key, value)| key)
            .take(width)
            .cloned()
            .collect();
        let mut ordered_dict_iter = self.ordered_dict.iter();
        ordered_dict_iter.advance_by(self.ordered_dict.len()-width);
        self.bottom_matching_letters = ordered_dict_iter
            .map(|(key, value)| key)
            .take(width)
            .cloned()
            .collect();
        self
    }

    /// Return letters whose occurrences we have.
    fn letters(&self) -> linked_hash_map::Keys<char, u64> {
        self.ordered_dict.keys()
    }

    /// Compare two LetterHistogram instances.
    ///
    /// Score is calculated counting how many letters are in matching extremes of
    /// both instances. A coincidence is counted only if is present in top matching
    /// in both instances or in bottom matching in both instances.
    ///
    /// If matching extremes are of X length, then maximum score is of 2 * X.
    ///
    /// # Parameters:
    /// * one: First instance to compare.
    /// * other: Second instance to compare.
    ///
    /// # Returns:
    /// * Integer score. The higher the more coincidence between two instances.
    fn match_score(one: LetterHistogram, other: LetterHistogram) -> u8 {
        let top_match: u8 = one.top_matching_letters.iter()
            .filter(|letter| other.top_matching_letters.contains(*letter))
            .map(|_| 1)
            .sum();
        let bottom_match: u8 = one.bottom_matching_letters.iter()
            .filter(|letter| other.bottom_matching_letters.contains(*letter))
            .map(|_| 1)
            .sum();
        top_match + bottom_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FromStr;

    use rstest::*;
    use std::fs::File;
    use std::io::Read;

    #[fixture]
    fn language_histogram() -> LetterHistogram {
        let mut population_text = String::new();
        let mut file_to_read = File::open("resources/english_book.txt")
            .expect("Error opening english book.");
        file_to_read.read_to_string(&mut population_text)
            .expect("Error reading english book content.");
        let language_histogram = LetterHistogram::from_text(population_text,
                                                            6,
                                                            DEFAULT_CHARSET);
        language_histogram
    }

    #[test]
    fn test_get_letter_ocurrences() {
        let text = "Aaaa bb, c, da-a. efg\r\nggg";
        let mut expected_ocurrences: LinkedHashMap<char, u64> = LinkedHashMap::new();
        expected_ocurrences.insert(char::fromStr("a"), 6);
        expected_ocurrences.insert(char::fromStr("g"), 4);
        expected_ocurrences.insert(char::fromStr("b"), 2);
        expected_ocurrences.insert(char::fromStr("c"), 1);
        expected_ocurrences.insert(char::fromStr("d"), 1);
        expected_ocurrences.insert(char::fromStr("e"), 1);
        expected_ocurrences.insert(char::fromStr("f"), 1);
        let histogram = LetterHistogram::from_text(text, 6,
                                                   DEFAULT_CHARSET);
        for (letter, ocurrences) in expected_ocurrences.iter() {
            assert_eq!(histogram.ordered_dict.get(letter).unwrap(), ocurrences);
        }
        let expected_letters: Vec<&char> = expected_ocurrences.keys().collect();
        let returned_letters: Vec<&char> = histogram.letters().collect();
        for i in 0..3 {
            assert_eq!(returned_letters[i], expected_letters[i])
        }
    }

    #[test]
    fn test_set_matching_width() {
        let text = "Aaaa bb, c, da-a. efg\r\nggg";
        let expected_top = vec![char::fromStr("a"),
                                char::fromStr("g"),
                                char::fromStr("b")];
        let expected_bottom = vec![char::fromStr("x"),
                                   char::fromStr("y"),
                                   char::fromStr("z")];
        let frequencies = LetterHistogram::from_text(text,
                                                    3,
                                                    DEFAULT_CHARSET);
        assert_eq!(frequencies.top_matching_letters, expected_top);
        assert_eq!(frequencies.bottom_matching_letters, expected_bottom);
    }

    #[rstest]
    fn test_match_score(language_histogram: LetterHistogram) {
        let  text = "Sy l nlx sr pyyacao l ylwj eiswi upar lulsxrj isr sxrjsxwjr, ia esmm
            rwctjsxsza sj wmpramh, lxo txmarr jia aqsoaxwa sr pqaceiamnsxu, ia
            esmm caytra jp famsaqa sj. Sy, px jia pjiac ilxo, ia sr pyyacao
            rpnajisxu eiswi lyypcor l calrpx ypc lwjsxu sx lwwpcolxwa jp isr
            sxrjsxwjr, ia esmm lwwabj sj aqax px jia rmsuijarj aqsoaxwa. Jia
            pcsusx py nhjir sr agbmlsxao sx jisr elh. -Facjclxo Ctrramm";
        let expected_match_score = 5;
        let text_histogram = LetterHistogram::from_text(text,
                                                        6,
                                                        DEFAULT_CHARSET);
        let match_score = LetterHistogram::match_score(language_histogram, text_histogram);
        assert_eq!(match_score, expected_match_score);
    }
}