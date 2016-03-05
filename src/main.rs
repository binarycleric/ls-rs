use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn display_file_information(file: fs::DirEntry) {
    let meta = fs::metadata(file.path()).unwrap();
    let path = file.path();
    let mode = meta.permissions().mode();

    println!("{mode} {path}", path=path.display(), mode=mode);
}

fn list_files(path: &Path) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        display_file_information(e);
                    },
                    Err(_) => {

                    }
                }
            }
        },
        Err(_) => { 
         
        },
    }
}

fn main() {
    match env::args().nth(1) {
        Some(path) => list_files(Path::new(&path)),
        None => list_files(Path::new("./")),
    }
}
