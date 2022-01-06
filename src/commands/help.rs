use std::collections::HashMap;
use std::fmt::format;
use std::path::PathBuf;
use path_absolutize::Absolutize;
use crate::utils::error;
use crate::utils::success::ExecutionResult;

/// bm delete NAME
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
          add <name> <path> [options] Adds given directory to bookmarks with given name.

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

      -a, --add-anyway                Add path even if it does not exist.

      -d, --directory-only            Add bookmark if it is directory only.
                                      If -f is provided last, -f will be used.

      -f, --file-only                 Add bookmark if it is file only.
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