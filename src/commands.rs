mod add;
mod show;
mod delete;
mod help;

use std::collections::HashMap;
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
        println!("[DBG|CMD:{:?} --> {:?}]", self.type_of, self.args);
    }

    /// Execute
    pub fn execute(&self, store: &mut HashMap<String, String>) -> ExecutionResult {
        let mut result : ExecutionResult = Default::default();
        let params = &self.args;
        match self.type_of {
            CommandType::NONE => {
                error::print_error_and_exit("Impossible command.".into(), error::ErrorCode::ImpossibleCommand);
            }
            CommandType::ADD => {
                result = crate::commands::add::add(params, store);
            }
            CommandType::SHOW => {
                result = crate::commands::show::show(params, store.to_owned());
            }
            CommandType::DELETE => {
                result = crate::commands::delete::delete(params, store);
            }
            CommandType::HELP => {
                crate::commands::help::print_help();
            }
        }

        result




        // FIXME put command in execution functions to get rid of following checks.
        // match &self.args {
        //     // Commands with no argument
        //     None => {
        //         match self.type_of {
        //             CommandType::HELP => {
        //                 crate::utils::startup::print_help();
        //             }
        //             _ => {
        //                 error::print_error_and_exit("Command cannot be verified with given args".into(),
        //                                             error::ErrorCode::CommandVerificationError);
        //             }
        //         }
        //         return ExecutionResult{ success: false, write_to_file: false };
        //     },
        //     Some(params) => {
        //         match &self.type_of {
        //             CommandType::NONE => {
        //                 error::print_error_and_exit("Impossible command.".into(), error::ErrorCode::ImpossibleCommand);
        //                 // return ExecutionResult{ success: false, write_to_file: false };
        //             } // Command NONE
        //             CommandType::ADD => {
        //                 // return add(params, store)
        //             } // Command ADD
        //             CommandType::SHOW => {
        //
        //             } // Command SHOW
        //             CommandType::DELETE => {} // Command DELETE
        //             CommandType::CONFIG => {} // Command CONFIG
        //             CommandType::HELP => {} // Command HELP
        //         } // match command types
        //     } // Commands
        // } // match
        // return ExecutionResult{ success: false, write_to_file: false };
    }
}