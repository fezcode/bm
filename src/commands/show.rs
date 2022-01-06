use std::collections::HashMap;
use std::path::PathBuf;
use path_absolutize::Absolutize;
use crate::utils::error;
use crate::utils::success::ExecutionResult;

pub fn show(params: &Vec<String>, store: &mut HashMap<String, String>) -> ExecutionResult {
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
                    error::print_error_and_exit("Given path does not exist. Consider using -a option.".into(),
                                                error::ErrorCode::AddCommandPathNotFound);
                } else {
                    addable = true;
                }
            }
            Some(opt) => match opt.as_str() {
                "-a" | "--add-anyway" => { addable = true; }
                _ => {
                    error::print_error_and_exit("Given option is not recognized".into(),
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

    return ExecutionResult{ success: false, write_to_file: false };

}