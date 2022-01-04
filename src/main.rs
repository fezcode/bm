mod commands;
mod file;
mod parser;
mod models;
mod utils;

use std::collections::HashMap;

use crate::parser::toml_parser::parse_string;

fn main() {

    // match std::env::args().nth(1).expect("Command").as_str() {
    // let a = String::from("a");

    let store_path = file::dir::create_dir_and_file_if_not_exist();
    let store_content = file::file_reader::read_file(store_path);
    let mut store_map = parse_string(store_content);
    println!("--> {:?}", store_map);
    let command = utils::startup::parse_options();
    command.print_command();
    command.execute(&mut store_map);
    println!("--> {:?}", store_map);



    // commands::parse_options();



    // let bookmark_map : HashMap<String, String> = bookmarks.into_iter().map(|bm| (bm.name.clone(), bm.value.clone()) ).collect();

    // let str = String::from(r#"
    //     [[bookmark]]
    //     name = "name1"
    //     dir = "value1"
    //
    //     [[bookmark]]
    //     name = "name2"
    //     dir = "value2"
    //
    //     [[bookmark]]
    //     name = "name3"
    //     dir = "value3"
    // "#);


    // commands::parse_options();
    // print!("{}", commands::add::AddCommand());

    // let command = std::env::args().nth(1).expect("no command given");
    // let option1 = std::env::args().nth(2).expect("no option given");
    // let option2 = std::env::args().nth(3).expect("no option given");

    // print!("{} - {} - {}", command, option1, option2);
    return;
}