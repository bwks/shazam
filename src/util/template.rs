use std::collections::HashMap;
use std::fs;

use anyhow::{bail, Result};
use md5::{Digest, Md5};
use tera::{Context, Tera};
use tracing::{event, Level};

use crate::core::konst::TEMPLATES_DIR;
use crate::model::config::Config;
use crate::util::date_time;
use crate::util::file_sys::make_file;

/// Initialize a Tera environment
pub fn init_env(current_dir: &String, project: &String) -> Result<Tera> {
    let mut env = Tera::new(&format!("{current_dir}/{project}/templates/**/*.jinja"))?;
    env.register_filter("human_date", date_time::human_date);
    Ok(env)
}

/// Render a template
// TODO: Seems like a clippy bug here for map_err
// https://github.com/rust-lang/rust-clippy/issues/6460
#[allow(clippy::map_identity)]
pub fn render_template(
    env: &Tera,
    template_name: &str,
    kontext: &Context,
) -> Result<String, tera::Error> {
    env.render(template_name, kontext).map_err(|e| e)
}

/// Get the MD5 hash for a template
pub fn template_hasher(tempalte: &String) -> String {
    let template_hash = Md5::digest(tempalte.as_bytes());
    format!("{:x}", template_hash)
}

/// Builds a template and writes it to disk if the template has changed.
pub fn template_builder(
    template_hashes: &mut HashMap<String, String>,
    template_name: &String,
    template: &String,
    file_path: &String,
) -> Result<()> {
    template_hashes
        .entry(template_name.to_owned())
        .or_insert_with(|| "".to_owned());

    let current_hash = template_hashes[template_name].to_owned();
    let this_hash = template_hasher(template);

    if current_hash != this_hash {
        event!(target: "shazam", Level::INFO, "File: `{template_name}` has changed, rebuilding...");
        template_hashes.insert(template_name.to_owned(), this_hash);
        make_file(file_path, template)?;
    }
    Ok(())
}

/// Load all project templaes into environment
/// from the layouts and includes directories
#[allow(dead_code)]
pub fn load_templates(env: &mut Tera, config: &Config) -> Result<()> {
    let project_name = config.project.to_owned();
    let template_dirs = config.template_dirs.to_owned();

    for dir in &template_dirs {
        for entry in fs::read_dir(format!("{project_name}/{TEMPLATES_DIR}/{dir}/"))? {
            let file = entry?.file_name().into_string();
            match file {
                Ok(file_name) => {
                    if file_name.ends_with(".jinja") | file_name.ends_with(".j2") {
                        env.add_template_file(format!("{dir}/{file_name}"), None)?;
                    }
                }
                Err(_) => bail!("error loading templates"),
            }
        }
    }
    Ok(())
}
