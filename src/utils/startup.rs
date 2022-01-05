use crate::commands::{Command, CommandType};
use crate::commands::CommandType::NONE;
use crate::utils::error;

fn print_help() {
    let help = r#"
  NAME
      bm - Bookmark Manager

  SYNOPSIS
      bm [command] [options]

  DESCRIPTION
      This utility provides shell bookmarks that allows you to save paths and use them.

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
      1               Argument error
      2               Home directory accessing error
      3               Store file toml format parsing error
"#;
    print!("{}\n",help);
}

fn print_usage() {
    let help = r#"  Usage: bm <asdc> [name] [directory]
        For detailed help, type `bm help`
    "#;
    print!("{}\n",help);
}

pub fn parse_options() -> Command {
    let mut command: Command = Command::new(CommandType::NONE, None);
    let mut arguments : Vec<String> = std::env::args().collect();
    arguments.drain(0..2);
    let mut args_options: Option<Vec<String>> = None;
    if arguments.len() > 0 {
        args_options = Some(arguments);
    }

    match std::env::args().nth(1).as_deref() {
        Some("h") | Some("help") | Some("-h") | Some("--help") => {
            command = Command::new(CommandType::HELP, None);
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
        Some(x) => {
            error::print_error_and_exit(format!("unrecognized argument: {}\n", x),
                                        error::ErrorCode::UnrecognizedArgument);
        }
        _ => {
            print_usage();
            error::print_error_and_exit(String::from(""),
                                        error::ErrorCode::UnrecognizedArgument);
        }
    }
    command
}