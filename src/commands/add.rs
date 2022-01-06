use std::collections::HashMap;
use std::path::PathBuf;
use path_absolutize::Absolutize;
use crate::utils::error;
use crate::utils::success::ExecutionResult;

enum PathType {
    None,
    File,
    Directory
}

pub fn add(params: &Option<Vec<String>>, store: &mut HashMap<String, String>) -> ExecutionResult {
    let mut name : String;
    let mut path: String;
    let mut addanyway = false;
    let mut overwritable = false;
    let mut path_type = PathType::None;

    match params {
        None => {
            error::print_error_and_exit("Command cannot be verified with given args".into(),
                                        error::ErrorCode::CommandVerificationError);
        }
        Some(p) => {
            name = p[0].to_owned();
            path = p.get(1).unwrap_or(&std::env::current_dir().unwrap().to_string_lossy().to_string()).to_owned();
            let canon_path = PathBuf::from(path);

            // Options exist
            if p.len() > 2 {
                for opt in &p[2..] {
                    match opt.as_str() {
                        "-a" | "--add-anyway" => {
                            addanyway = true;
                        }
                        "-o" | "--overwrite" => {
                            overwritable = true;
                        }
                        "-d" | "--directory-only" => {
                            path_type = PathType::Directory;
                        }
                        "-f" | "--file-only" => {
                            path_type = PathType::File;
                        }
                        _ => {
                            error::print_error_and_exit("Given option is not recognized".into(),
                                                        error::ErrorCode::AddCommandOptionMatchFailed);
                        }
                    }
                }
            }

            if store.contains_key(name.as_str()) && !overwritable {
                error::print_error_and_exit(format!("Entry with name `{}` already exists. Specify -o option to overwrite it", name),
                                            error::ErrorCode::AddCommandNotOverwritable)
            }

            if !canon_path.exists() && !addanyway {
                error::print_error_and_exit("Given path does not exist. Specify -a option to add anyway.".into(),
                                            error::ErrorCode::AddCommandPathNotFound);
            }

            match path_type {
                PathType::None => {}
                PathType::File => {
                    if !canon_path.is_file() {
                        error::print_error_and_exit("Cannot add path to store since you specified that the path must be a file.".into(),
                                                    error::ErrorCode::AddCommandPathTypeError);
                    }
                }
                PathType::Directory => {
                    if !canon_path.is_dir() {
                        error::print_error_and_exit("Cannot add path to store since you specified that the path must be a directory.".into(),
                                                    error::ErrorCode::AddCommandPathTypeError);
                    }
                }
            }

            path = canon_path.absolutize().unwrap().display().to_string();
            store.insert(name, path);
            return ExecutionResult{ success: true, write_to_file: true };

        }
    }
    return ExecutionResult{ success: false, write_to_file: false };
}