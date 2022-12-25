use std::env;
use std::fs;
use std::io;
use std::path::Path;

use anyhow::{bail, Result};

/// Get the full path to the current working directory
pub fn current_dir() -> Result<String> {
    let cwd = env::current_dir()?;
    match cwd.to_str() {
        None => bail!("unable to get current working directory"),
        Some(s) => Ok(s.to_owned()),
    }
}

/// Create the child directories for a parent path
pub fn make_dirs(parent_path: &String, directories: Vec<String>) -> Result<()> {
    for d in directories {
        let dir_path = format!("{parent_path}/{d}");
        let file_path = format!("{parent_path}/{d}/.gitkeep");
        fs::create_dir_all(Path::new(dir_path.as_str()))?;
        fs::File::create(Path::new(file_path.as_str()))?;
    }
    Ok(())
}

/// Create a file with the supplient content
pub fn make_file(path: &String, content: &String) -> Result<()> {
    let file_path = Path::new(path);
    fs::write(file_path, content)?;
    Ok(())
}

/// Copy files from source to destination recursively.
/// https://nick.groenen.me/notes/recursively-copy-files-in-rust/
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
