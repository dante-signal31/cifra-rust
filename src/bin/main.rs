extern crate cifra;

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::str::FromStr;
use std::env::args;
use std::fmt::{Display, Formatter};
use clap::{Arg, App, ArgMatches};
use error_chain::bail;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use cifra::{ErrorKind, Result, ResultExt};
use cifra::cipher::common::DEFAULT_CHARSET;
use cifra::cipher::substitution::DEFAULT_CHARSET as SUBSTITUTION_DEFAULT_CHARSET;
use cifra::attack::dictionaries::Dictionary;



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
#[derive(EnumIter, Debug, PartialEq, Clone)]
enum CipheringAlgorithms {
    Caesar,
    Substitution,
    Transposition,
    Affine,
    Vigenere,
}

impl CipheringAlgorithms {

    /// Get a list with every possible variant this enum can adopt.
    pub fn get_all_possible_values()-> Vec<String>{
        let mut values: Vec<String> = Vec::new();
        for algorithm in CipheringAlgorithms::iter() {
            let algorithm_str = format!("{:?}", algorithm);
            values.push(algorithm_str.to_lowercase())
        }
        values
    }

    /// Get a set with every ciphering variant that uses a string as a key.
    pub fn get_string_key_algorithms()-> HashSet<String> {
        let key_algorithms: HashSet<String> = vec!["substitution", "vigenere"].into_iter()
            .map(|str| String::from(str))
            .collect();
        key_algorithms
    }

    /// Get a set with every ciphering variant that uses a integer number as a key.
    pub fn get_integer_key_algorithms()-> HashSet<String> {
        let all_algorithms: HashSet<String> = Self::get_all_possible_values().into_iter().collect();
        let key_algorithms: HashSet<String> = Self::get_string_key_algorithms();
        let integer_algorithms: HashSet<String> = all_algorithms.difference(&key_algorithms)
            .into_iter()
            .map(|Str| Str.clone())
            .collect();
        integer_algorithms
    }

    /// Get current value as an string.
    pub fn get_string_value(&self)-> String {
        format!("{:?}", self)
    }
}

impl Display for CipheringAlgorithms{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for CipheringAlgorithms {
    type Error = cifra::Error;

