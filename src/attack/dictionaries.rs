/// Module to deal with words dictionaries.
///
/// A dictionary is a repository of distinct words present in an actual language.
use std::collections::{HashSet, HashMap};
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Error as IO_Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use diesel::RunQueryDsl;
use diesel::prelude::*;

use crate::attack::database::{Database, DatabaseSession, Language, NewLanguage, NewWord};
use crate::schema::*;
use crate::schema::languages;
use crate::schema::languages::dsl::*;
use crate::schema::words;
use crate::schema::words::dsl::*;
use diesel::result::Error::DatabaseError;
use regex::Regex;
use std::io::Read;


/// Cifra stores word dictionaries in a local database. This class
/// is a wrapper to not to deal directly with that database.
///
/// This class is intended to be used as a context manager so you don't have
/// to deal with opening and closing this dictionary. So, call this method
/// as a context manager, it will return this instance so you can call
/// further methods to manage its words.
pub struct Dictionary {
    pub language: String,
    language_id: i32,
    database: Database
}

impl Dictionary {
    /// Remove given language from database.
    ///
    /// Be aware that all its words will be removed too.
    ///
    /// # Parameters:
    /// * language: Language to remove from database.
    pub fn remove_dictionary<T>(_language: T)
        where T: AsRef<str> {
        let database = Database::new();
        diesel::delete(languages::table.filter(language.eq(_language.as_ref())))
            .execute(&database.session)
            .expect("Error deleting language");
    }

    /// Get languages dictionaries present at database.
    ///
    /// # Returns:
    /// * A list with names of dictionaries present at database.
    pub fn get_dictionaries_names()-> Vec<String> {
        let database = Database::new();
        let dictionaries_names = languages::table.select(languages::language)
            .load::<String>(&database.session)
            .expect("Language list could not be retrieved.");
        dictionaries_names
    }

