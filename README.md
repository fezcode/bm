# BM - Bookmark Manager
## A Simple Bookmark Manager for Your Shell.

Creates `~/.bm` directory to store `store.toml` file which contains your bookmarks in
`toml` format.

## Commands
| Command                                | Description                                                               |
|:---------------------------------------|:--------------------------------------------------------------------------|
| `bm a/add <name>                     ` | Adds current directory to bookmarks.                                      |
| `bm a/add <name> <directory> [option]` | Adds given directory to bookmarks.                                        |
| `bm s/show                           ` | Show all bookmarks.                                                       |
| `bm s/show <name>                    ` | Show bookmark associated with given name.                                 |
| `bm d/delete <name>                  ` | Delete bookmark with given name.                                          |
| `bm h/help                           ` | Prints help text.                                                         |
| `bm -h/--help                        ` | Prints help text.                                                         |
| `bm debug <any full command>         ` | For any command, activates debug mode, if any debug prints are available. |

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

### Show Options
```
   -p, --pretty
       When priting all files, via `bm show`, giving this option prints output
       in table format.
```

## Examples

### 1. Add directory to bookmarks
```bash
# Add directory 
$ bm a HOME_DIR ~ 
# Add current directory
$ bm a HOME_DIR 
```

### 2. Change directory to `HOME_DIR`
**Linux**
```bash
$ cd `bm s HOME_DIR`
# or
$ cd $(bm s HOME_DIR)
```

**Windows**
```bash
$ cd $(.\bm s HOME_DIR)
# or
$ bm s HOME_DIR | cd
```

### 3. Show all bookmarks
```bash
$ bm s
# or pretty print
$ bm s -p
```

### 4. Delete bookmark
```bash
$ bm d HOME_DIR 
```

### 5. Add path to bookmarks if given path is path of a file
```bash
$ bm d FILE_PATH file.txt -f
```

### 6. Add path to bookmarks if given path is path of a directory
```bash
$ bm d DIR_PATH /path/to/dir -d
```

### 7. Print additional information via Debug command
```bash
$ bm debug add PATH /path
```

## Building Project
In order to build project, run

```bash
$ cargo build
```

To run project,

```bash
$ cargo run --package bm --bin bm
```

## Creating Debian Package
1. First install cargo-deb package
```bash
   cargo install cargo-deb 
```

2. Add metadata to Cargo.toml
```toml
[package.metadata.deb]
maintainer = "Username <username@example.com>"
copyright = "Copyright (c) 2022 Username <username@example.com>"
```

3. Run `cargo deb`
```
cargo deb
```

## Creating RPM Package

### Install

```cargo install cargo-generate-rpm```

### Usage

```rust
cargo build --release
strip -s target/release/XXX
cargo generate-rpm
```

### Configuration

``` toml
[package.metadata.generate-rpm]
    name: the package name. If not present, package.name is used.
    version: the package version. If not present, package.version is used.
    license: the package license. If not present, package.license is used.
    summary: the package summary/description. If not present, package.description is used.
    assets: (mandatory) the array of the files to be included in the package
        source: the location of that asset in the Rust project. (e.g. target/release/XXX) Wildcard character * is allowed.
        dest: the install-destination. (e.g. /usr/bin/XXX) If source contains wildcard character *, it must be a directory, not a file path.
        mode: the permissions as octal string. (e.g. 755 to indicate -rwxr-xr-x)
        config: set true if it is a configuration file.
        doc: set true if it is a document file.
    release: optional string of release.
    epoch: optional number of epoch.
    pre_install_script: optional string of pre_install_script.
    pre_uninstall_script: optional string of pre_uninstall_script.
    post_install_script: optional string of post_install_script.
    post_uninstall_script: optional string of post_uninstall_script.
    requires: optional list of Requires
    auto-req: optional string "no" to disable the automatic dependency process
    obsoletes: optional list of Obsoletes
    conflicts: optional list of Conflicts
    provides: optional list of Provides
```

### Location
```target/generate-rpm/XXX.rpm```

**_[Source](https://blog.karmacomputing.co.uk/how-to-create-deb-package-from-rust/)_**
