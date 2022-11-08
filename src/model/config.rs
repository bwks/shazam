use crate::string_vec;

use crate::core::konst::{
    BLOG_DIR, CONFIG_DIR, CSS_DIR, CSS_FRAMEWORK, DATA_DIR, ERROR_DIR, FAVICON_DIR, FONT_DIR,
    IMG_DIR, INCLUDES_DIR, JS_DIR, LAYOUTS_DIR, MACROS_DIR, OUTPUT_DIR, SYNTAX_HIGHLIGHTER,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: String,
    pub owner: String,
    pub owner_email: String,
    pub css: String,
    pub code_block: String,
    pub config_dir: String,
    pub data_dir: String,
    pub output_dir: String,
    pub asset_dirs: Vec<String>,
    pub template_dirs: Vec<String>,
    pub content_dirs: Vec<String>,
}
impl Config {
    pub fn init(project: String, owner: String, owner_email: String) -> Self {
        Self {
            project: project,
            owner: owner,
            owner_email: owner_email,
            css: CSS_FRAMEWORK.to_owned(),
            code_block: SYNTAX_HIGHLIGHTER.to_owned(),
            config_dir: CONFIG_DIR.to_owned(),
            data_dir: DATA_DIR.to_owned(),
            output_dir: OUTPUT_DIR.to_owned(),
            asset_dirs: string_vec![CSS_DIR, JS_DIR, FONT_DIR, IMG_DIR, FAVICON_DIR, ERROR_DIR],
            template_dirs: string_vec![LAYOUTS_DIR, INCLUDES_DIR, MACROS_DIR],
            content_dirs: string_vec![BLOG_DIR],
        }
    }
}
