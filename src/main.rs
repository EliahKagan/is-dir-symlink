use std::path::Path;

fn main() {
    for path in ["file-symlink", "dir-symlink"] {
        println!("{}: {}", path, Path::new(path).is_dir());
    }
}
