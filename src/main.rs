mod commands;
mod file;
mod parser;
mod models;
mod utils;

use crate::file::file_writer::write_to_file;
use crate::parser::toml_deserialize::parse_string;

fn main() {
    let store_path = file::dir::create_dir_and_file_if_not_exist();
    let store_content = file::file_reader::read_file(&store_path);
    let mut store_map = parse_string(store_content);
    println!("\n--> {:?}\n", store_map);
    let command = utils::startup::parse_cli_options();
    command.print_command();
    let result = command.execute(&mut store_map);

    if result.write_to_file {
        let new_store_content = parser::toml_serialize::create_store_content(store_map);
        if write_to_file(&store_path, new_store_content) {

        }
    }

    if result.success {
        std::process::exit(0);
    }
}