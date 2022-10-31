use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::{bail, Result};

pub fn current_dir() -> Result<String> {
    let cwd = env::current_dir()?;
    match cwd.to_str() {
        None => bail!("unable to get current working directory"),
        Some(s) => Ok(s.to_owned()),
    }
}

pub fn make_dirs(parent_path: &String, directories: Vec<String>) -> Result<()> {
    for d in directories {
        fs::create_dir_all(format!("{parent_path}{PATH_SEP}{d}"))?;
        fs::File::create(format!("{parent_path}{PATH_SEP}{d}{PATH_SEP}.gitkeep"))?;
    }
    Ok(())
}

pub fn make_file(path: &String, content: &String) -> Result<()> {
    fs::write(path, content)?;
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
