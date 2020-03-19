use crate::attack::database::{Database, DatabaseSession};
use std::collections::{HashSet, HashMap};
use std::path::{PathBuf, Path};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Module to deal with words dictionaries.
///
/// A dictionary is a repository of distinct words present in an actual language.

/// Cifra stores word dictionaries in a local database. This class
/// is a wrapper to not to deal directly with that database.
///
/// This class is intended to be used as a context manager so you don't have
/// to deal with opening and closing this dictionary. So, call this method
/// as a context manager, it will return this instance so you can call
/// further methods to manage its words.
pub struct Dictionary {
    pub language: String,
    database: Database
}

impl Dictionary {
    /// Remove given language from database.
    ///
    /// Be aware that all its words will be removed too.
    ///
    /// # Parameters:
    /// * language: Language to remove from database.
    pub fn remove_dictionary<T>(language: T)
        where T: AsRef<str> {
        unimplemented!();
    }

    /// Get languages dictionaries present at database.
    ///
    /// # Returns:
    /// * A list with names of dictionaries present at database.
    pub fn get_dictionaries_names()-> Vec<String> {
        unimplemented!();
    }

    /// # Parameters:
    /// * language: Language you want to manage its words.
    /// * create: Whether this language should be created in database if not present yet.
    ///    It defaults to False. If it is set to False and language is not already present at
    ///    database then a dictionaries.NotExistingLanguage exception is raised, but if it is
    ///    set to True then language is registered in database as a new language.
    pub fn new<T>(language: T, create: bool)-> Result<Self, NotExistingLanguage>
        where T: AsRef<str> {
        // let language = language.as_ref().to_string();
        // Ok(Dictionary {
        //     language,
        //     database
        //     })
        unimplemented!()
    }

    /// Get open session for current dictionary database.
    pub fn session(&self) -> &DatabaseSession {
        &self.database.session
    }

    /// Add given word to dictionary.
    ///
    /// If word is already present at dictionary, do nothing.
    ///
    /// # Parameters:
    /// * word: word to add to dictionary.
    pub fn add_word<T>(&mut self, word: T)
        where T: AsRef<str> {
        unimplemented!()
    }

    /// Add given words to dictionary.
    ///
    /// # Parameters:
    /// * words: Set of words to add to dictionary.
    pub fn add_multiple_words(&mut self, words: HashSet<String>){
        unimplemented!()
    }

    /// Remove given word from dictionary.
    ///
    /// If word is not already present at dictionary, do nothing.
    ///
    /// # Parameters:
    /// * word: word to remove from dictionary.
    pub fn remove_word<T>(&mut self, word: T)
        where T: AsRef<str> {
        unimplemented!()
    }

    /// Check if given word exists at this dictionary.
    ///
    /// # Parameters:
    /// * word: word to check.
    ///
    /// # Returns:
    /// True if word is already present at dictionary, False otherwise.
    pub fn word_exists<T>(&self, word: T) -> bool
        where T: AsRef<str> {
        unimplemented!()
    }

    /// Read a file's words and stores them at this language database.
    ///
    /// # Parameters:
    /// * file_pathname: Absolute path to file with text to analyze.
    pub fn populate<T>(&mut self, file_pathname: T)
        where T: AsRef<Path> {
        unimplemented!()
    }

    /// Check if a table for this instance language already exists at database or not.
    ///
    /// # Returns:
    /// True if a table exists for this instance language, otherwise False.
    fn already_created(&self)-> bool {
        unimplemented!()
    }

    /// Create this instance language table in database.
    fn create_dictionary(&mut self) {

    }
}

/// Extract words from given file.
///
/// # Parameters:
/// * param file_pathname: Absolute filename to file to be read.
///
/// # Returns:
/// A set of words normalized to lowercase and without any punctuation mark.
pub fn get_words_from_text_file<T>(file_pathname: T) -> HashSet<String>
    where T: AsRef<Path> {
    unimplemented!()
}

