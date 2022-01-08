use std::fs;
use dirs;
use std::path::{PathBuf};
use crate::utils::error;

pub fn create_dir_and_file_if_not_exist() -> String {
    match dirs::home_dir() {
        Some(path) => {
            // println!("Home Directory: {}", path.display());
            let mut dir_path_buf = PathBuf::new();
            dir_path_buf.push(path);
            dir_path_buf.push(".bm");

            let mut store_path_buf = dir_path_buf.clone();
            store_path_buf.push("store.toml");

            let dir_path= dir_path_buf.as_path();
            let store_path = store_path_buf.as_path();

            // println!("Dir Path: {}, Store Path: {} ", dir_path.display(), store_path.display());

            // No ~/.bm
            if !dir_path.exists() {
                fs::create_dir_all(dir_path).expect("Cannot create directory!");
            }

            // No ~/.bm/store.toml
            if !store_path.exists() {
                fs::File::create(store_path).expect("Store cannot be created");
            }

            // ~/.bm/store.toml exists
            // read_file(store_path.display().to_string());
            return store_path.display().to_string();
        },
        None => {
            error::print_error_and_exit(String::from("Home directory cannot be accessed."), error::ErrorCode::HomeDirAccess);
            String::from("")    // Empty line, does not really do anything, added for compiler error
        }
    }
}
