use std::env;
use std::path::Path;
use std::fs;

fn list_files(path: &Path) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        println!("Name: {:?}", entry.unwrap().path())
    }
}

fn main() {
    match env::args().nth(1) {
        Some(path) => list_files(Path::new(&path)),
        None => list_files(Path::new("./")),
    }
}
