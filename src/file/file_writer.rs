use std::fs;
use crate::utils::error::{ErrorCode, print_error_and_exit};

///
/// Reads contents of file as string
///
/// ## Arguments
/// - `filepath` - String representation of store file's path.
///
/// [External Link - Stackoverflow](https://stackoverflow.com/a/31193386)
///
pub fn write_to_file(filepath :&String, content: String) -> bool {
    match fs::write(filepath, content) {
        // .expect("Unable to write file");
        Ok(_) => true,
        Err(err) => {
            print_error_and_exit(format!("Unable to write to file: {}", err.to_string()),
                                 ErrorCode::UnableToWriteToFile);
            false
        }
    }
}
