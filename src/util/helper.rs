use std::fs;
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::Result;

use crate::core::konst::{CONFIG_DIR, CONFIG_FILE};
use crate::model::config::Config;
use crate::model::post::Data;

#[allow(dead_code)]
pub fn load_config() -> Result<Config> {
    let config_file = fs::read_to_string(format!("{CONFIG_DIR}{PATH_SEP}{CONFIG_FILE}"))?;
    let config: Config = toml::from_str(config_file.as_str())?;
    Ok(config)
}

#[allow(dead_code)]
pub fn load_data_file(filename: String) -> Result<Data> {
    let data_file = fs::read_to_string(filename)?;
    let posts: Data = toml::from_str(data_file.as_str())?;
    Ok(posts)
}
