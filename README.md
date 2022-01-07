# BM - Bookmark Manager

## Your Shell Bookmark Manager.

```shell
bm a/add <name>                      : Adds current directory to bookmarks.
bm a/add <name> <directory> [option] : Adds given directory to bookmarks.
bm s/show                            : Show all bookmarks.
bm s/show <name>                     : Show bookmark associated with given name.
bm d/delete <name>                   : Delete bookmark with given name.
bm h/help                            : Prints help text.
bm --help                            : Prints help text.
bm debug <any full command>          : For any command, activates debug mode.
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

.into()
.to_string()
.to_owned()