use std::collections::HashMap;
use std::error::Error;
use toml;
use serde_derive::Deserialize;

use crate::models::bookmark::Bookmark;
use crate::utils::error::*;

pub fn parse_string(toml_string : String) -> HashMap<String, String> {

    // TODO fix this line and try following unwrap_or_else
    // toml::from_str(toml_string.as_str()).unwrap_or_else(|err| {
    //         eprintln!("Problem parsing arguments: {}", err);
    //         process::exit(1);
    //     });

    let parse_result: Result<HashMap<String, Vec<Bookmark>>, toml::de::Error> = toml::from_str(toml_string.as_str());
    let mut bm_map : HashMap<String, String> = Default::default();
    match parse_result {
        Ok(bookmarks_table) => {
            let bookmarks = &bookmarks_table["bookmark"];
            bm_map = bookmarks.into_iter().map(|bm| (bm.name_clone(), bm.dir_clone()) ).collect();
        },
        Err(e) => {
            print_error_and_exit(format!("Store file parsing error: {:?}", e.to_string()), ErrorCode::StoreFileParseError);
        }
    }
    return bm_map;

    // let bookmarks_table: HashMap<String, Vec<Bookmark>> = toml::from_str(toml_string.as_str()).expect("Store file cannot be parsed");

    // let bookmarks_parser: Result<HashMap, Error> = toml::from_str(toml_string.as_str());
    // let mut bookmarks_table;
    // match bookmarks_parser {
    //     Ok(a) => {
    //         bookmarks_table = &a["bookmark"];
    //         println!("{:?}", bookmarks_table);
    //     },
    //     Err(err) => {
    //         println!("Store file cannot be parsed! {}", err);
    //         std::process::exit(3);
    //     }
    // }
    // let bookmarks_table: HashMap<String, Vec<Bookmark>> = toml::from_str(toml_string.as_str()).expect("Store file cannot be parsed")
    // let bookmarks_table : HashMap<String, Vec<Bookmark>> = toml::from_str(toml_string.as_str())
}