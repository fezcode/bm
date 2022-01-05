use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::utils::error;
use path_absolutize::*;

///
/// # Items
/// - [`CommandType::ADD`] - Add to bookmarks
/// - [`CommandType::SHOW`] - Show bookmark(s)
/// - [`CommandType::DELETE`] - Delete given bookmark
/// - [`CommandType::CONFIG`] - Set/Get config values
/// - [`CommandType::HELP`] - Display help
/// - [`CommandType::NONE`] - For initialization, do NOT use i
///
/// Use [`Command::new`] with one of the enum items listed above to create command.
///
/// # Examples
#[derive(Debug)]
pub enum CommandType {
    NONE,       // Default value. Should not be used besides initialization of command.
    ADD,
    SHOW,
    DELETE,
    CONFIG,
    HELP
}

pub struct Command {
    type_of : CommandType,
    args : Option<Vec<String>>
}

impl Command {
    pub fn new(type_of : CommandType, args: Option<Vec<String>>) -> Command {
        Command {
            type_of,
            args
        }
    }

    pub fn print_command(&self) {
        println!("{:?} --> {:?}", self.type_of, self.args);
    }

    /// Execute
    pub fn execute(&self, store: &mut HashMap<String, String>) -> bool {
        match &self.args {
            // No arguments
            None => {
                error::print_error_and_exit("Command cannot be verified with given args".to_string(), error::ErrorCode::VerificationError);
                return false;
            }, // No command
            Some(params) => {
                match &self.type_of {
                    CommandType::NONE => {
                        error::print_error_and_exit("Impossible command.".to_string(), error::ErrorCode::ImpossibleCommand);
                        return false;
                    } // NONE
                    CommandType::ADD => {
                        let mut name : String = params[0].clone();
                        let mut directory : String = std::env::current_dir().unwrap().to_string_lossy().to_string();
                        let mut option = params.get(2);
                        let mut addable = false;
                        // Directory is given at least 2 params
                        if params.len() > 1 {
                            directory = params[1].clone();
                            let canon_dir = PathBuf::from(directory);
                            // println!("]]] {}", canon_dir.display().to_string());
                            match option {
                                // No option is given, don't add if path does not exist
                                None => {
                                    if !canon_dir.exists() {
                                        error::print_error_and_exit("Given path does not exist. Consider using -a option.".to_string(),
                                                                    error::ErrorCode::AddCommandPathNotFound);
                                    } else {
                                        addable = true;
                                    }
                                }
                                Some(opt) => match opt.as_str() {
                                    "-a" | "--add-anyway" => { addable = true; }
                                    _ => {
                                        error::print_error_and_exit("Given option is not recognized".to_string(),
                                                                    error::ErrorCode::AddCommandOptionMatchFailed);
                                    }
                                },
                            } // add command option
                            directory = canon_dir.absolutize().unwrap().display().to_string();
                            // TODO check this part
                            // directory = fs::canonicalize(canon_dir).unwrap().as_path().display().to_string();
                            // directory = fs::canonicalize(canon_dir).unwrap().display().to_string();
                            // directory = directory[4..].to_string();
                        }

                        println!("[0] -> {}", name);
                        println!("[1] -> {}", directory);

                        // let canon_dir = PathBuf::from(directory);

                        if addable {
                            store.insert(name, directory);
                            return true;
                        }


                    } // ADD
                    CommandType::SHOW => {} // SHOW
                    CommandType::DELETE => {} // DELETE
                    CommandType::CONFIG => {} // CONFIG
                    CommandType::HELP => {} // HELP
                } // match command types
            } // Commands
        } // match
        false
    }
}