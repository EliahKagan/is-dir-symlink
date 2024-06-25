# is-dir-symlink - Windows `Path::is_dir`/`Metadata::is_dir` experiment

Windows distinguishes file and directory symbolic links. Each symbolic link is a file or directory symbolic link, separately from its target.

Making a file symbolic link that dereferences to a directory (or to a directory symbolic link), or making a directory symbolic link that dereferences to a file (or to a file symbolic link), are not typically good practices. However, this occasionally occurs, either by accident or due to conditions where it cannot be avoided because the type of the target is not known when the symlink is created.

This examines what `Path::is_dir`, `std::fs::metadata`, and `Metadata::is_dir` (on the result of `std::fs::metadata`) in the Rust standard library return, for a number of combinations of symbolic link type and target type. Note that `Path::is_dir` will return `false` when dereferencing is unsuccessful, as well as when it is successful but the ultimate target is not a directory.

## License

[0BSD](LICENSE)

## How to run

Run `create.cmd` to create the test files. If you want to delete the test files, run `delete.cmd`. These are meant to be run from the top-level directory of this repository.

To run the experiment, run `cargo run`, also from the top-level directory of this repository.

If the experiment is run without `create.cmd` having been run first (or after `delete.cmd` is run, without rerunning `create.cmd` afterwards), then all the results will be `false`, because the files will not exist and `Path::is_dir` maps errors to `false`.

## Test data creation details

I got this output, as expected, from `./create.cmd`:

```text
        1 file(s) copied.
symbolic link created for file-symlink-to-file <<===>> file
symbolic link created for dir-symlink-to-file <<===>> file
symbolic link created for file-symlink-to-dir <<===>> dir
symbolic link created for dir-symlink-to-dir <<===>> dir
symbolic link created for file-symlink-to-nonexistent <<===>> nonexistent
symbolic link created for dir-symlink-to-nonexistent <<===>> nonexistent
symbolic link created for file-symlink-to-file-symlink-to-file <<===>> file-symlink-to-file
symbolic link created for dir-symlink-to-file-symlink-to-file <<===>> file-symlink-to-file
symbolic link created for file-symlink-to-dir-symlink-to-file <<===>> dir-symlink-to-file
symbolic link created for dir-symlink-to-dir-symlink-to-file <<===>> dir-symlink-to-file
symbolic link created for file-symlink-to-file-symlink-to-dir <<===>> file-symlink-to-dir
symbolic link created for dir-symlink-to-file-symlink-to-dir <<===>> file-symlink-to-dir
symbolic link created for file-symlink-to-dir-symlink-to-dir <<===>> dir-symlink-to-dir
symbolic link created for dir-symlink-to-dir-symlink-to-dir <<===>> dir-symlink-to-dir
symbolic link created for file-symlink-to-file-symlink-to-nonexistent <<===>> file-symlink-to-nonexistent
symbolic link created for dir-symlink-to-file-symlink-to-nonexistent <<===>> file-symlink-to-nonexistent
symbolic link created for file-symlink-to-dir-symlink-to-nonexistent <<===>> dir-symlink-to-nonexistent
symbolic link created for dir-symlink-to-dir-symlink-to-nonexistent <<===>> dir-symlink-to-nonexistent
 Volume in drive C is OS
 Volume Serial Number is B203-10FB

 Directory of C:\Users\ek\source\repos\is-dir-symlink

06/25/2024  12:24 AM    <DIR>          .
06/25/2024  12:24 AM    <DIR>          ..
06/24/2024  02:49 PM                62 .gitattributes
06/24/2024  12:16 PM               631 .gitignore
06/24/2024  11:47 AM               158 Cargo.lock
06/24/2024  01:52 PM               159 Cargo.toml
06/24/2024  12:13 PM             1,124 create.cmd
06/24/2024  12:15 PM               744 delete.cmd
06/25/2024  12:24 AM    <DIR>          dir
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-dir [dir]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-dir-symlink-to-dir [dir-symlink-to-dir]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-dir-symlink-to-file [dir-symlink-to-file]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-dir-symlink-to-nonexistent [dir-symlink-to-nonexistent]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-file [file]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-file-symlink-to-dir [file-symlink-to-dir]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-file-symlink-to-file [file-symlink-to-file]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-file-symlink-to-nonexistent [file-symlink-to-nonexistent]
06/25/2024  12:24 AM    <SYMLINKD>     dir-symlink-to-nonexistent [nonexistent]
06/25/2024  12:24 AM                 0 file
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-dir [dir]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-dir-symlink-to-dir [dir-symlink-to-dir]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-dir-symlink-to-file [dir-symlink-to-file]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-dir-symlink-to-nonexistent [dir-symlink-to-nonexistent]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-file [file]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-file-symlink-to-dir [file-symlink-to-dir]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-file-symlink-to-file [file-symlink-to-file]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-file-symlink-to-nonexistent [file-symlink-to-nonexistent]
06/25/2024  12:24 AM    <SYMLINK>      file-symlink-to-nonexistent [nonexistent]
06/24/2024  01:54 PM               664 LICENSE
06/24/2024  02:02 PM             6,506 README.md
06/24/2024  11:47 AM    <DIR>          src
06/24/2024  11:47 AM    <DIR>          target
              18 File(s)         10,048 bytes
              14 Dir(s)  94,519,246,848 bytes free
```