    /// Get a CipheringAlgorithm variant depending on a provided string name.
    ///
    /// It is used to get a CipheringAlgorithm variant from a console argument.
    ///
    /// # Parameters:
    /// * value: Algorithm name.
    ///
    /// # Returns:
    /// * OK with variant or Err if provided name is not a known variant.
    fn try_from(value: &str) -> Result<Self> {
        let normalized_value = value.to_lowercase();
        match normalized_value.as_str() {
            "caesar"=> Ok(CipheringAlgorithms::Caesar),
            "substitution"=> Ok(CipheringAlgorithms::Substitution),
            "transposition"=> Ok(CipheringAlgorithms::Transposition),
            "affine"=> Ok(CipheringAlgorithms::Affine),
            "vigenere"=> Ok(CipheringAlgorithms::Vigenere),
            _=> bail!(format!("Unknown algorithm: {}", value))
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
        output_recovered_key: bool, charset: Option<String>},
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
struct Configuration {
    running_mode: Modes,
}

impl Configuration {
    /// Create a new configuration instance with given mode.
    pub fn new(mode: Modes)-> Self {
        Configuration{
            running_mode: mode
        }
    }
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
        // I use unwrap() liberally in this function because parse_arguments() enforces which
        // arguments are required, so I'm sure they are there when I unwrap them.
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
                    charset: if _matches.is_present("charset") {
                        Some(String::from(_matches.value_of("charset").unwrap()))
                    } else {
                        None
                    }
                }
            }
        } else {
            // TODO: An error is thrown if you call cifra with no arguments, when help should be displayed instead.
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
                    output_recovered_key: _matches.is_present("output_recovered_key"),
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
fn file_exists(path: &str)-> std::result::Result<(), String>{
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
fn parse_arguments(arg_vec: &Vec<&str>) -> Configuration {
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
            .arg(Arg::new("output_recovered_key")
                .short('k')
                .long("output_recovered_key")
                .about("Include guessed key in output. If not used only recovered text is output."))
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

/// Helper generic function to process files to cipher and decipher.
///
/// # Parameters:
/// * configuration: Cifra running configurations.
///
/// # Returns:
/// * Processed resulting string.
fn process_file_with_key(configuration: &Configuration)-> Result<String> {
    match &configuration.running_mode {
        Modes::Cipher { algorithm, key, file_to_cipher,
            ciphered_file, charset } => {
                let input_file_path = file_to_cipher;
                let content_to_process = read_to_string(input_file_path)
                    .chain_err(|| ErrorKind::IOError(String::from(input_file_path.to_str().unwrap())))?;
                let processed_content: String;
                match algorithm {
                    CipheringAlgorithms::Caesar | CipheringAlgorithms::Affine=> {
                        let process_function: fn(&str, usize, &str)-> Result<String> = get_integer_key_and_charset_ciphering_function(algorithm)?;
                        let process_key = usize::from_str(key.as_str())
                            .chain_err(|| ErrorKind::ConversionError("key", "&String", "usize"))?;
                        if let Some(charset_string) = charset {
                            processed_content = process_function(&content_to_process, process_key, charset_string)
                                .chain_err(|| "Error ciphering text.")?;
                        } else {
                            processed_content = process_function(&content_to_process, process_key, DEFAULT_CHARSET)
                                .chain_err(|| "Error ciphering text.")?;
                        }
                    },
                    CipheringAlgorithms::Substitution | CipheringAlgorithms::Vigenere=> {
                        let process_function: fn(&str, &str, &str)-> Result<String> = get_string_key_and_charset_ciphering_function(algorithm)?;
                        if let Some(charset_string) = charset {
                            processed_content = process_function(&content_to_process, key, charset_string)
                                .chain_err(|| "Error ciphering text.")?;
                        } else {
                            processed_content = process_function(&content_to_process, key, SUBSTITUTION_DEFAULT_CHARSET)
                                .chain_err(|| "Error ciphering text.")?;
                        }
                    },
                    CipheringAlgorithms::Transposition=> {
                        let process_function: fn(&str, usize)-> String = get_integer_key_ciphering_function(algorithm)?;
                        let process_key = usize::from_str(key.as_str())
                            .chain_err(|| ErrorKind::ConversionError("key", "&String", "usize"))?;
                        processed_content = process_function(&content_to_process, process_key);
                    }
                }
                return Ok(processed_content)
        }
        Modes::Decipher { algorithm, key,
            file_to_decipher , deciphered_file, charset } => {
            let input_file_path = file_to_decipher;
            let content_to_process = read_to_string(input_file_path)
                .chain_err(|| ErrorKind::IOError(String::from(input_file_path.to_str().unwrap())))?;
            let processed_content: String;
            match algorithm {
                CipheringAlgorithms::Caesar | CipheringAlgorithms::Affine=> {
                    let process_function: fn(&str, usize, &str)-> Result<String> = get_integer_key_and_charset_deciphering_function(algorithm)?;
                    let process_key = usize::from_str(key.as_str())
                        .chain_err(|| ErrorKind::ConversionError("key", "&String", "usize"))?;
                    if let Some(charset_string) = charset {
                        processed_content = process_function(&content_to_process, process_key, charset_string)
                            .chain_err(|| "Error deciphering text.")?;
                    } else {
                        processed_content = process_function(&content_to_process, process_key, DEFAULT_CHARSET)
                            .chain_err(|| "Error deciphering text.")?;
                    }
                },
                CipheringAlgorithms::Substitution | CipheringAlgorithms::Vigenere=> {
                    let process_function: fn(&str, &str, &str)-> Result<String> = get_string_key_and_charset_deciphering_function(algorithm)?;
                    if let Some(charset_string) = charset {
                        processed_content = process_function(&content_to_process, key, charset_string)
                            .chain_err(|| "Error deciphering text.")?;
                    } else {
                        processed_content = process_function(&content_to_process, key, SUBSTITUTION_DEFAULT_CHARSET)
                            .chain_err(|| "Error deciphering text.")?;
                    }
                },
                CipheringAlgorithms::Transposition=> {
                    let process_function: fn(&str, usize)-> Result<String> = get_integer_key_deciphering_function(algorithm)?;
                    let process_key = usize::from_str(key.as_str())
                        .chain_err(|| ErrorKind::ConversionError("key", "&String", "usize"))?;
                    processed_content = process_function(&content_to_process, process_key)
                        .chain_err(|| "Error deciphering text.")?;
                }
            }
            return Ok(processed_content)
        }
        _ => bail!("Can only process here files to cipher or decipher, but asked an unsupported \
        operation instead.")
    }
}

/// Helper generic function to output resulting content.
///
/// # Parameters:
/// * result: String with resulting processed content. If an output file has been requested then
/// result is written to that file or to screen otherwise.
/// * key: Key used to get result.
/// * configuration: Cifra running configurations.
fn output_result<T>(result: T, recovered_key: Option<String>, configuration: &Configuration)-> Result<()>
where T: AsRef<str>{
    let output_file_option: &Option<PathBuf>;
    let mut output_guessed_key = false;
    match &configuration.running_mode {
        Modes::Cipher { algorithm, key, file_to_cipher,
            ciphered_file, charset } => {
            output_file_option = ciphered_file;
        },
        Modes::Decipher { algorithm, key, file_to_decipher,
            deciphered_file, charset }=> {
            output_file_option = deciphered_file;
        },
        Modes::Attack { algorithm, file_to_attack,
            deciphered_file, charset, output_recovered_key
        }=> {
            output_file_option = deciphered_file;
            output_guessed_key = *output_recovered_key;
        }
        _ => {
            bail!("Used mode is not compatible with file output, nor should use output_result().")
        }
    }
    let output_string = match output_guessed_key {
        true=> format!("{{\n  \"guessed_key\":\"{}\"\n  \"recovered_text\":\"{}\"\n}}",
                       recovered_key.unwrap(),
                       result.as_ref()),
        false=> format!("{}", result.as_ref())
    };
    return if let Some(output_file_path) = output_file_option {
        write(output_file_path, output_string.as_str());
        Ok(())
    } else {
        println!("{}", output_string.as_str());
        return Ok(())
    }
}

/// Apply crypto attack to file to get most likely plain text.
///
/// # Parameters:
/// * configuration: Cifra running configuration.
///
/// # Returns:
/// * Most likely original plain text and most likely key string.
fn attack_file(configuration: &Configuration)-> Result<(String, String)> {
    if let Modes::Attack { algorithm, file_to_attack,
        deciphered_file, output_recovered_key, charset
    } = &configuration.running_mode {
        let ciphered_content = read_to_string(file_to_attack)
            .expect("Error reading file to attack.");
        match algorithm {
            CipheringAlgorithms::Caesar | CipheringAlgorithms::Affine => {
                let attack_function: fn(&str, &str)-> Result<usize> = get_charset_attack_function(algorithm)
                    .chain_err(||"Error getting attack function.")?;
                let key = if let Some(charset_str) = charset {
                        attack_function(ciphered_content.as_str(), charset_str)?
                    } else {
                        attack_function(ciphered_content.as_str(), DEFAULT_CHARSET)?
                    };
                let deciphered_text = process_file_with_key(&Configuration::new(Modes::Decipher {
                    algorithm: algorithm.clone(),
                    key: usize::to_string(&key),
                    file_to_decipher: file_to_attack.clone(),
                    deciphered_file: deciphered_file.clone(),
                    charset: charset.clone()
                }));
                return Ok((deciphered_text?, key.to_string()))
            },
            CipheringAlgorithms::Substitution => {
                let attack_function: fn(&str, &str)-> Result<(String, f64)> = get_string_key_and_charset_attack_function(algorithm)
                    .chain_err(||"Error getting attack function.")?;
                let (key, _) = if let Some(charset_str) = charset {
                    attack_function(ciphered_content.as_str(), charset_str)?
                } else {
                    attack_function(ciphered_content.as_str(), DEFAULT_CHARSET)?
                };
                let deciphered_text = process_file_with_key(&Configuration::new(Modes::Decipher {
                    algorithm: algorithm.clone(),
                    key: key.clone(),
                    file_to_decipher: file_to_attack.clone(),
                    deciphered_file: deciphered_file.clone(),
                    charset: charset.clone()
                }));
                return Ok((deciphered_text?, key))
            },
            CipheringAlgorithms::Transposition => {
                let attack_function: fn(&str)-> Result<usize> = get_no_charset_attack_function(algorithm)
                    .chain_err(||"Error getting attack function.")?;
                let key = attack_function(ciphered_content.as_str())?;
                let deciphered_text = process_file_with_key(&Configuration::new(Modes::Decipher {
                    algorithm: algorithm.clone(),
                    key: usize::to_string(&key),
                    file_to_decipher: file_to_attack.clone(),
                    deciphered_file: deciphered_file.clone(),
                    charset: charset.clone()
                }));
                return Ok((deciphered_text?, key.to_string()))
            },
            CipheringAlgorithms::Vigenere => {
                let attack_function: fn(&str, &str, bool)-> Result<String> = get_testing_mode_attack_function(algorithm)
                    .chain_err(||"Error getting attack function.")?;
                let key= if let Some(charset_str) = charset {
                    attack_function(ciphered_content.as_str(), charset_str, false)?
                } else {
                    attack_function(ciphered_content.as_str(), DEFAULT_CHARSET, false)?
                };
                let deciphered_text = process_file_with_key(&Configuration::new(Modes::Decipher {
                    algorithm: algorithm.clone(),
                    key: key.clone(),
                    file_to_decipher: file_to_attack.clone(),
                    deciphered_file: deciphered_file.clone(),
                    charset: charset.clone()
                }));
                return Ok((deciphered_text?, key))
            },
        }
    } else {
        return bail!("You tried to use attack_file function with a configuration that is not for attack mode.")
    }
}


/// Get a pointer to ciphering function for given algorithm.
///
/// Use only with algorithms that use integer keys and charsets.
fn get_integer_key_and_charset_ciphering_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, usize, &str)-> Result<String>> {
    let function = match algorithm {
        CipheringAlgorithms::Caesar=> cifra::cipher::caesar::cipher,
        CipheringAlgorithms::Affine=> cifra::cipher::affine::cipher,
        _ => return bail!("Given algorithm does not use integer key and charset.")
    };
    Ok(function)
}

/// Get a pointer to ciphering function for given algorithm.
///
/// Use only with algorithms that use string keys and charsets.
fn get_string_key_and_charset_ciphering_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, &str, &str)-> Result<String>> {
    let function = match algorithm {
        CipheringAlgorithms::Substitution => cifra::cipher::substitution::cipher,
        CipheringAlgorithms::Vigenere=> cifra::cipher::vigenere::cipher,
        _ => return bail!("Given algorithm does not use string key and charset.")
    };
    Ok(function)
}

/// Get a pointer to ciphering function for given algorithm.
///
/// Use only with algorithms that use integer keys but not charsets.
fn get_integer_key_ciphering_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, usize)-> String> {
    let function = match algorithm {
        CipheringAlgorithms::Transposition => cifra::cipher::transposition::cipher,
        _ => return bail!("Given algorithm does not use integer key or includes a charset.")
    };
    Ok(function)
}

/// Get a pointer to deciphering function for given algorithm.
///
/// Use only with algorithms that use integer keys and charsets.
fn get_integer_key_and_charset_deciphering_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, usize, &str)-> Result<String>> {
    let function = match algorithm {
        CipheringAlgorithms::Caesar=> cifra::cipher::caesar::decipher,
        CipheringAlgorithms::Affine=> cifra::cipher::affine::decipher,
        _ => return bail!("Given algorithm does not use integer key and charset.")
    };
    Ok(function)
}

