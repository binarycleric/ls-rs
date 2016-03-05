use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

struct DisplayFile {
    name: std::path::PathBuf, 
    permissions: std::fs::Permissions, 
}

impl DisplayFile {
    fn from_dir_entry(file: fs::DirEntry) -> DisplayFile {
        let meta = fs::metadata(file.path()).unwrap();
        let path = file.path();

        DisplayFile { 
            name: path,
            permissions: meta.permissions(),
        }
    }

    // TODO: Make this return instead of just printing.
    fn display(&self) {
        let mode = self.permissions.mode();
        let path = self.name.display(); 

        println!("{mode} {path}", mode=mode, path=path);
    }
}

fn list_files(path: &Path) {
    let mut displayable = Vec::new();
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let display = DisplayFile::from_dir_entry(entry.unwrap());
        displayable.push(display);
    }

    for display in displayable {
        display.display();
    }
}

fn main() {
    match env::args().nth(1) {
        Some(path) => list_files(Path::new(&path)),
        None => list_files(Path::new("./")),
    }
}
