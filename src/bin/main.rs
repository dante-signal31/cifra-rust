extern crate cifra_rust;

use std::convert::TryFrom;
use std::path::PathBuf;
use clap::{Arg, App, ArgMatches};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use cifra_rust::cipher::common::DEFAULT_CHARSET;

/// Get an string containing current app version.
///
/// # Returns:
/// * This app's current version.
fn get_version()-> String {
    format!("{}.{}.{}{}",
    env!("CARGO_PKG_VERSION_MAJOR"),
    env!("CARGO_PKG_VERSION_MINOR"),
    env!("CARGO_PKG_VERSION_PATCH"),
    option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
}

/// Ciphering algorithms cifra-rust understand about.
#[derive(EnumIter, Debug, PartialEq)]
enum CipheringAlgorithms {
    Caesar,
    Substitution,
    Transposition,
    Affine,
    Vigenere,
}

impl CipheringAlgorithms {
    pub fn get_all_possible_values()-> Vec<String>{
        let mut values: Vec<String> = Vec::new();
        for algorithm in CipheringAlgorithms::iter() {
            let algorithm_str = format!("{:?}", algorithm);
            values.push(algorithm_str.to_lowercase())
        }
        values
    }
}

impl TryFrom<&str> for CipheringAlgorithms {
    type Error = String;

    /// Get a CipheringAlgorithm variant depending on a provided string name.
    ///
    /// It is used to get a CipheringAlgorithm variant from a console argument.
    ///
    /// # Parameters:
    /// * value: Algorithm name.
    ///
    /// # Returns:
    /// * OK with variant or Err if provided name is not a known variant.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized_value = value.to_lowercase();
        match normalized_value.as_str() {
            "caesar"=> Ok(CipheringAlgorithms::Caesar),
            "substitution"=> Ok(CipheringAlgorithms::Substitution),
            "transposition"=> Ok(CipheringAlgorithms::Transposition),
            "affine"=> Ok(CipheringAlgorithms::Affine),
            "vigenere"=> Ok(CipheringAlgorithms::Vigenere),
            _=> Err(format!("Unknown algorithm: {}", value))
        }
    }
}



/// Root abstraction for cifra-rust functioning modes.
#[derive(Debug, PartialEq)]
enum Modes {
    Dictionary(DictionaryActions),
    Cipher{algorithm: CipheringAlgorithms, key: String, file_to_cipher: PathBuf, ciphered_file: Option<PathBuf>,
        charset: Option<String>},
    Decipher{algorithm: CipheringAlgorithms, key: String, file_to_decipher: PathBuf, deciphered_file: Option<PathBuf>,
        charset: Option<String>},
    Attack{algorithm: CipheringAlgorithms, file_to_attack: PathBuf, deciphered_file: Option<PathBuf>,
        charset: Option<String>},
}

/// What you can do with a dictionary.
#[derive(Debug, PartialEq)]
enum DictionaryActions {
    Create{dictionary_name: String, initial_words_file: Option<PathBuf>},
    Delete{dictionary_name: String},
    Update{dictionary_name: String, words_file: PathBuf},
    List,
}

/// Configuration to run app.
#[derive(Debug, PartialEq)]
pub struct Configuration {
    running_mode: Modes,
}