/// Get a pointer to deciphering function for given algorithm.
///
/// Use only with algorithms that use string keys and charsets.
fn get_string_key_and_charset_deciphering_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, &str, &str)-> Result<String>> {
    let function = match algorithm {
        CipheringAlgorithms::Substitution => cifra::cipher::substitution::decipher,
        CipheringAlgorithms::Vigenere=> cifra::cipher::vigenere::decipher,
        _ => return bail!("Given algorithm does not use string key and charset.")
    };
    Ok(function)
}

/// Get a pointer to deciphering function for given algorithm.
///
/// Use only with algorithms that use integer keys but not charsets.
fn get_integer_key_deciphering_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, usize)-> Result<String>> {
    let function = match algorithm {
        CipheringAlgorithms::Transposition => cifra::cipher::transposition::decipher,
        _ => return bail!("Given algorithm does not use integer key or includes a charset.")
    };
    Ok(function)
}

/// Get a pointer to attack function for given algorithm.
///
/// Use only with algorithms that use charsets.
fn get_charset_attack_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, &str)-> Result<usize>>{
    let function = match algorithm {
        CipheringAlgorithms::Caesar => cifra::attack::caesar::brute_force_mp,
        CipheringAlgorithms::Affine=> cifra::attack::affine::brute_force_mp,
        _ => return bail!("Given algorithm does not use charset.")
    };
    Ok(function)
}

