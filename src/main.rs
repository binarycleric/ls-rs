use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

struct DisplayFile {
    name: std::path::PathBuf, 
    permissions: std::fs::Permissions, 
}

fn display_file_information(file: DisplayFile) {
    println!("{perms} {path}", 
             perms=file.permissions.mode(), 
             path=file.name.display());
}

fn build_display_file(file: fs::DirEntry) -> DisplayFile {
    let meta = fs::metadata(file.path()).unwrap();
    let path = file.path();

    DisplayFile { 
        name: path,
        permissions: meta.permissions(),
    }
}

fn list_files(path: &Path) {
    let mut displayable = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        displayable.push(build_display_file(e));
                    },
                    Err(_) => {

                    }
                }
            }
        },
        Err(_) => { 
         
        },
    }

    for file in displayable {
        display_file_information(file);
    }
}

fn main() {
    match env::args().nth(1) {
        Some(path) => list_files(Path::new(&path)),
        None => list_files(Path::new("./")),
    }
}
