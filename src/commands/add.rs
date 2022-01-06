use std::collections::HashMap;
use std::path::PathBuf;
use path_absolutize::Absolutize;
use crate::utils::error;
use crate::utils::success::ExecutionResult;

pub fn add(params: &Option<Vec<String>>, store: &mut HashMap<String, String>) -> ExecutionResult {
    let mut name : String;
    let mut path: String;
    let mut addanyway = false;
    let mut overwritable = false;

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

            path = canon_path.absolutize().unwrap().display().to_string();


            // println!("[0] -> {}", name);
            // println!("[1] -> {}", path);

            store.insert(name, path);
            return ExecutionResult{ success: true, write_to_file: true };

        }
    }

    return ExecutionResult{ success: false, write_to_file: false };

}

//
// pub fn add(params: &Vec<String>, store: &mut HashMap<String, String>) -> ExecutionResult {
//     let name : String = params[0].clone();
//     let mut directory : String = std::env::current_dir().unwrap().to_string_lossy().to_string();
//     let option = params.get(2);
//     let mut addable = false;
//
//     // Directory is given at least 2 params
//     if params.len() > 1 {
//         directory = params[1].clone();
//         let canon_dir = PathBuf::from(directory);
//         // println!("]]] {}", canon_dir.display().to_string());
//         match option {
//             // No option is given, don't add if path does not exist
//             None => {
//                 if !canon_dir.exists() {
//                     error::print_error_and_exit("Given path does not exist. Consider using -a option.".into(),
//                                                 error::ErrorCode::AddCommandPathNotFound);
//                 } else {
//                     addable = true;
//                 }
//             }
//             Some(opt) => match opt.as_str() {
//                 "-a" | "--add-anyway" => { addable = true; }
//                 _ => {
//                     error::print_error_and_exit("Given option is not recognized".into(),
//                                                 error::ErrorCode::AddCommandOptionMatchFailed);
//                 }
//             },
//         } // add command option
//         directory = canon_dir.absolutize().unwrap().display().to_string();
//     }
//
//     println!("[0] -> {}", name);
//     println!("[1] -> {}", directory);
//
//     if addable {
//         store.insert(name, directory);
//         return ExecutionResult{ success: true, write_to_file: true };
//     }
//
//     return ExecutionResult{ success: false, write_to_file: false };
//
// }