/// Get a pointer to attack function for given algorithm.
///
/// Use only with algorithms that return a string key and a float tuple.
fn get_string_key_and_charset_attack_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, &str)-> Result<(String, f64)>>{
    let function = match algorithm {
        CipheringAlgorithms::Substitution=> cifra::attack::substitution::hack_substitution_mp,
        _ => return bail!("Given algorithm do use string key and charset")
    };
    Ok(function)
}

/// Get a pointer to attack function for given algorithm.
///
/// Use only with algorithms that don't use charsets.
fn get_no_charset_attack_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str)-> Result<usize>>{
    let function = match algorithm {
        CipheringAlgorithms::Transposition=> cifra::attack::transposition::brute_force_mp,
        _ => return bail!("Given algorithm do use charset.")
    };
    Ok(function)
}

/// Get a pointer to attack function for given algorithm.
///
/// Use only with algorithms that have charset and a testing mode.
fn get_testing_mode_attack_function(algorithm: &CipheringAlgorithms)-> Result<fn(&str, &str, bool)-> Result<String>>{
    let function = match algorithm {
        CipheringAlgorithms::Vigenere=> cifra::attack::vigenere::brute_force_mp,
        _ => return bail!("Given algorithm don't have testing mode.")
    };
    Ok(function)
}


