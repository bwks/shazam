use std::error::Error;
use std::fs;
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::{bail, Result};
use tera::{Context, Tera};

use crate::core::konst::TEMPLATES_DIR;
use crate::model::config::Config;
use crate::util::{date_time, text};

/// Truncate a string to the desired length
// pub fn truncate(string: String, length: usize) -> String {
//     let mut str_vec: Vec<&str> = string.split_whitespace().collect();
//     str_vec.truncate(length);
//     str_vec.join(" ")
// }

pub fn init_env(current_dir: &String, project: &String) -> Result<Tera> {
    let mut env = Tera::new(&format!("{current_dir}/{project}/templates/**/*.jinja"))?;
    env.register_filter("human_date", date_time::human_date);
    env.register_filter("title_case", text::title_case);
    Ok(env)
}

/// Render a template
pub fn render_template(env: &Tera, template_name: &str, kontext: &Context) -> Result<String> {
    match env.render(template_name, kontext) {
        Ok(r) => Ok(r),
        Err(e) => {
            bail!(format!(
                " Failed to render: {}\n Error: {}",
                template_name,
                e.source().unwrap()
            ))
        }
    }
}

/// Load all project templaes into environment
/// from the layouts and includes directories
#[allow(dead_code)]
pub fn load_templates(env: &mut Tera, config: &Config) -> Result<()> {
    let project_name = config.project.to_owned();
    let template_dirs = config.template_dirs.to_owned();

    for dir in &template_dirs {
        for entry in fs::read_dir(format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{dir}{PATH_SEP}"
        ))? {
            let file = entry?.file_name().into_string();
            match file {
                Ok(file_name) => {
                    if file_name.ends_with(".jinja") | file_name.ends_with(".j2") {
                        env.add_template_file(format!("{dir}{PATH_SEP}{file_name}"), None)?;
                    }
                }
                Err(_) => bail!("error loading templates"),
            }
        }
    }
    Ok(())
}
