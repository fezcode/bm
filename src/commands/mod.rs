
pub enum Commands {
    NONE,
    ADD,
    SHOW,
    DELETE,
    CONFIG,
    HELP
}
pub mod add;

pub fn parse_options() -> Commands {
    let mut command = Commands::NONE;
    match std::env::args().nth(1).as_deref() {
        Some("h") | Some("help") | Some("-h") | Some("--help") => { command = Commands::HELP; print_help(); }
        Some("a") | Some("add") => { command = Commands::ADD; print!("a");}
        Some("s") | Some("show") => { command = Commands::SHOW; print!("s"); }
        Some("d") | Some("delete") => { command = Commands::DELETE; print!("d"); }
        Some("c") | Some("config") => { command = Commands::CONFIG; print!("c"); }
        Some(x) => { print!("unrecognized argument: {}\n", x); std::process::exit(1);}
        _ => { print_usage(); std::process::exit(1); }
    }
    return command;
}

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
"#;
    print!("{}\n",help);
}

fn print_usage() {
    let help = r#"  Usage: bm <asdc> [name] [directory]
        For detailed help, type `bm help`
    "#;
    print!("{}\n",help);
}