fn _main(argv: Vec<&str>) {
    let configuration = parse_arguments(&argv);

    match configuration.running_mode {
        // Dictionary management.
        Modes::Dictionary(DictionaryActions::Create
                            {dictionary_name, initial_words_file})=> {
            let mut new_dictionary = Dictionary::new(dictionary_name, true)
                .expect("Error creating new dictionary.");
            if let Some(path) = initial_words_file{
                let pathname = path.to_str()
                    .expect("Error processing initial words file name.");
                new_dictionary.populate(pathname);
            }
        }
        Modes::Dictionary(DictionaryActions::Delete
                            { dictionary_name })=> {
            Dictionary::remove_dictionary(dictionary_name)
                .expect("Error removing dictionary.");
        }
        Modes::Dictionary(DictionaryActions::Update
                          { dictionary_name, words_file })=> {
            let mut dictionary = Dictionary::new(dictionary_name, false)
                .expect("Error opening dictionary.");
            let pathname = words_file.to_str()
                .expect("Error processing words file name.");
            dictionary.populate(pathname);
        }
        Modes::Dictionary(DictionaryActions::List)=> {
            let dictionaries = Dictionary::get_dictionaries_names()
                .expect("Error retrieving available dictionaries.");
            for dictionary in &dictionaries{
                println!("{}", dictionary);
            }
        }
        // Ciphering management.
        Modes::Cipher{ .. }
        | Modes::Decipher { .. }=> {
            let ciphered_content = process_file_with_key(&configuration)
                .expect("Error deciphering text.");
            output_result(&ciphered_content, None, &configuration)
                .expect("Error outputting recovered text.");
        }
        Modes::Attack{ .. }=> {
            if let Ok((recovered_content, key)) = attack_file(&configuration) {
                output_result(&recovered_content, Some(key), &configuration)
                    .expect("Error outputting recovered text.");
            } else {
                panic!("Error attacking ciphered text.");
            }
        }
    }
}

fn main() {
    // I make an indirection to run functional test feeding my own vector to _main().
    // Shame on Rust for not having default arguments!
    let args: Vec<String> = args().collect();
    let args_str: Vec<&str> = args.iter().map(|Str| Str.as_str()).collect();
    _main(args_str);
}

#[cfg(test)]
mod tests {
    extern crate cifra;

