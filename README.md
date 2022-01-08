# BM - Bookmark Manager
## Your Shell Bookmark Manager.

Creates `~/.bm` directory to store `store.toml` file which contains your bookmarks in
`toml` format.

## Commands

```bash
bm a/add <name>                      : Adds current directory to bookmarks.
bm a/add <name> <directory> [option] : Adds given directory to bookmarks.
bm s/show                            : Show all bookmarks.
bm s/show <name>                     : Show bookmark associated with given name.
bm d/delete <name>                   : Delete bookmark with given name.
bm h/help                            : Prints help text.
bm -h/--help                         : Prints help text.
bm debug <any full command>          : For any command, activates debug mode, if any debug prints are available.
```

### Add Options
```
   -a, --add-anyway
       Adds path to bookmarks even if it does not exist
   
   -o, --overwrite
       Overwrites bookmark with new given value if a bookmark with
       same name exists.
       
   -d, --directory-only
       Add bookmark if it is directory only. If -f is provided last, -f will be used.
       
   -f, --file-only 
       Add bookmark if it is file only. If -d is provided last, -d will be used.
       
```

## Create Debian Package
[Source](https://blog.karmacomputing.co.uk/how-to-create-deb-package-from-rust/)
1. First install cargo-deb package 
```bash
   cargo install cargo-deb 
```

2. Add metadata to Cargo.toml
```toml

[package.metadata.deb]
maintainer = "Fezcode <samil.bulbul@gmail.com>"
copyright = "Copyright (c) 2022 fezcode <samil.bulbul@gmail.com>"

```

3. Run `cargo deb`
```
cargo deb
```