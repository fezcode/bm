use crate::utils::error;

/// bm help
/// Print help text and exit with non-zero code.
pub fn print_help() {
    let help = r#"
  NAME
      bm - Bookmark Manager

  VERSION
      {version}

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

      help                            Prints this help text.

      version                         Prints version info.

      debug                           Activates debug mode for other commands.
                                      Must be given as the first argument to application.
                                      Example: bm debug add name path --option

  OPTIONS
      -o, --overwrite                 Overwrite previous value of bookmark.

      -a, --add-anyway                Add path even if it does not exist.

      -d, --directory-only            Add bookmark if it is directory only.
                                      If -f is provided last, -f will be used.

      -f, --file-only                 Add bookmark if it is file only.
                                      If -d is provided last, -d will be used.

      -h, --help                      Prints help text.

  RETURN CODES
      0               Success.
      1               Unrecognized argument.
      2               No command argument is provided.
      3               User's Home Directoty cannot be accessed.
      4               Store file parsing error.
      5               Store file serialization error.
      6               Unable to write to store file.
      7               Command verification error for given options of command.
      8               Impossible command error.
      9               Add Command: Given bookmark path is not found.
      10              Add Command: Given Option/Flag is not recognized.
      11              Add Command: Given name already exists and overwrite option not given.
      12              Add Command: Given path is not given path type (file or directory).
      13              Show Command: Given name not found in store.
      14              Delete Command: No name is given.
      15              Delete Command: Given name not found in store.
      255             Help is printed.
"#;
    let formatted_version = help.replace("{version}", env!("CARGO_PKG_VERSION"));
    print!("{}\n",formatted_version);
    std::process::exit(error::ErrorCode::HelpPrinted as i32);
}

pub fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
    std::process::exit(error::ErrorCode::HelpPrinted as i32);
}