impl From<ArgMatches> for Configuration {
    /// Convert clap output into a cifra configuration type.
    ///
    /// # Parameters:
    /// * matches: Output from clap parsing.
    ///
    /// # Returns:
    /// * A Configuration type.
    fn from(matches: ArgMatches) -> Self {
        if let Some(_matches) = matches.subcommand_matches("dictionary") {
            if let Some(__matches) = _matches.subcommand_matches("create"){
                return Configuration{
                    running_mode: Modes::Dictionary(DictionaryActions::Create {
                        dictionary_name: String::from(__matches.value_of("dictionary_name").unwrap()),
                        initial_words_file: if __matches.is_present("initial_words_file") {
                                Some(PathBuf::from(__matches.value_of("initial_words_file").unwrap()))
                            } else {
                                None
                            }
                        })}
            } else if let Some(__matches) = _matches.subcommand_matches("delete") {
                return Configuration{
                    running_mode: Modes::Dictionary(DictionaryActions::Delete {
                        dictionary_name: String::from(__matches.value_of("dictionary_name").unwrap())
                    })}
            } else if let Some(__matches) = _matches.subcommand_matches("update") {
                return Configuration {
                    running_mode: Modes::Dictionary(DictionaryActions::Update {
                        dictionary_name: String::from(__matches.value_of("dictionary_name").unwrap()),
                        words_file: PathBuf::from(__matches.value_of("words_file").unwrap())
                    })
                }
            } else {
                return Configuration{
                    running_mode: Modes::Dictionary(DictionaryActions::List)
                }
            }
        } else if let Some(_matches) = matches.subcommand_matches("cipher") {
            return Configuration {
                running_mode: Modes::Cipher {
                    algorithm: CipheringAlgorithms::try_from(_matches.value_of("algorithm").unwrap()).unwrap(),
                    key: String::from(_matches.value_of("key").unwrap()),
                    file_to_cipher: PathBuf::from(_matches.value_of("file_to_cipher").unwrap()),
                    ciphered_file: if _matches.is_present("ciphered_file") {
                        Some(PathBuf::from(_matches.value_of("ciphered_file").unwrap()))
                    } else {
                        None
                    },
                    charset: if _matches.is_present("charset") {
                        Some(String::from(_matches.value_of("charset").unwrap()))
                    } else {
                        None
                    }
                }
            }
        } else if let Some(_matches) = matches.subcommand_matches("decipher") {
            return Configuration {
                running_mode: Modes::Decipher {
                    algorithm: CipheringAlgorithms::try_from(_matches.value_of("algorithm").unwrap()).unwrap(),
                    key: String::from(_matches.value_of("key").unwrap()),
                    file_to_decipher: PathBuf::from(_matches.value_of("file_to_decipher").unwrap()),
                    deciphered_file: if _matches.is_present("deciphered_file") {
                        Some(PathBuf::from(_matches.value_of("deciphered_file").unwrap()))
                    } else {
                        None
                    },
                    charset: if matches.is_present("charset") {
                        Some(String::from(_matches.value_of("charset").unwrap()))
                    } else {
                        None
                    }
                }
            }
        } else {
            let _matches = matches.subcommand_matches("attack").unwrap();
            return Configuration {
                running_mode: Modes::Attack {
                    algorithm: CipheringAlgorithms::try_from(_matches.value_of("algorithm").unwrap()).unwrap(),
                    file_to_attack: PathBuf::from(_matches.value_of("file_to_attack").unwrap()),
                    deciphered_file: if _matches.is_present("deciphered_file") {
                        Some(PathBuf::from(_matches.value_of("deciphered_file").unwrap()))
                    } else {
                        None
                    },
                    charset: if _matches.is_present("charset") {
                        Some(String::from(_matches.value_of("charset").unwrap()))
                    } else {
                        None
                    }
                }
            }
        }
        }
}

/// Check that provided path actually exists.
///
/// This function is used as a validator in parse_arguments.
///
/// # Parameters:
/// * path: Absolute path to file.
///
/// # Returns:
/// * Ok(()) if file exists Err if not.
fn file_exists(path: &str)-> Result<(), String>{
    let pathbuf = PathBuf::from(path);
    if pathbuf.exists(){
        Ok(())
    } else {
        Err(format!("File does not exists: {}", path))
    }
}

