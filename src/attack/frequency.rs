use linked_hash_map::LinkedHashMap;
use std::collections::{HashMap, HashSet};
use crate::{ErrorKind, Result, ResultExt};
use crate::cipher::common::{normalize_text, Counter};
use crate::cipher::vigenere::{DEFAULT_CHARSET, cipher, decipher};
use std::iter::FromIterator;
// use std::collections::hash_map::Keys;
use crate::{FromStr, FindFromIndex};
// use std::path::Prefix::Verbatim;


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

    /// Return frequency for given letter.
    ///
    /// Frequency is the possibility of occurrence of given letter inside a normal text.
    /// Its value goes from 0 to 1.
    ///
    /// # Parameters:
    /// * key: Letter to look its frequency for.
    ///
    /// # Returns:
    /// * Probability of occurrence of given letter.
    fn frequency<T>(&self, key: T) -> Result<f64>
        where T: AsRef<str>{
        let ocurrences = self.ordered_dict.get(&char::fromStr(key.as_ref()))
            .chain_err(|| ErrorKind::KeyError(key.as_ref().to_string(), "Error finding letter frequency.".to_string()))?;
        let frequency: f64 = *ocurrences as f64/self.total_letters as f64;
        Ok(frequency)
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
    fn match_score(one: &LetterHistogram, other: &LetterHistogram) -> u8 {
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

///  Take a text a return repeated patterns with its separations.
///
///  # Parameters:
///  * text: Text to analyze.
///  * length: Length of patterns to search for.
///
///  # Returns:
///  * A dict whose keys are found patterns and its values are a list of integers
///      with separations between found patterns.
pub fn find_repeated_sequences<T>(text: T, length: usize) -> HashMap<String, Vec<usize>>
    where T: AsRef<str> {
    let mut sequences = find_adjacent_separations(text, length);
    find_not_adjacent_separations(&mut sequences);
    sequences
}

///  Find repeated sequences of given length and separations between adjacent
///  repeated sequences.
///
///  # Parameters:
///  * text: Text to analyze.
///  * length: Length of patterns to search for.
///
///  # Returns:
///  * A dict whose keys are found patterns and its values are a list of
///      integers with separations between adjacent found patters.
fn find_adjacent_separations<T>(text: T, length: usize) -> HashMap<String, Vec<usize>>
    where T: AsRef<str> {
    let normalized_words = normalize_text(&text);
    let char_string = String::from_iter(normalized_words);
    // Calling len() only gets me bytes length, but what I need is char length.
    let char_string_length = char_string.chars()
        .map(|_| 1)
        .sum();
    let mut sequences: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, char) in char_string.chars().enumerate() {
        if i + length > char_string_length {break;}
        let sequence_to_find = &char_string[i..i + length];
        if !sequences.contains_key(sequence_to_find) {
            let mut index = i + length;
            let mut previous_index = i;
            while index < char_string_length {
                if let Some(new_index) = String::findFromIndex(&char_string,
                                                           sequence_to_find,
                                                           index) {
                    index = new_index;
                    let separation = index - previous_index;
                    if sequences.contains_key(sequence_to_find) {
                       let mut values = sequences.get_mut(sequence_to_find).unwrap();
                        values.push(separation);
                    } else {
                        sequences.insert(sequence_to_find.to_string(), vec![separation]);
                    }
                    previous_index = index;
                    index += length;
                } else {
                    break;
                }
            }
        }
    }
    sequences
}

/// Complete a dict of repeated sequences calculating separation between
/// not adjacent repetitions.
///
/// # Parameters:
/// * sequences: A dict whose keys are found patterns and its values are a list of
///      integers with separations between adjacent found patters. This dict will be
///      updated in place with calculated sequences.
fn find_not_adjacent_separations(sequences: &mut HashMap<String, Vec<usize>>) {
    for (_, separations) in sequences.iter_mut() {
        let mut not_adjacent_spaces: Vec<usize> = Vec::new();
        let sequence_length = separations.len();
        if sequence_length > 1 {
            for (i, &space) in separations.iter().enumerate() {
                for n in (i+2..sequence_length+1).rev().step_by(1){
                    let spaces_to_add: Vec<usize> = (separations[i+1..n]).to_vec();
                    let spaces_to_add_sum: usize = spaces_to_add.iter().sum();
                    not_adjacent_spaces.push(space + spaces_to_add_sum);
                }
            }
            separations.append(&mut not_adjacent_spaces)
        }
    }
}


/// Get substrings for a given step.
///
/// ```ignore
/// let ciphertext = "abc dabc dabcd abcd";
/// let substrings = get_substrings(ciphertext, 4);
/// assert_eq!(substrings[0] == "aaaa");
/// assert_eq!(substrings[1] == "bbbb");
/// assert_eq!(substrings[2] == "cccc");
/// assert_eq!(substrings[3] == "dddd");
/// ```
///
/// # Parameters:
/// * ciphertext: Text to extract substrings from.
/// * step: How many letters lap before extracting next substring letter.
///
/// # Returns:
/// * A list with substrings. This lists will have the same length as step parameter.
pub fn get_substrings<T>(ciphertext: T, step: usize) -> Vec<String>
    where T: AsRef<str> {
    let normalized_text = normalize_text(&ciphertext);
    let ciphered_stream: String = normalized_text.join("");
    let mut substrings: Vec<String> = Vec::new();
    for i in 0..step {
        let mut ciphered_stream_iter = ciphered_stream.chars();
        ciphered_stream_iter.advance_by(i);
        let substring: String = ciphered_stream_iter.step_by(step).collect();
        substrings.push(substring);
    }
    substrings
}


/// Compare a substring against a known letter histogram.
///
/// The higher the returned value the more likely this substring is from the same
/// language as reference histogram.
///
/// # Parameters:
/// * substring: String to compare.
/// * reference_histogram: Histogram to compare against.
///
/// # Returns:
/// * Score value.
fn match_substring<T>(substring: T, reference_histogram: &LetterHistogram) -> u8
    where T: AsRef<str> {
    let substring_histogram = LetterHistogram::from_text(&substring, 6, DEFAULT_CHARSET);
    let match_result = LetterHistogram::match_score(&substring_histogram, reference_histogram);
    match_result
}


/// Get the most likely letters used to get given ciphered substring in the context of
/// given language histogram.
///
/// # Parameters:
/// * substring: Ciphered substring.
/// * reference_histogram: Histogram to compare against.
///
/// # Returns:
/// * A list of letters as most likely candidates to be the key for given ciphered substring.
fn find_most_likely_subkeys<T>(substring: T, reference_histogram: &LetterHistogram) -> Result<Vec<String>>
    where T: AsRef<str> {
    let mut scores: HashMap<String, u64> = HashMap::new();
    for letter in reference_histogram.charset.chars() {
        let deciphered_text = decipher(&substring.as_ref(), &letter.to_string(), &reference_histogram.charset)?;
        let deciphered_histogram = LetterHistogram::from_text(&deciphered_text, 6, &reference_histogram.charset);
        let score = LetterHistogram::match_score(&deciphered_histogram, &reference_histogram);
        scores.insert(letter.to_string(), u64::from(score));
    }
    let scores_counter = Counter::from(&scores);
    let (_, maximum_count) = scores_counter.most_common()[0];
    let mut most_likely_subkeys: Vec<String> = scores_counter.items()
        .filter(|(_, value)| **value == *maximum_count)
        .map(|(key, value)| key)
        .cloned()
        .collect();
    most_likely_subkeys.sort();
    Ok(most_likely_subkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FromStr;
    use crate::attack::dictionaries::tests::ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS;

    use rstest::*;
    use std::fs::File;
    use std::io::Read;
    use float_cmp::{ApproxEq, F64Margin};

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
    fn test_get_letter_frequencies() {
        let text = "Aaaa bb, c, da-a. efg\r\nggg";
        let mut expected_ocurrences: LinkedHashMap<char, f64> = LinkedHashMap::new();
        expected_ocurrences.insert(char::fromStr("a"), 6_f64/16_f64);
        expected_ocurrences.insert(char::fromStr("g"), 4_f64/16_f64);
        expected_ocurrences.insert(char::fromStr("b"), 2_f64/16_f64);
        expected_ocurrences.insert(char::fromStr("c"), 1_f64/16_f64);
        expected_ocurrences.insert(char::fromStr("d"), 1_f64/16_f64);
        expected_ocurrences.insert(char::fromStr("e"), 1_f64/16_f64);
        expected_ocurrences.insert(char::fromStr("f"), 1_f64/16_f64);
        let histogram = LetterHistogram::from_text(text, 6,
                                                   DEFAULT_CHARSET);
        for (letter, frequency) in expected_ocurrences.iter() {
            let returned_frequency = histogram.frequency(letter.to_string()).unwrap();
            assert!(returned_frequency.approx_eq(*frequency, (0.0,2)));
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
        let match_score = LetterHistogram::match_score(&language_histogram, &text_histogram);
        assert_eq!(match_score, expected_match_score);
    }

    #[test]
    fn test_find_repeated_sequences() {
        let ciphered_text = "PPQCA XQVEKG YBNKMAZU YBNGBAL JON I TSZM JYIM. VRAG VOHT VRAU C TKSG. DDWUO XITLAZU VAVV RAZ C VKB QP IWPOU";
        let mut expected_patterns: HashMap<String, Vec<usize>> = HashMap::new();
        expected_patterns.insert("ybn".to_string(), vec![8]);
        expected_patterns.insert("azu".to_string(), vec![48]);
        expected_patterns.insert("vra".to_string(), vec![8, 24, 32]);
        let found_patterns = find_repeated_sequences(ciphered_text, 3);
        let found_set: HashMap<&String, HashSet<usize>> = HashMap::from_iter(found_patterns.iter()
            .map(|(pattern, separations)| (pattern, HashSet::from_iter(separations.iter().cloned()))));
        let expected_set: HashMap<&String, HashSet<usize>> = HashMap::from_iter(expected_patterns.iter()
            .map(|(pattern, separations)| (pattern, HashSet::from_iter(separations.iter().cloned()))));
        for (pattern, separations) in found_set {
            assert!(expected_set.contains_key(pattern));
            assert_eq!(*expected_set.get(pattern).unwrap(), separations);
        }
    }

    #[test]
    fn test_find_repeated_sequences_many_repetitions() {
        let ciphered_text = "PPQCAXQVEKGYBNKMAZUYBNGBALJONITSZMJYIM. VRA GVOHT VRA UCTKSG.DDWUOXITLAZUVAV VRA ZCVKBQPIWPOUX VRA WZ VRA";
        let mut expected_patterns: HashMap<String, Vec<usize>> = HashMap::new();
        expected_patterns.insert("ybn".to_string(), vec![8]);
        expected_patterns.insert("azu".to_string(), vec![48]);
        expected_patterns.insert("vra".to_string(), vec![8, 24, 16, 5, 32, 48, 53, 40, 45, 21]);
        let found_patterns = find_repeated_sequences(ciphered_text, 3);
        let found_set: HashMap<&String, HashSet<usize>> = HashMap::from_iter(found_patterns.iter()
            .map(|(pattern, separations)| (pattern, HashSet::from_iter(separations.iter().cloned()))));
        let expected_set: HashMap<&String, HashSet<usize>> = HashMap::from_iter(expected_patterns.iter()
            .map(|(pattern, separations)| (pattern, HashSet::from_iter(separations.iter().cloned()))));
        for (pattern, separations) in found_set {
            assert!(expected_set.contains_key(pattern));
            assert_eq!(*expected_set.get(pattern).unwrap(), separations);
        }
    }

    #[test]
    fn test_get_substrings() {
        let ciphertext = "abc dabc dabcd abcd";
        let substrings = get_substrings(&ciphertext, 4);
        assert_eq!(substrings[0], "aaaa");
        assert_eq!(substrings[1], "bbbb");
        assert_eq!(substrings[2], "cccc");
        assert_eq!(substrings[3], "dddd");
    }

    #[rstest]
    fn test_match_substring(language_histogram: LetterHistogram) {
        let substring = "PAEBABANZIAHAKDXAAAKIU";
        let expected_result = 4;
        let match_result = match_substring(&substring, &language_histogram);
        assert_eq!(match_result, expected_result);
    }

    #[rstest]
    fn test_most_likely_subkey(language_histogram: LetterHistogram) {
        let ciphered_substring = "PAEBABANZIAHAKDXAAAKIU";
        let expected_result = vec!["p", "t", "w", "x"];
        let result = find_most_likely_subkeys(&ciphered_substring, &language_histogram).unwrap();
        assert_eq!(result, expected_result)
    }
}