use std::path::Path;

#[derive(Debug)]
enum BoolOrErrorKind {
    Value(bool),
    Kind(std::io::ErrorKind),
}

fn main() {
    let paths = [
        "file",
        "dir",

        "file-symlink-to-file",
        "dir-symlink-to-file",
        "file-symlink-to-dir",
        "dir-symlink-to-dir",
        "file-symlink-to-nonexistent",
        "dir-symlink-to-nonexistent",

        "file-symlink-to-file-symlink-to-file",
        "dir-symlink-to-file-symlink-to-file",
        "file-symlink-to-dir-symlink-to-file",
        "dir-symlink-to-dir-symlink-to-file",
        "file-symlink-to-file-symlink-to-dir",
        "dir-symlink-to-file-symlink-to-dir",
        "file-symlink-to-dir-symlink-to-dir",
        "dir-symlink-to-dir-symlink-to-dir",
        "file-symlink-to-file-symlink-to-nonexistent",
        "dir-symlink-to-file-symlink-to-nonexistent",
        "file-symlink-to-dir-symlink-to-nonexistent",
        "dir-symlink-to-dir-symlink-to-nonexistent",
    ];

    for path in paths {
        println!("{}: Path::is_dir() -> {}", path, Path::new(path).is_dir());
    }

    println!();

    for path in paths {
        let is_dir = match Path::new(path).metadata() {
            Ok(m) => format!("Ok({})", m.is_dir()),
            Err(e) => format!("Err({:?})", e.kind()),
        };
        println!("{}: Metadata::is_dir() -> {}", path, is_dir);
    }

    // TODO: Cover is_directory()
}