/// Extract words from given text.
///
/// Extracted words are normalized to lowercase and any punctuation mark
/// adjacent to words are removed.
///
/// # Parameters:
/// * text: Text to extract words from.
///
/// # Returns:
/// A set of words normalized to lowercase and without any punctuation mark.
pub fn get_words_from_text<T>(text: T)-> HashSet<String>
    where T: AsRef<str> {
    unimplemented!()
}

/// Language selected as more likely to be the one the message is written into.
///
/// # Members:
/// * winner: Name of language more likely. If None the no proper language was found.
/// * winner_probability: Probability this language is actually de right one. If None the no proper language was found.
/// * candidates: Dict with all languages probabilities. Probabilities are floats from 0 to 1.
pub struct IdentifiedLanguage {
    winner: Option<String>,
    winner_probability: Option<f64>,
    candidates: HashMap<String, String>
}

/// Identify language used to write text.
///
/// It check each word present at text to find out if is present in any language.
/// The language that has more words is select as winner.
///
/// # Parameters:
/// * Text: Text to analyze.
///
/// # Returns:
/// * Language selected as more likely to be the one used to write text.
pub fn identify_language<T>(text: T)-> IdentifiedLanguage
    where T: AsRef<str> {
    unimplemented!()
}

/// Get frequency of presence of words in each language.
///
/// # Parameters:
/// * words: Text words.
///
/// # Returns:
/// * Dict with all languages probabilities. Probabilities are floats
///    from 0 to 1. The higher the frequency of presence of words in language
///    the higher of this probability.
fn get_candidates_frecuency(words: HashSet<String>)-> HashMap<String, f64> {
    unimplemented!()
}

/// Return candidate with highest frequency.
///
/// # Parameters:
/// * candidates: Dict with all languages probabilities. Probabilities are floats
///    from 0 to 1. The higher the frequency of presence of words in language
///    the higher of this probability
fn get_winner(candidates: HashMap<String, f64>)-> String {
    unimplemented!()
}

/// Error to alarm when you try to work with a Language that has not been created yet.
#[derive(Debug)]
pub struct NotExistingLanguage {
    language_tried: String
}

impl NotExistingLanguage {
    pub fn new<T>(language_tried: T)-> Self
        where T: AsRef<str> {
        let language = language_tried.as_ref().to_string();
        NotExistingLanguage{language_tried: language}
    }
}

impl Error for NotExistingLanguage {}

impl Display for NotExistingLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Does not exist any dictionary for {} language", self.language_tried)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PathBuf;
    use std::fs::{create_dir, File, OpenOptions};
    use test_common::fs::ops::{copy_files};
    use test_common::fs::tmp::TestEnvironment;
    use test_common::system::env::TemporalEnvironmentVariable;
    use std::ffi::OsString;
    use std::path::Path;
    use std::io::{Write, BufReader, Read};
    use crate::attack::database;
    use std::env::temp_dir;


    const TEXT_FILE_NAME: &'static str = "text_to_load.txt";
    const ENGLISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS: &'static str = "his eBook is for the use of anyone anywhere at no cost and with
almost no restrictions whatsoever You may copy it give it away or
re use it under the terms of the Project Gutenberg License included
with this eBook or online at";
    const ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS: &'static str = "This eBook is for the use of anyone anywhere at no cost and with
almost no restrictions whatsoever.You may copy it, give it away or
re-use it under the terms of the Project Gutenberg License included
with this eBook or online at 2020";
    const SPANISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS: &'static str = "Todavía lo recuerdo como si aquello hubiera sucedido ayer llegó á las
puertas de la posada estudiando su aspecto afanosa y atentamente
seguido por su maleta que alguien conducía tras él en una carretilla de
mano Era un hombre alto fuerte pesado con un moreno pronunciado
color de avellana Su trenza ó coleta alquitranada le caía sobre los
hombros de su nada limpia blusa marina Sus manos callosas destrozadas
y llenas de cicatrices enseñaban las extremidades de unas uñas rotas y
negruzcas Y su rostro moreno llevaba en una mejilla aquella gran
cicatriz de sable sucia y de un color blanquizco lívido y repugnante
Todavía lo recuerdo paseando su mirada investigadora en torno del
cobertizo silbando mientras examinaba y prorrumpiendo en seguida en
aquella antigua canción marina que tan á menudo le oí cantar después";
    const SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS: &'static str = "Todavía lo recuerdo como si aquello hubiera sucedido ayer: llegó á las
