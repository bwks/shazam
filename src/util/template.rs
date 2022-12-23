use std::fs;
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::{bail, Result};
use md5::{Digest, Md5};
use tera::{Context, Tera};

use crate::core::konst::TEMPLATES_DIR;
use crate::model::config::Config;
use crate::util::date_time;

pub fn init_env(current_dir: &String, project: &String) -> Result<Tera> {
    let mut env = Tera::new(&format!("{current_dir}/{project}/templates/**/*.jinja"))?;
    env.register_filter("human_date", date_time::human_date);
    Ok(env)
}

/// Render a template
pub fn render_template(
    env: &Tera,
    template_name: &str,
    kontext: &Context,
) -> Result<String, tera::Error> {
    env.render(template_name, kontext).map_err(|e| e)
}

pub fn template_hasher(tempalte: &String) -> String {
    let template_hash = Md5::digest(tempalte.as_bytes());
    format!("{:x}", template_hash)
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
