mod commands;
use crate::commands::Commands;
use crate::commands::add;
fn main() {

    // match std::env::args().nth(1).expect("Command").as_str() {
    // let a = String::from("a");

    commands::parse_options();
    print!("{}", commands::add::AddCommand());

    // let command = std::env::args().nth(1).expect("no command given");
    // let option1 = std::env::args().nth(2).expect("no option given");
    // let option2 = std::env::args().nth(3).expect("no option given");

    // print!("{} - {} - {}", command, option1, option2);
    return;
}