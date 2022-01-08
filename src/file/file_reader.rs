use std::fs;

// https://stackoverflow.com/a/31193386

///
/// Reads contents of file as string
///
/// ## Arguments
/// - `filepath` - String representation of store file's path.
pub fn read_file(filepath : &String) -> String {
    fs::read_to_string(filepath).expect("Unable to read file.")
}