    use rstest::*;
    use std::fs::create_dir;
    use std::env;
    use super::*;

    use test_common::fs::tmp::{TestEnvironment, TestFile};
    use test_common::fs::ops::copy_files;
    use test_common::system::env::TemporalEnvironmentVariable;

    use cifra::attack::database;
    use cifra::cipher::substitution;

    const CAESAR_ORIGINAL_MESSAGE: &str = "This is my secret message.";
    const CAESAR_CIPHERED_MESSAGE_KEY_13: &str = "guv6Jv6Jz!J6rp5r7Jzr66ntrM";
    const CAESAR_TEST_KEY: usize = 13;
    const SUBSTITUTION_TEST_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const SUBSTITUTION_TEST_KEY: &'static str =     "lfwoayuisvkmnxpbdcrjtqeghz";
    const SUBSTITUTION_ORIGINAL_MESSAGE: &'static str  = "If a man is offered a fact which goes against his \
                                    instincts, he will scrutinize it closely, and unless \
                                    the evidence is overwhelming, he will refuse to believe \
                                    it. If, on the other hand, he is offered something which \
                                    affords a reason for acting in accordance to his \
                                    instincts, he will accept it even on the slightest \
                                    evidence. The origin of myths is explained in this way. \
                                    -Bertrand Russell";
    const SUBSTITUTION_CIPHERED_MESSAGE: &'static str = "Sy l nlx sr pyyacao l ylwj eiswi upar lulsxrj isr \
                                    sxrjsxwjr, ia esmm rwctjsxsza sj wmpramh, lxo txmarr \
                                    jia aqsoaxwa sr pqaceiamnsxu, ia esmm caytra \
                                    jp famsaqa sj. Sy, px jia pjiac ilxo, ia sr \
                                    pyyacao rpnajisxu eiswi lyypcor l calrpx ypc \
                                    lwjsxu sx lwwpcolxwa jp isr sxrjsxwjr, ia esmm \
                                    lwwabj sj aqax px jia rmsuijarj aqsoaxwa. Jia pcsusx \
                                    py nhjir sr agbmlsxao sx jisr elh. -Facjclxo Ctrramm";

