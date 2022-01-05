use std::collections::HashMap;
use toml;

use crate::models::bookmark::Bookmark;
use crate::utils::error::*;

pub fn create_store_content(store : HashMap<String, String>  ) -> String {
    let mut store_map : HashMap<String, Vec<Bookmark>> = HashMap::new();
    let mut bookmark_vec: Vec<Bookmark> = Vec::new();

    for val in store {
        let bm = Bookmark::new(val.0, val.1);
        bookmark_vec.push(bm);
    }

    store_map.insert("bookmark".to_string(), bookmark_vec);

    let toml = toml::to_string(&store_map);
    match toml {
        Ok(str) => { return str; }
        Err(err) => {
            print_error_and_exit(format!("Serialization error of store: {}", err.to_string()),
                                 ErrorCode::StoreFileSerializationError);
        }
    }
    return "".to_string();
}