puertas de la posada estudiando su aspecto, afanosa y atentamente,
seguido por su maleta que alguien conducía tras él en una carretilla de
mano. Era un hombre alto, fuerte, pesado, con un moreno pronunciado,
color de avellana. Su trenza ó coleta alquitranada le caía sobre los
hombros de su nada limpia blusa marina. Sus manos callosas, destrozadas
y llenas de cicatrices enseñaban las extremidades de unas uñas rotas y
negruzcas. Y su rostro moreno llevaba en una mejilla aquella gran
cicatriz de sable, sucia y de un color blanquizco, lívido y repugnante.
Todavía lo recuerdo, paseando su mirada investigadora en torno del
cobertizo, silbando mientras examinaba y prorrumpiendo, en seguida, en
aquella antigua canción marina que tan á menudo le oí cantar después:";
    const FRENCH_TEXT_WITHOUT_PUNCTUATIONS_MARKS: &'static str = "Combien le lecteur tandis que commodément assis au coin de son feu
il s amuse à feuilleter les pages d un roman combien il se rend peu
compte des fatigues et des angoisses de l auteur Combien il néglige de
se représenter les longues nuits de luttes contre des phrases rétives
les séances de recherches dans les bibliothèques les correspondances
avec d érudits et illisibles professeurs allemands en un mot tout
l énorme échafaudage que l auteur a édifié et puis démoli simplement
pour lui procurer à lui lecteur quelques instants de distraction au
coin de son feu ou encore pour lui tempérer l ennui d une heure en
wagon";
    const FRENCH_TEXT_WITH_PUNCTUATIONS_MARKS: &'static str = "Combien le lecteur,--tandis que, commodément assis au coin de son feu,
il s'amuse à feuilleter les pages d'un roman,--combien il se rend peu
compte des fatigues et des angoisses de l'auteur! Combien il néglige de
se représenter les longues nuits de luttes contre des phrases rétives,
les séances de recherches dans les bibliothèques, les correspondances
avec d'érudits et illisibles professeurs allemands, en un mot tout
l'énorme échafaudage que l'auteur a édifié et puis démoli, simplement
pour lui procurer, à lui, lecteur, quelques instants de distraction au
coin de son feu, ou encore pour lui tempérer l'ennui d'une heure en
wagon!";
    const GERMAN_TEXT_WITHOUT_PUNCTUATIONS_MARKS: &'static str = "Da unser Gutsherr Mr Trelawney Dr Livesay und die übrigen Herren
mich baten alle Einzelheiten über die Schatzinsel von Anfang bis zu
Ende aufzuschreiben und nichts auszulassen als die Lage der Insel und
auch die nur weil noch ungehobene Schätze dort liegen nehme ich im
Jahre die Feder zur Hand und beginne bei der Zeit als mein Vater
noch den Gasthof Zum Admiral Benbow hielt und jener dunkle alte
Seemann mit dem Säbelhieb über der Wange unter unserem Dache Wohnung
nahm";
    const GERMAN_TEXT_WITH_PUNCTUATIONS_MARKS: &'static str = "Da unser Gutsherr, Mr. Trelawney, Dr. Livesay und die übrigen Herren
