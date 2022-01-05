use std::fs;
use crate::utils::error::{ErrorCode, print_error_and_exit};

// https://stackoverflow.com/a/31193386

///
/// Reads contents of file as string
///
/// ## Arguments
/// - `filepath` - String representation of store file's path.
pub fn write_to_file(filepath :&String, content: String) -> bool {
    match fs::write(filepath, content) {
        // .expect("Unable to write file");
        Ok(_) => { return true; }
        Err(err) => {
            print_error_and_exit(format!("Unable to write to file: {}", err.to_string()),
                                 ErrorCode::UnableToWriteToFile)
        }
    }
    false
}
