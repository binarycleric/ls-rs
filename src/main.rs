use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

struct DisplayFile {
    name: std::path::PathBuf,
    metadata: std::fs::Metadata, 
}

impl DisplayFile {

    fn from_dir_entry(file: fs::DirEntry) -> DisplayFile {
        let meta = fs::metadata(file.path()).unwrap();
        let path = file.path();

        DisplayFile { 
            name: path,
            metadata: meta, 
        }
    }

    pub fn display(&self) -> String {
        let mode = self.permissions().mode();
        let length = self.metadata.len();

        return format!("{mode:o} - {length} bytes :: {path}", 
                       mode=mode, 
                       path=self.display_name(), 
                       length=length);
    }

    pub fn is_hidden(&self) -> bool {
        return self.name.to_str().unwrap().starts_with("./.");
    }

    fn permissions(&self) -> std::fs::Permissions {
        return self.metadata.permissions()
    }

    fn display_name(&self) -> String {
        match self.name.to_str() {
            Some(name) => {
                return name.replace("./", "") 
            },
            None => {
                return String::from("")
            }
        }
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
        if !display.is_hidden() {
            println!("{}", display.display());
        }
    }
}

fn main() {
    match env::args().nth(1) {
        Some(path) => list_files(Path::new(&path)),
        None => list_files(Path::new("./")),
    }
}