/// Parse given console arguments.
///
/// # Parameters:
/// * arg_vec: Arguments to parse. Usually it will br env::args() but you may want to enter your
/// own vector to test parsing.
///
///# Returns:
/// * A cifra Configuration type.
pub fn parse_arguments(arg_vec: &Vec<&str>) -> Configuration {
    let algorithm_options = CipheringAlgorithms::get_all_possible_values();
    let algorithm_options_str: Vec<&str> = algorithm_options.iter().map(|str| str.as_str()).collect();
    let matches = App::new("cifra")
        .version(get_version().as_str())
        .author(env!("CARGO_PKG_AUTHORS"))
        .long_about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(App::new("dictionary")
                        .about("Manage dictionaries to perform crypto attacks.")
                        .subcommand(App::new("create")
                            .about("Create a dictionary of unique words.")
                            .arg(Arg::new("dictionary_name").index(1)
                                .required(true)
                                .value_name("NEW_DICTIONARY_NAME")
                                .takes_value(true)
                                .about("Name for the dictionary to create."))
                            .arg(Arg::new("initial_words_file")
                                .long("initial_words_file")
                                .short('i')
                                .value_name("PATH_TO FILE_WITH_WORDS")
                                .takes_value(true)
                                .validator(file_exists)
                                .about("Optionally you can load in the dictionary words located in a text file")))
                        .subcommand(App::new("delete")
                            .about("Remove an existing dictionary.")
                            .arg(Arg::new("dictionary_name").index(1)
                                .required(true)
                                .value_name("DICTIONARY_NAME_TO_DELETE")
                                .takes_value(true)
                                .about("Name for the dictionary to delete.")))
                        .subcommand(App::new("update")
                            .about("Add words to an existing dictionary.")
                            .arg(Arg::new("dictionary_name").index(1)
                                .required(true)
                                .value_name("DICTIONARY_NAME_TO_UPDATE")
                                .takes_value(true)
                                .about("Name for the dictionary to update with additional words."))
                            .arg(Arg::new("words_file").index(2)
                                .required(true)
                                .value_name("PATH_TO_FILE_WITH_WORDS")
                                .takes_value(true)
                                .validator(file_exists)
                                .about("Pathname to a file with words to add to dictionary")))
                        .subcommand(App::new("list")
                            .about("Show existing dictionaries.")))
        .subcommand(App::new("cipher")
            .about("Cipher a text using a key.")
            .arg(Arg::new("algorithm").index(1)
                .required(true)
                .value_name("ALGORITHM_NAME")
                .takes_value(true)
                .possible_values(algorithm_options_str.as_slice())
                .about("Algorithm to use to cipher."))
            .arg(Arg::new("key").index(2)
                .required(true)
                .value_name("CIPHERING_KEY")
                .takes_value(true)
                .about("Key to use to cipher."))
            .arg(Arg::new("file_to_cipher").index(3)
                .required(true)
                .value_name("FILE_TO_CIPHER")
                .takes_value(true)
                .validator(file_exists)
                .about("Path to file with text to cipher."))
            .arg(Arg::new("ciphered_file")
                .long("ciphered_file")
                .short('o')
                .value_name("OUTPUT_CIPHERED_FILE")
                .takes_value(true)
                .about("Path to output file to place ciphered text. If not used then ciphered text will be dumped to console."))
            .arg(Arg::new("charset")
                .long("charset")
                .short('c')
                .value_name("CHARSET")
                .takes_value(true)
                .about(&format!("Default charset is: {}, but you can set here another", DEFAULT_CHARSET))))
        .subcommand(App::new("decipher")
            .about("Decipher a text using a key.")
            .arg(Arg::new("algorithm").index(1)
                .required(true)
                .value_name("ALGORITHM_NAME")
                .takes_value(true)
                .possible_values(algorithm_options_str.as_slice())
                .about("Algorithm to use to decipher."))
            .arg(Arg::new("key").index(2)
                .required(true)
                .value_name("CIPHERING_KEY")
                .takes_value(true)
                .about("Key to use to decipher."))
            .arg(Arg::new("file_to_decipher").index(3)
                .required(true)
                .value_name("FILE_TO_DECIPHER")
                .takes_value(true)
                .validator(file_exists)
                .about("Path to file with text to decipher."))
            .arg(Arg::new("deciphered_file")
                .long("deciphered_file")
                .short('o')
                .value_name("OUTPUT_DECIPHERED_FILE")
                .takes_value(true)
                .about("Path to output file to place deciphered text. If not used then deciphered text will be dumped to console."))
            .arg(Arg::new("charset")
                .long("charset")
                .short('c')
                .value_name("CHARSET")
                .takes_value(true)
                .about(&format!("Default charset is: {}, but you can set here another", DEFAULT_CHARSET))))
        .subcommand(App::new("attack")
            .about("Attack a ciphered text to get its plain text")
            .arg(Arg::new("algorithm").index(1)
                .required(true)
                .value_name("ALGORITHM_NAME")
                .takes_value(true)
                .possible_values(algorithm_options_str.as_slice())
                .about("Algorithm to attack."))
            .arg(Arg::new("file_to_attack").index(2)
                .required(true)
                .value_name("FILE_TO_ATTACK")
                .takes_value(true)
                .validator(file_exists)
                .about("Path to file with text to attack."))
            .arg(Arg::new("deciphered_file")
                .short('o')
                .long("deciphered_file")
                .value_name("OUTPUT_DECIPHERED_FILE")
                .takes_value(true)
                .about("Path to output file to place deciphered text. If not used then deciphered text will be dumped to console."))
            .arg(Arg::new("charset")
                .short('c')
                .long("charset")
                .value_name("CHARSET")
                .takes_value(true)
                .about(&format!("Default charset is: {}, but you can set here another", DEFAULT_CHARSET))))
        .get_matches_from(arg_vec);
    let configuration = Configuration::from(matches);
    configuration
}