mich baten, alle Einzelheiten über die Schatzinsel von Anfang bis zu
Ende aufzuschreiben und nichts auszulassen als die Lage der Insel, und
auch die nur, weil noch ungehobene Schätze dort liegen, nehme ich im
Jahre 17.. die Feder zur Hand und beginne bei der Zeit, als mein Vater
noch den Gasthof „Zum Admiral Benbow“ hielt und jener dunkle, alte
Seemann mit dem Säbelhieb über der Wange unter unserem Dache Wohnung
nahm.";

    const TEXT_TUPLES: Vec<(&'static str, &'static str, &'static str)> = vec![
        ("english", ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS, ENGLISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS),
        ("spanish", SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS, SPANISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS),
        ("french", FRENCH_TEXT_WITH_PUNCTUATIONS_MARKS, FRENCH_TEXT_WITHOUT_PUNCTUATIONS_MARKS),
        ("german", GERMAN_TEXT_WITH_PUNCTUATIONS_MARKS, GERMAN_TEXT_WITHOUT_PUNCTUATIONS_MARKS)];

    const LANGUAGES: Vec<&str> = vec!["english", "spanish", "french", "german"];

    /// Class with info to use a temporary dictionaries database.
    struct LoadedDictionaries {
        pub temp_dir: PathBuf,
        pub languages: Vec<String>,
        temp_env: TestEnvironment
    }

    impl LoadedDictionaries {
        pub fn new()-> Self {
            let temp_env = TestEnvironment::new();
            let temp_dir = temp_env.path().to_owned();
            let mut resources_path = temp_dir.clone();
            resources_path.push("resources");
            create_dir(&resources_path);
            copy_files(LANGUAGES.iter()
                .map(|x| format!("cifra-rust/tests/resources/{}_book.txt", x).as_str())
                .collect(),
                       resources_path.as_path().as_os_str().to_str()
                           .expect("Path contains not unicode characters."))
                .expect("Error copying books to temporal folder.");
            for language in LANGUAGES {
                let mut dictionary = Dictionary::new(language, true)
                    .expect(format!("No dictionary found for {} language.", language).as_str());
                let mut language_book = resources_path.clone();
                language_book.push(format!("{}_book.txt", language));
                dictionary.populate(language_book);
            }
            let mut languages = Vec::new();
            LANGUAGES.iter().map(|x| languages.push(x.to_string())).collect::<Vec<_>>();
            LoadedDictionaries{
                temp_dir,
                languages,
                temp_env
            }
        }
    }

    /// Get a HashMap with languages as keys and a list of words for every language.
    fn get_micro_dictionaries() -> HashMap<&'static str, Vec<String>>{
        let mut micro_dictionaries: HashMap<&'static str, Vec<String>> = HashMap::new();
        micro_dictionaries.insert("english", vec!("yes".to_string(), "no".to_string(), "dog".to_string(), "cat".to_string()));
        micro_dictionaries.insert("spanish", vec!("si".to_string(), "no".to_string(), "perro".to_string(), "gato".to_string()));
        micro_dictionaries.insert("french", vec!("qui".to_string(), "non".to_string(), "chien".to_string(), "chat".to_string()));
        micro_dictionaries.insert("german", vec!("ja".to_string(), "nein".to_string(), "hund".to_string(), "katze".to_string()));
        micro_dictionaries
    }

    /// Create a dictionary at a temp dir filled with only a handful of words.
    ///
    /// # Returns:
    /// Yields created temp_dir to host temporal dictionary database.
    fn loaded_dictionary_temp_dir()-> TestEnvironment {
        let micro_dictionaries= get_micro_dictionaries();
        let temp_env = TestEnvironment::new();
        for (language, words) in &micro_dictionaries {
            let mut language_dictionary = Dictionary::new(language, true)
                .expect(format!("Dictionary not found for {} language", language).as_str());
            words.iter().map(|&word| language_dictionary.add_word(word)).collect::<Vec<_>>();
        }
        for (language, words) in micro_dictionaries {
            let language_dictionary = Dictionary::new(language, false)
                .expect(format!("Dictionary not found for {} language", language).as_str());
            assert!(words.iter().all(|&word| language_dictionary.word_exists(word)));
        }
        temp_env
    }

    /// File with denormalized text in a temporary path.
    ///
    /// Language name this text is written is is at its *language_name* attributte, while
    /// its *normalized_text* has the normalized version.
    struct TemporaryTextFile {
        pub text_file: File,
        pub normalized_text: String,
        pub language_name: String,
        pub temp_filename: PathBuf
    }

    impl TemporaryTextFile {
        pub fn new<T, U>(temp_dir: T, text: U, normalized_text: U, language_name: U)-> Self
            where T: AsRef<Path>,
                  U: AsRef<str> {
            let mut temporary_text_file_pathname = PathBuf::from(temp_dir.as_ref().as_os_str());
            temporary_text_file_pathname.push(TEXT_FILE_NAME);
            let mut text_file = OpenOptions::new()
                                            .write(true)
                                            .create(true)
                                            .open(&temporary_text_file_pathname)
                .expect("Error opening temporary text file for writing into it.");
            text_file.write_all(text.as_ref().as_bytes());
            TemporaryTextFile {
                text_file,
                normalized_text: normalized_text.as_ref().to_string(),
                language_name: language_name.as_ref().to_string(),
                temp_filename: temporary_text_file_pathname
            }
        }
    }

    impl AsRef<Path> for TemporaryTextFile {
        fn as_ref(&self) -> &Path {
            self.temp_filename.as_path()
        }
    }


    /// Creates a temporary folder and set that folder at database home.
    ///
    /// # Returns:
    /// You may not use then, but keep them in scope or temp folder will be removed
    /// and environment var to find database will be restored to its default value.
    fn temporary_database_folder(temp_dir: Option<TestEnvironment>)-> (TestEnvironment, TemporalEnvironmentVariable){
        let temp_dir = match temp_dir {
            None => TestEnvironment::new(),
            Some(test_env) => test_env
        };
        let mut temp_database_path = PathBuf::from(temp_dir.path());
        temp_database_path.push("cifra_database.sqlite");
        let temp_env_database_path = TemporalEnvironmentVariable::new(database::DATABASE_ENV_VAR,
                                                                      temp_database_path.as_os_str().to_str()
                                                                          .expect("Path contains non unicode chars."));
        (temp_dir, temp_env_database_path)
    }

    #[test]
    fn test_open_not_existing_dictionary() {
        let temp_dir = TestEnvironment::new();
        match Dictionary::new("english", false) {
            Ok(_)=> assert!(false),
            Err(_)=> assert!(true)
        }
    }

    #[test]
    fn test_open_existing_dictionary() {
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        // Create not existing language.
        {
            Dictionary::new("english", true);
        }
        // Open newly created language.
        {
            let english_dictionary = Dictionary::new("english", false)
                .expect("Error opening dictionary.");
            assert!(english_dictionary.already_created());
        }
    }

    #[test]
    /// Test if we can check for word existence, write a new word and finally delete it.
    fn test_cwd_word() {
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        let word = "test";
        let mut english_dictionary = Dictionary::new("english", true)
            .expect("Error opening dictionary");
        assert!(!english_dictionary.word_exists(word));
        english_dictionary.add_word(word);
        assert!(english_dictionary.word_exists(word));
        english_dictionary.remove_word(word);
        assert!(!english_dictionary.word_exists(word));
    }

    #[test]
    /// Test a new language creation at database.
    fn test_create_language() {
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        let mut english_dictionary = Dictionary {
            language: "english".to_string(),
            database: database::create_database()
        };
        assert!(!english_dictionary.already_created());
        english_dictionary.create_dictionary();
        assert!(english_dictionary.already_created());
    }

    #[test]
    /// Test delete a language also removes its words.
    fn test_delete_language() {
        let mut micro_dictionaries = get_micro_dictionaries();
        let loaded_dictionary = loaded_dictionary_temp_dir();
        let (temp_dir, temp_env_database_path) = temporary_database_folder(Some(loaded_dictionary));
        let language_to_remove = "german";
        Dictionary::remove_dictionary(language_to_remove);
        // Check all words from removed language have been removed too.
        let not_existing_dictionary = Dictionary {
            language: language_to_remove.to_string(),
            database: database::create_database()
        };
        let micro_dictionary = micro_dictionaries.get(language_to_remove)
            .expect("Error opening dictionary to be removed");
        assert!(micro_dictionary.iter().all(|word| !not_existing_dictionary.word_exists(word)));
    }

    #[test]
    fn test_get_words_from_text_file() {
        let temp_dir = TestEnvironment::new();
        for (language_name, text_with_puntuation_marks, text_without_punctuation_marks) in TEXT_TUPLES {
            let temporary_text = TemporaryTextFile::new(&temp_dir,
                                                        text_with_puntuation_marks,
                                                        text_without_punctuation_marks,
                                                        language_name);
            let mut expected_set = HashSet::new();
            temporary_text.normalized_text.split_ascii_whitespace().map(|word| expected_set.insert(word.to_string())).collect::<Vec<_>>();
            let returned_set = get_words_from_text_file(temporary_text.temp_filename);
            assert!(expected_set.eq(&returned_set);)
        }
    }

    #[test]
    fn test_populate_words_from_text_files() {
        let temp_dir = TestEnvironment::new();
        let mut temporary_text_file = TemporaryTextFile::new(temp_dir,
                                                         ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS,
                                                         ENGLISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS,
                                                         "english");
        let mut expected_set = HashSet::new();
        let mut file_content = String::new();
        temporary_text_file.text_file.read_to_string(&mut file_content);
        file_content.to_lowercase().split_ascii_whitespace().map(|x| expected_set.insert(x)).collect::<Vec<_>>();
        {
            let mut dictionary = Dictionary::new(&temporary_text_file.language_name, false)
                .expect("Error opening dictionary");
            dictionary.populate(temporary_text_file.temp_filename.as_path());
        }
        {
            let dictionary = Dictionary::new(&temporary_text_file.language_name, false)
                .expect("Error opening dictionary");
            assert!(expected_set.iter().all(|word| dictionary.word_exists(word)));
        }
    }

    #[test]
    fn test_get_words_from_text() {
        let test_tuples = &TEXT_TUPLES;
        for test_tuple in test_tuples {
            let mut expected_set = HashSet::new();
            test_tuple.2.to_lowercase().split_ascii_whitespace().map(|word| expected_set.insert(word.to_string())).collect::<Vec<_>>();
            let returned_set = get_words_from_text(test_tuple.1);
            assert_eq!(expected_set, returned_set);
        }
    }
    
    #[test]
    fn test_get_dictionaries_names() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let dictionaries_names = Dictionary::get_dictionaries_names();
        assert_eq!(dictionaries_names, loaded_dictionaries.languages)
    }

    #[test]
    fn test_add_multiple_words() {
        let language = "english";
        let micro_dictionaries = get_micro_dictionaries();
        let mut words_to_add: HashSet<String> = HashSet::new();
        micro_dictionaries[language].iter().map(|word| words_to_add.insert(word.clone())).collect::<Vec<_>>();
        let temp_dir = TestEnvironment::new();
        let mut dictionary = Dictionary::new(language, true)
            .expect("Error opening dictionary.");
        assert!(!micro_dictionaries[language].iter().all(|word| dictionary.word_exists(word)));
        dictionary.add_multiple_words(words_to_add);
        assert!(micro_dictionaries[language].iter().all(|word| dictionary.word_exists(word)));
    }

    #[test]
    fn test_identify_language() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let test_cases = vec![(ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS, "english"),
                              (SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS, "spanish")];
        for (text, language) in test_cases{
            let identified_language = identify_language(text);
            if let Some(winner) = identified_language.winner {
                assert_eq!(winner, language, "Language not correctly identified.");
            } else {
                assert!(false, "Language not identified")
            }
            if let Some(winner_probability) = identified_language.winner_probability {
                assert_eq!(winner_probability, 1.0, "Language probability incorrectly calculated.");
            } else {
                assert!(false, "Language probability not found.")
            }
        }
    }
}