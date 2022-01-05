mod add;

use std::collections::HashMap;
use std::path::PathBuf;
use path_absolutize::*;
use crate::utils::error;
use crate::utils::success::ExecutionResult;

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
    ///
    pub fn execute(&self, store: &mut HashMap<String, String>) -> ExecutionResult {
        match &self.args {
            // No argument commands
            None => {
                match self.type_of {
                    CommandType::HELP => {
                        crate::utils::startup::print_help();
                    }
                    _ => {
                        error::print_error_and_exit("Command cannot be verified with given args".to_string(),
                                                    error::ErrorCode::CommandVerificationError);
                    }
                }
                return ExecutionResult{ success: false, write_to_file: false };
            },
            Some(params) => {
                match &self.type_of {
                    CommandType::NONE => {
                        error::print_error_and_exit("Impossible command.".to_string(), error::ErrorCode::ImpossibleCommand);
                        // return ExecutionResult{ success: false, write_to_file: false };
                    } // Command NONE
                    CommandType::ADD => {
                        let name : String = params[0].clone();
                        let mut directory : String = std::env::current_dir().unwrap().to_string_lossy().to_string();
                        let option = params.get(2);
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
                        }

                        println!("[0] -> {}", name);
                        println!("[1] -> {}", directory);

                        if addable {
                            store.insert(name, directory);
                            return ExecutionResult{ success: true, write_to_file: true };
                        }

                    } // Command ADD
                    CommandType::SHOW => {

                    } // Command SHOW
                    CommandType::DELETE => {} // Command DELETE
                    CommandType::CONFIG => {} // Command CONFIG
                    CommandType::HELP => {} // Command HELP
                } // match command types
            } // Commands
        } // match
        return ExecutionResult{ success: false, write_to_file: false };
    }
}