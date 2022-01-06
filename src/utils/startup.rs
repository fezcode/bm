use crate::commands::{Command, CommandType};
use crate::utils::error;
use crate::utils::error::ErrorCode;

/// Print help text and exit with non-zero code.
pub fn print_help() {
    let help = r#"
  NAME
      bm - Bookmark Manager

  SYNOPSIS
      bm [command] [options]

  DESCRIPTION
      This utility provides shell bookmarks that allows you to save paths and use them.
      Bookmarks are kept in file called `store file`. Store file is located/created in
      `~/.bm/store.toml` path. Store file is an TOML file.

  COMMANDS
      add                             Adds bookmark.
          add <name> [options]        Adds current directory to bookmarks with given name.
          add <name> <dir> [options]  Adds given directory to bookmarks with given name.

      show                            Show bookmark.
          show                        Show all bookmarks.
          show <name>                 Show bookmark with given name.

      delete
          delete <name>               Delete bookmark with given name

      config                          Read or edit configuration
          config set key=value[,k=v]  Sets config value.
          config get key              Prints config value.

       help                           Prints this help text.

  OPTIONS
      -o, --overwrite                 Overwrite previous value of bookmark.

      -d, --directory                 Add bookmark if it is directory only.
                                      If -f is provided last, -f will be used.

      -f, --file                      Add bookmark if it is file only.
                                      If -d is provided last, -d will be used.

      -p, --pretty                    Show bookmark(s) or config results as a table.

      -h, --help                      Prints help text.

  CONFIG
      CONFDIR         Config file directory. Default: ~/.bm/
      CONFNAME        Config file name. Default: conf.json (~/.bm/conf.json)

  RETURN CODES
      1               Unrecognized argument.
      2               No command argument is provided
      3               User's Home Directoty cannot be accessed.
      4               Store file parsing error
      5               Store file serialization error
      6               Unable to write to store file
      7               Command verification error for given options of command
      8               Impossible command error.
      9               Add Command: Given bookmark path is not found.
      10              Add Command: Given Option/Flag is not recognized.
      255             Help is printed
"#;
    print!("{}\n",help);
    std::process::exit(error::ErrorCode::HelpPrinted as i32);
}

/// Print usage text and exit with non-zero code.
fn print_usage(any_code : Option<ErrorCode>) {
    let help = r#"Usage: bm <asdc> [name] [directory]
  For detailed help, type `bm help`
    "#;
    print!("{}\n",help);
    match any_code {
        None => { std::process::exit(error::ErrorCode::UnrecognizedArgument as i32); }
        Some(code) => { std::process::exit(code as i32); }
    }
}

pub fn parse_cli_options() -> Command {
    let mut command: Command = Command::new(CommandType::NONE, None);
    let mut arguments : Vec<String> = std::env::args().collect();

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

    match std::env::args().nth(1).as_deref() {
        Some("h") | Some("help") | Some("-h") | Some("--help") => {
            command = Command::new(CommandType::HELP, args_options);    // args_option = None
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
        Some("c") | Some("config") => {
            command = Command::new(CommandType::CONFIG, args_options);
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
    command
}