## Results of the experiment

Then I got these results from `cargo run`:

```text
file: Path::is_dir() -> false
dir: Path::is_dir() -> true
file-symlink-to-file: Path::is_dir() -> false
dir-symlink-to-file: Path::is_dir() -> false
file-symlink-to-dir: Path::is_dir() -> false
dir-symlink-to-dir: Path::is_dir() -> true
file-symlink-to-nonexistent: Path::is_dir() -> false
dir-symlink-to-nonexistent: Path::is_dir() -> false
file-symlink-to-file-symlink-to-file: Path::is_dir() -> false
dir-symlink-to-file-symlink-to-file: Path::is_dir() -> false
file-symlink-to-dir-symlink-to-file: Path::is_dir() -> false
dir-symlink-to-dir-symlink-to-file: Path::is_dir() -> false
file-symlink-to-file-symlink-to-dir: Path::is_dir() -> false
dir-symlink-to-file-symlink-to-dir: Path::is_dir() -> false
file-symlink-to-dir-symlink-to-dir: Path::is_dir() -> false
dir-symlink-to-dir-symlink-to-dir: Path::is_dir() -> true
file-symlink-to-file-symlink-to-nonexistent: Path::is_dir() -> false
dir-symlink-to-file-symlink-to-nonexistent: Path::is_dir() -> false
file-symlink-to-dir-symlink-to-nonexistent: Path::is_dir() -> false
dir-symlink-to-dir-symlink-to-nonexistent: Path::is_dir() -> false

file: Metadata::is_dir() -> Ok(false)
dir: Metadata::is_dir() -> Ok(true)
file-symlink-to-file: Metadata::is_dir() -> Ok(false)
dir-symlink-to-file: Metadata::is_dir() -> Err(NotADirectory)
file-symlink-to-dir: Metadata::is_dir() -> Err(PermissionDenied)
dir-symlink-to-dir: Metadata::is_dir() -> Ok(true)
file-symlink-to-nonexistent: Metadata::is_dir() -> Err(NotFound)
dir-symlink-to-nonexistent: Metadata::is_dir() -> Err(NotFound)
file-symlink-to-file-symlink-to-file: Metadata::is_dir() -> Ok(false)
dir-symlink-to-file-symlink-to-file: Metadata::is_dir() -> Err(NotADirectory)
file-symlink-to-dir-symlink-to-file: Metadata::is_dir() -> Err(PermissionDenied)
dir-symlink-to-dir-symlink-to-file: Metadata::is_dir() -> Err(NotADirectory)
file-symlink-to-file-symlink-to-dir: Metadata::is_dir() -> Err(PermissionDenied)
dir-symlink-to-file-symlink-to-dir: Metadata::is_dir() -> Err(NotADirectory)
file-symlink-to-dir-symlink-to-dir: Metadata::is_dir() -> Err(PermissionDenied)
dir-symlink-to-dir-symlink-to-dir: Metadata::is_dir() -> Ok(true)
file-symlink-to-file-symlink-to-nonexistent: Metadata::is_dir() -> Err(NotFound)
dir-symlink-to-file-symlink-to-nonexistent: Metadata::is_dir() -> Err(NotADirectory)
file-symlink-to-dir-symlink-to-nonexistent: Metadata::is_dir() -> Err(PermissionDenied)
dir-symlink-to-dir-symlink-to-nonexistent: Metadata::is_dir() -> Err(NotFound)
```