    /// # Parameters:
    /// * language: Language you want to manage its words.
    /// * create: Whether this language should be created in database if not present yet.
    ///    It defaults to False. If it is set to False and language is not already present at
    ///    database then a dictionaries.NotExistingLanguage exception is raised, but if it is
    ///    set to True then language is registered in database as a new language.
    pub fn new<T>(_language: T, create: bool)-> Result<Self, NotExistingLanguage>
        where T: AsRef<str> {
        let new_language = _language.as_ref().to_string();
        let current_database = Database::new();
        let mut current_dictionary = Dictionary {
            language: new_language,
            language_id: 0,
            database:current_database
        };
        if !current_dictionary.already_created() {
            if create {
                current_dictionary.create_dictionary();
            } else {
                 return Err(NotExistingLanguage::new(&_language))
            }
        }
        current_dictionary.language_id = languages::table.filter(language.eq(&current_dictionary.language))
            .select(languages::id)
            .first::<i32>(current_dictionary.session())
            .expect("Language that does not exists in database yet.");
        Ok(current_dictionary)
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
    pub fn add_word<T>(&mut self, _word: T)
        where T: AsRef<str> {
        let new_word = NewWord {
            word: _word.as_ref(),
            language_id: self.language_id
        };
        diesel::insert_into(words::table)
            .values(&new_word)
            .execute(self.session())
            .expect("Error saving new word.");
    }

    /// Add given words to dictionary.
    ///
    /// # Parameters:
    /// * words: Set of words to add to dictionary.
    pub fn add_multiple_words(&mut self, _words: &HashSet<String>){
        for _word in _words {
            self.add_word(_word)
        }
    }

    /// Remove given word from dictionary.
    ///
    /// If word is not already present at dictionary, do nothing.
    ///
    /// # Parameters:
    /// * word: word to remove from dictionary.
    pub fn remove_word<T>(&mut self, _word: T)
        where T: AsRef<str> {
        diesel::delete(words::table.filter(word.eq(_word.as_ref()).and(language_id.eq(&self.language_id))))
            .execute(self.session())
            .expect("Error deleting word");
    }

    /// Check if given word exists at this dictionary.
    ///
    /// # Parameters:
    /// * word: word to check.
    ///
    /// # Returns:
    /// True if word is already present at dictionary, False otherwise.
    pub fn word_exists<T>(&self, _word: T) -> bool
        where T: AsRef<str> {
        if let Ok(count) = words::table.filter(word.eq(_word.as_ref()).and(language_id.eq(&self.language_id)))
            .count()
            .first::<i64>(self.session()) {
            if count > 0 {true} else {false}
        } else {
            false
        }
    }

    /// Read a file's words and stores them at this language database.
    ///
    /// # Parameters:
    /// * file_pathname: Absolute path to file with text to analyze.
    pub fn populate<T>(&mut self, file_pathname: T)-> Result<(), IO_Error>
        where T: AsRef<Path> {
        let _words = get_words_from_text_file(file_pathname.as_ref())?;
        self.add_multiple_words(&_words);
        Ok(())
    }

    /// Check if a table for this instance language already exists at database or not.
    ///
    /// # Returns:
    /// True if a table exists for this instance language, otherwise False.
    fn already_created(&self)-> bool {
        if let Ok(_) = languages::table.filter(language.eq(&self.language))
            .select(languages::id)
            .first::<i32>(self.session()) {
            true
        } else {
            false
        }
    }

    /// Create this instance language table in database.
    fn create_dictionary(&mut self) {
        let new_language = NewLanguage {language: self.language.as_str()};
        diesel::insert_into(languages::table)
            .values(&new_language)
            .execute(self.session())
            .expect("Error saving new language.");
        self.language_id = languages::table.filter(language.eq(&self.language))
            .select(languages::id)
            .first::<i32>(self.session())
            .expect("Error getting newly created language id.");
    }
}

/// Extract words from given file.
///
/// # Parameters:
/// * param file_pathname: Absolute filename to file to be read.
///
/// # Returns:
/// A set of words normalized to lowercase and without any punctuation mark.
pub fn get_words_from_text_file<T>(file_pathname: T) -> Result<HashSet<String>, IO_Error>
    where T: AsRef<Path> {
    let mut file_content = String::new();
    let mut file_to_read = File::open(file_pathname.as_ref())?;
    file_to_read.read_to_string(&mut file_content);
    let words_set = get_words_from_text(file_content);
    Ok(words_set)
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
    let lowercase_text = text.as_ref().to_lowercase();
    let re = Regex::new(r"[^\W\d_]+")
        .expect("Invalid regex to search for normalized words.");
    let mut words_set: HashSet<String> = HashSet::new();
    for _word in re.find_iter(&lowercase_text) {
        words_set.insert(_word.as_str().to_string());
    }
    words_set
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
fn get_candidates_frecuency(_words: HashSet<String>)-> HashMap<String, f64> {
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
        let _language = language_tried.as_ref().to_string();
        NotExistingLanguage{language_tried: _language }
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
    /// IMPORTANT NOTE: Diesel uses an environment variable to store its database path. These tests
    /// set that environment variable to point to temporal folder where to store test database. Problem
    /// is that cargo test launch test concurrently so each test changes environment variable concurrently
    /// and you suffer data races, making your tests fail. So, to make these tests work right you
    /// should run cargo test with this environment variable set:
    ///
    /// RUST_TEST_THREADS=1
    ///
    /// This way cargo test run every test sequentially and there is no data race.
    use super::*;
    use std::fs::{create_dir, File, OpenOptions};
    use std::env;
    use test_common::fs::ops::{copy_files};
    use test_common::fs::tmp::TestEnvironment;
    use test_common::system::env::TemporalEnvironmentVariable;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};
    use std::io::{Write, BufReader, Read};
    use crate::attack::database;
    use std::env::temp_dir;


    const TEXT_FILE_NAME: &'static str = "text_to_load.txt";
    const ENGLISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS: &'static str = "This eBook is for the use of anyone anywhere at no cost and with
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

    const LANGUAGES: [&'static str; 4] = ["english", "spanish", "french", "german"];

    /// Class with info to use a temporary dictionaries database.
    struct LoadedDictionaries {
        pub temp_dir: PathBuf,
        pub languages: Vec<String>,
        temp_env: TestEnvironment,
        temp_env_var: TemporalEnvironmentVariable
    }

    impl LoadedDictionaries {
        pub fn new()-> Self {
            let (temp_env, temp_env_var) = temporary_database_folder(None);
            database::create_database();
            let temp_dir = temp_env.path().to_owned();
            let mut resources_path = temp_dir.clone();
            resources_path.push("resources");
            create_dir(&resources_path);
            let mut source_path = env::current_dir()
                .expect("Could not get current working dir");
            source_path.push("resources");
            copy_files(LANGUAGES.iter()
                .map(|x| format!("{}/{}_book.txt", source_path.to_str().expect("Path contains non unicode characters"), x))
                .collect(),
                       resources_path.as_path().as_os_str().to_str()
                           .expect("Path contains not unicode characters."))
                .expect("Error copying books to temporal folder.");
            for _language in LANGUAGES.iter() {
                let mut dictionary = Dictionary::new(_language, true)
                    .expect(format!("No dictionary found for {} language.", _language).as_str());
                let mut language_book = resources_path.clone();
                language_book.push(format!("{}_book.txt", _language));
                dictionary.populate(language_book);
            }
            let mut _languages = Vec::new();
            LANGUAGES.iter().map(|x| _languages.push(x.to_string())).collect::<Vec<_>>();
            LoadedDictionaries{
                temp_dir,
                languages: _languages,
                temp_env,
                temp_env_var
            }
        }
    }

    /// Get tuples with a language name, a text with punctuations marks and a text without it.
    fn get_text_tuples()-> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("english", ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS, ENGLISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS),
            ("spanish", SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS, SPANISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS),
            ("french", FRENCH_TEXT_WITH_PUNCTUATIONS_MARKS, FRENCH_TEXT_WITHOUT_PUNCTUATIONS_MARKS),
            ("german", GERMAN_TEXT_WITH_PUNCTUATIONS_MARKS, GERMAN_TEXT_WITHOUT_PUNCTUATIONS_MARKS)]
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
    fn loaded_dictionary_temp_dir()-> (TestEnvironment, TemporalEnvironmentVariable) {
        let (temp_env, temp_env_database_path) = temporary_database_folder(None);
        database::create_database();
        let micro_dictionaries= get_micro_dictionaries();
        // let temp_env = TestEnvironment::new();
        for (_language, _words) in &micro_dictionaries {
            let mut language_dictionary = Dictionary::new(_language, true)
                .expect(format!("Dictionary not found for {} language", _language).as_str());
            _words.iter().map(|_word| language_dictionary.add_word(_word)).collect::<Vec<_>>();
        }
        for (_language, _words) in micro_dictionaries {
            let language_dictionary = Dictionary::new(_language, false)
                .expect(format!("Dictionary not found for {} language", _language).as_str());
            assert!(_words.iter().all(|_word| language_dictionary.word_exists(_word)));
        }
        (temp_env, temp_env_database_path)
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
        pub fn new<T, U, V, W>(temp_dir: T, text: U, normalized_text: V, language_name: W)-> Self
            where T: AsRef<Path>,
                  U: AsRef<str>,
                  V: AsRef<str>,
                  W: AsRef<str> {
            let mut temporary_text_file_pathname = PathBuf::from(temp_dir.as_ref().as_os_str());
            temporary_text_file_pathname.push(TEXT_FILE_NAME);
            let mut text_file = OpenOptions::new()
                                            .write(true)
                                            .create(true)
                                            .truncate(true)
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
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        match Dictionary::new("english", false) {
            Ok(_)=> assert!(false),
            Err(_)=> assert!(true)
        }
    }

    #[test]
    fn test_open_existing_dictionary() {
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        database::create_database();
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
        database::create_database();
        let _word = "test";
        let mut english_dictionary = Dictionary::new("english", true)
            .expect("Error opening dictionary");
        assert!(!english_dictionary.word_exists(_word));
        english_dictionary.add_word(_word);
        assert!(english_dictionary.word_exists(_word));
        english_dictionary.remove_word(_word);
        assert!(!english_dictionary.word_exists(_word));
    }

    #[test]
    /// Test a new language creation at database.
    fn test_create_language() {
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        let mut english_dictionary = Dictionary {
            language: "english".to_string(),
            language_id: 0,
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
        let (temp_dir, temp_env_database_path) = loaded_dictionary_temp_dir();
        let language_to_remove = "german";
        Dictionary::remove_dictionary(language_to_remove);
        // Check all words from removed language have been removed too.
        let not_existing_dictionary = Dictionary {
            language: language_to_remove.to_string(),
            language_id: 0,
            database: database::create_database()
        };
        let micro_dictionary = micro_dictionaries.get(language_to_remove)
            .expect("Error opening dictionary to be removed");
        assert!(micro_dictionary.iter().all(|_word| !not_existing_dictionary.word_exists(_word)));
    }

    #[test]
    fn test_get_words_from_text_file() {
        let temp_dir = TestEnvironment::new();
        let text_tuples = get_text_tuples();
        for (language_name, text_with_puntuation_marks, text_without_punctuation_marks) in text_tuples {
            let temporary_text = TemporaryTextFile::new(&temp_dir,
                                                        text_with_puntuation_marks,
                                                        text_without_punctuation_marks,
                                                        language_name);
            let mut expected_set = HashSet::new();
            temporary_text.normalized_text.to_lowercase().split_ascii_whitespace().map(|_word| expected_set.insert(_word.to_string())).collect::<Vec<_>>();
            let returned_set = get_words_from_text_file(temporary_text.temp_filename)
                .expect("Error reading text file");
            let mut diff: Vec<String> = Vec::new();
            for x in returned_set.symmetric_difference(&expected_set){
                diff.push(x.clone());
            }
            assert_eq!(expected_set, returned_set);
        }
    }

    #[test]
    fn test_populate_words_from_text_files() {
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        let mut temporary_text_file = TemporaryTextFile::new(temp_dir,
                                                         ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS,
                                                         ENGLISH_TEXT_WITHOUT_PUNCTUATIONS_MARKS,
                                                         "english");
        let mut expected_set = HashSet::new();
        let mut file_content = String::new();
        temporary_text_file.text_file.read_to_string(&mut file_content);
        let lowercase_content = file_content.to_lowercase();
        lowercase_content.split_ascii_whitespace().map(|x| expected_set.insert(x)).collect::<Vec<_>>();
        {
            let mut dictionary = Dictionary::new(&temporary_text_file.language_name, false)
                .expect("Error opening dictionary");
            dictionary.populate(temporary_text_file.temp_filename.as_path());
        }
        {
            let dictionary = Dictionary::new(&temporary_text_file.language_name, false)
                .expect("Error opening dictionary");
            assert!(expected_set.iter().all(|_word| dictionary.word_exists(_word)));
        }
    }

    #[test]
    fn test_get_words_from_text() {
        let test_tuples = get_text_tuples();
        for test_tuple in test_tuples {
            let mut expected_set = HashSet::new();
            test_tuple.2.to_lowercase().split_ascii_whitespace().map(|_word| expected_set.insert(_word.to_string())).collect::<Vec<_>>();
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
        let (temp_dir, temp_env_database_path) = temporary_database_folder(None);
        database::create_database();
        let _language = "english";
        let micro_dictionaries = get_micro_dictionaries();
        let mut words_to_add: HashSet<String> = HashSet::new();
        micro_dictionaries[_language].iter().map(|_word| words_to_add.insert(_word.clone())).collect::<Vec<_>>();
        let mut dictionary = Dictionary::new(_language, true)
            .expect("Error opening dictionary.");
        assert!(!micro_dictionaries[_language].iter().all(|_word| dictionary.word_exists(_word)));
        dictionary.add_multiple_words(&words_to_add);
        assert!(micro_dictionaries[_language].iter().all(|_word| dictionary.word_exists(_word)));
    }

    #[test]
    fn test_identify_language() {
        let loaded_dictionaries = LoadedDictionaries::new();
        let test_cases = vec![(ENGLISH_TEXT_WITH_PUNCTUATIONS_MARKS, "english"),
                              (SPANISH_TEXT_WITH_PUNCTUATIONS_MARKS, "spanish")];
        for (text, _language) in test_cases{
            let identified_language = identify_language(text);
            if let Some(winner) = identified_language.winner {
                assert_eq!(winner, _language, "Language not correctly identified.");
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