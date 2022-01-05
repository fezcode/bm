use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    name: String,
    dir: String,
    // hash: String
}

impl Bookmark {
    pub fn new(name : String, dir : String) -> Bookmark{
        return Bookmark{
            name,
            dir
        };
    }

    pub fn name_clone(&self) -> String {
        self.name.clone()
    }

    pub fn dir_clone(&self) -> String {
        self.dir.clone()
    }

}