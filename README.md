# BM - Bookmark Manager

## Your Shell Bookmark Manager.

```shell
bm a/add <name>                      : Adds current directory to bookmarks.
bm a/add <name> <directory> [option] : Adds given directory to bookmarks.
bm s/show                            : Show all bookmarks.
bm s/show <name>                     : Show bookmark associated with given name.
bm d/delete <name>                   : Delete bookmark with given name.
bm c/config set key=value[,k=v]      : Edit config values
bm c/config get key[,key2]           : Get config values
bm h/help                            : Prints help text.
bm --help                            : Prints help text.
```

### Add Options
```
   -a, --add-anyway
       Adds path to bookmarks even if it does not exist
   
   -o, --overwrite
       Overwrites bookmark with new given value if a bookmark with
       same name exists.
```

.into()
.to_string()
.to_owned()