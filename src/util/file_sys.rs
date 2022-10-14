use std::fs;
use std::io;
use std::path::Path;
use std::path::MAIN_SEPARATOR;

use anyhow::Result;

pub fn make_dirs(parent_path: &String, directories: Vec<String>) -> Result<()> {
    for d in directories {
        fs::create_dir_all(format!("{parent_path}{MAIN_SEPARATOR}{d}"))?;
        fs::File::create(format!(
            "{parent_path}{MAIN_SEPARATOR}{d}{MAIN_SEPARATOR}.gitkeep"
        ))?;
    }
    Ok(())
}

pub fn make_file(path: &String, content: &String) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
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