    const LANGUAGES: [&'static str; 4] = ["english", "spanish", "french", "german"];

    /// Class with info to use a temporary dictionaries database.
    pub struct LoadedDictionaries {
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

    #[fixture]
    fn full_loaded_temp_dictionaries()-> LoadedDictionaries {
        LoadedDictionaries::new()
    }

    #[fixture]
    fn temp_dir()-> TestEnvironment {TestEnvironment::new()}

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
                file_to_attack: PathBuf::from(message_file.path().to_str().unwrap()),
                output_recovered_key: false
            }
        };
        let recovered_configuration = parse_arguments(&provided_args);
        assert_eq!(expected_configuration, recovered_configuration);
    }

    #[test]
    fn test_parser_attack_caesar_with_recovered_key() {
        let message_file = TestFile::new();
        let command = format!("cifra attack caesar {} --deciphered_file recovered_message.txt --output_recovered_key", message_file.path().to_str().unwrap());
        let provided_args: Vec<&str> = command.split_whitespace().collect();
        let expected_configuration = Configuration {
            running_mode: Modes::Attack {
                algorithm: CipheringAlgorithms::Caesar,
                charset: None,
                deciphered_file: Some(PathBuf::from("recovered_message.txt")),
                file_to_attack: PathBuf::from(message_file.path().to_str().unwrap()),
                output_recovered_key: true
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
                file_to_attack: PathBuf::from(message_file.path().to_str().unwrap()),
                output_recovered_key: false
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

    #[rstest]
    fn test_cipher_caesar(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries) {
        let message_file = TestFile::new();
        write(message_file.path(), CAESAR_ORIGINAL_MESSAGE);
        let output_file_name = temp_dir.path().join("ciphered_message.txt");
        let provided_args = format!("cifra cipher caesar {} {} --ciphered_file {}",
                                    CAESAR_TEST_KEY,
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap());
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(CAESAR_CIPHERED_MESSAGE_KEY_13, recovered_content)
        } else {
            assert!(false);
        }
    }

    #[rstest]
    fn test_decipher_caesar(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries) {
        let message_file = TestFile::new();
        write(message_file.path(), CAESAR_CIPHERED_MESSAGE_KEY_13);
        let output_file_name = temp_dir.path().join("deciphered_message.txt");
        let provided_args = format!("cifra decipher caesar {} {} --deciphered_file {}",
                                    CAESAR_TEST_KEY,
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap());
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(CAESAR_ORIGINAL_MESSAGE, recovered_content)
        } else {
            assert!(false);
        }
    }

    #[rstest]
    fn test_cipher_substitution(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries){
        let message_file = TestFile::new();
        write(message_file.path(), SUBSTITUTION_ORIGINAL_MESSAGE);
        let output_file_name = temp_dir.path().join("ciphered_message.txt");
        let provided_args = format!("cifra cipher substitution {} {} --ciphered_file {} --charset {}",
                                    SUBSTITUTION_TEST_KEY,
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap(),
                                    SUBSTITUTION_TEST_CHARSET);
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(SUBSTITUTION_CIPHERED_MESSAGE, recovered_content)
        } else {
            assert!(false);
        }
    }

    #[rstest]
    fn test_decipher_substitution(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries){
        let message_file = TestFile::new();
        write(message_file.path(), SUBSTITUTION_CIPHERED_MESSAGE);
        let output_file_name = temp_dir.path().join("deciphered_message.txt");
        let provided_args = format!("cifra decipher substitution {} {} --deciphered_file {} --charset {}",
                                    SUBSTITUTION_TEST_KEY,
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap(),
                                    SUBSTITUTION_TEST_CHARSET);
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(SUBSTITUTION_ORIGINAL_MESSAGE, recovered_content)
        } else {
            assert!(false);
        }
    }

    #[rstest]
    fn test_attack_caesar(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries){
        let message_file = TestFile::new();
        write(message_file.path(), CAESAR_CIPHERED_MESSAGE_KEY_13);
        let output_file_name = temp_dir.path().join("recovered_message.txt");
        let provided_args = format!("cifra attack caesar {} --deciphered_file {}",
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap());
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(CAESAR_ORIGINAL_MESSAGE, recovered_content)
        } else {
            assert!(false);
        }
    }

    #[rstest]
    fn test_attack_caesar_with_recovered_key(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries){
        let message_file = TestFile::new();
        write(message_file.path(), CAESAR_CIPHERED_MESSAGE_KEY_13);
        let output_file_name = temp_dir.path().join("recovered_message.txt");
        let provided_args = format!("cifra attack caesar {} --deciphered_file {} --output_recovered_key",
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap());
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(format!("{{\n  \"guessed_key\":\"{}\"\n  \"recovered_text\":\"{}\"\n}}",
                               CAESAR_TEST_KEY.to_string(),
                               CAESAR_ORIGINAL_MESSAGE), recovered_content)
        } else {
            assert!(false);
        }
    }

    #[rstest]
    fn test_attack_substitution(temp_dir: TestEnvironment, full_loaded_temp_dictionaries: LoadedDictionaries){
        // Prepare a ciphered text file to attack.
        let message_file = TestFile::new();
        let english_book = env::current_dir().unwrap()
            .join("resources/english_book_c1.txt");
        let original_message: String = read_to_string(english_book.as_path())
            .expect("Error reading english book.");
        let ciphered_text = substitution::cipher(original_message.as_str(),
                                                 SUBSTITUTION_TEST_KEY,
                                                 SUBSTITUTION_TEST_CHARSET).unwrap();
        write(message_file.path(), ciphered_text);

        // Perform test.
        let output_file_name = temp_dir.path().join("recovered_message.txt");
        let provided_args = format!("cifra attack substitution {} --deciphered_file {} --charset {}",
                                    message_file.path().to_str().unwrap(),
                                    output_file_name.to_str().unwrap(),
                                    SUBSTITUTION_TEST_CHARSET);
        let provided_args_vec: Vec<&str> = provided_args.split_whitespace().collect();
        _main(provided_args_vec);
        if let Ok(recovered_content) = read_to_string(&output_file_name){
            assert_eq!(original_message, recovered_content)
        } else {
            assert!(false);
        }
    }


}