use std::fs;
use std::io;
use std::path::Path;

pub fn make_dirs(parent_path: &String, directories: Vec<String>) {
    for d in directories {
        fs::create_dir_all(format!("{}/{}", parent_path, d)).unwrap();
        fs::File::create(format!("{}/{}/.gitkeep", parent_path, d)).unwrap();
    }
}

pub fn make_file(path: &String, content: &String) {
    fs::write(path, content).unwrap()
}

/// Copy files from source to destination recursively.
/// https://nick.groenen.me/notes/recursively-copy-files-in-rust/
#[allow(dead_code)]
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