fn main() {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use test_common::fs::tmp::{TestEnvironment, TestFile};

    #[test]
    fn test_parser_create_dictionary() {
        let provided_args: Vec<&str> = "cifra dictionary create klingon".split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Dictionary(DictionaryActions::Create {
                dictionary_name: String::from("klingon"),
                initial_words_file: None
            })
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_create_dictionary_with_initial_file() {
        let output_file = TestFile::new();
        let command = format!("cifra dictionary create klingon --initial_words_file {}", output_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Dictionary(DictionaryActions::Create {
                dictionary_name: String::from("klingon"),
                initial_words_file: Some(PathBuf::from(output_file.path().to_str().unwrap()))
            })
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_delete_dictionary(){
        let provided_args = "cifra dictionary delete klingon".split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Dictionary(DictionaryActions::Delete {
                dictionary_name: String::from("klingon"),
            })
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_update_dictionary(){
        let words_file = TestFile::new();
        let command = format!("cifra dictionary update klingon {}", words_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Dictionary(DictionaryActions::Update {
                dictionary_name: "klingon".to_string(),
                words_file: PathBuf::from(words_file.path().to_str().unwrap()) }
            )
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_cipher_caesar() {
        let message_file = TestFile::new();
        let command = format!("cifra cipher caesar 3 {}", message_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Cipher {
                algorithm: CipheringAlgorithms::Caesar,
                key: "3".to_string(),
                file_to_cipher: PathBuf::from(message_file.path().to_str().unwrap()),
                ciphered_file: None,
                charset: None
            }
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_decipher_caesar() {
        let message_file = TestFile::new();
        let command = format!("cifra decipher caesar 3 {}", message_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Decipher {
                algorithm: CipheringAlgorithms::Caesar,
                key: "3".to_string(),
                file_to_decipher: PathBuf::from(message_file.path().to_str().unwrap()),
                charset: None,
                deciphered_file: None
            }
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_decipher_caesar_with_output_file() {
        let message_file = TestFile::new();
        let command = format!("cifra decipher caesar 3 {} --deciphered_file deciphered_message.txt", message_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Decipher {
                algorithm: CipheringAlgorithms::Caesar,
                key: "3".to_string(),
                file_to_decipher: PathBuf::from(message_file.path().to_str().unwrap()),
                charset: None,
                deciphered_file: Some(PathBuf::from("deciphered_message.txt"))
            }
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_attack_caesar() {
        let message_file = TestFile::new();
        let command = format!("cifra attack caesar {} --deciphered_file recovered_message.txt", message_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Attack {
                algorithm: CipheringAlgorithms::Caesar,
                charset: None,
                deciphered_file: Some(PathBuf::from("recovered_message.txt")),
                file_to_attack: PathBuf::from(message_file.path().to_str().unwrap())
            }
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_attack_caesar_with_charset() {
        let message_file = TestFile::new();
        let command = format!("cifra attack caesar {} --deciphered_file recovered_message.txt --charset abcdefghijklmnñopqrstuvwxyz", message_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Attack {
                algorithm: CipheringAlgorithms::Caesar,
                charset: Some(String::from("abcdefghijklmnñopqrstuvwxyz")),
                deciphered_file: Some(PathBuf::from("recovered_message.txt")),
                file_to_attack: PathBuf::from(message_file.path().to_str().unwrap())
            }
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_list_dictionaries() {
        let provided_args = "cifra dictionary list".split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Dictionary(DictionaryActions::List)
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }
}