use crate::commands::{Command, CommandType};
use crate::utils::error;
use crate::utils::error::ErrorCode;

/// Print usage text and exit with non-zero code.
fn print_usage(any_code : Option<ErrorCode>) {
    let help = r#"Usage: bm [debug] <acsd> [name] [directory]
  For detailed help, type `bm help`
    "#;
    print!("{}\n",help);
    match any_code {
        None => { std::process::exit(error::ErrorCode::UnrecognizedArgument as i32); }
        Some(code) => { std::process::exit(code as i32); }
    }
}

pub fn parse_cli_options() -> (Command, bool) {
    let mut command: Command = Command::new(CommandType::NONE, None);
    let mut arguments : Vec<String> = std::env::args().collect();
    let mut command_index = 1;
    let mut debug_mode = false;

    if arguments.len() > 1 && arguments[1] == "debug" {
        println!("[DBG|Debug Mode]");
        arguments.remove(1);
        command_index = 2;
        debug_mode = true;
    }

    // No argument is given.
    // ./bm
    if arguments.len() == 1 {
        print_usage(Some(ErrorCode::NoArgumentProvided));
    }

    // At least one possible command is given.
    // ./bm add
    arguments.drain(0..2);      // 0 and 1
    let mut args_options: Option<Vec<String>> = None;

    // Create command arguments.
    // (./bm add) NAME [PATH]
    if arguments.len() > 0 {
        args_options = Some(arguments);
    }

    match std::env::args().nth(command_index).as_deref() {
        Some("h") | Some("help") | Some("-h") | Some("--help") => {
            command = Command::new(CommandType::HELP, None);    // args_option = None
        }
        Some("a") | Some("add") => {
                command = Command::new(CommandType::ADD, args_options);
        }
        Some("s") | Some("show") => {
            command = Command::new(CommandType::SHOW, args_options);
        }
        Some("d") | Some("delete") => {
            command = Command::new(CommandType::DELETE, args_options);
        }
        // Some(x) => {
        //     error::print_error_and_exit("No argument provided".to_string(),
        //                                 error::ErrorCode::NoArgumentProvided);
        // }
        _ => {
            eprintln!("Parameter not recognized.\n");
            print_usage(None);
        }
    }
    (command, debug_mode)
}