use std::path::Path;

fn main() {
    for path in [
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
    ] {
        println!("{}: {}", path, Path::new(path).is_dir());
    }
}
