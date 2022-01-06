use std::collections::HashMap;
use std::fmt::format;
use std::path::PathBuf;
use path_absolutize::Absolutize;
use crate::utils::error;
use crate::utils::success::ExecutionResult;

/// bm delete NAME
pub fn delete(params: &Option<Vec<String>>, store: &mut HashMap<String, String>) -> ExecutionResult {
    let mut name : String;
    match params {
        // Show all
        None => {
            error::print_error_and_exit("No name is given to delete".into(),
                                        error::ErrorCode::DeleteCommandNameNotGiven);
        }
        Some(p) => {
            name = p[0].to_owned();
            if store.contains_key(name.as_str()) {
                store.remove(name.as_str());
                return ExecutionResult{ success: true, write_to_file: true };
            } else {
                error::print_error_and_exit(format!("Given key `{}` does not exist.", name),
                                            error::ErrorCode::DeleteCommandNameNotFound)
            }
        }
    }
    return ExecutionResult{ success: false, write_to_file: false };
}