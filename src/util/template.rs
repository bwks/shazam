use std::fs;

use anyhow::{bail, Result};
use minijinja::value::Value;
use minijinja::{Environment, Source};

use crate::core::konst::TEMPLATES_DIR;
use crate::model::config::Config;
use crate::util::date_time;
use crate::util::text;

/// Initialize template environment
pub fn init_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_filter("capitalize", text::capitalize);
    env.add_filter("parameterize", text::parameterize);
    env.add_filter("title_case", text::title_case);
    env.add_filter("human_date", date_time::human_date);
    env
}

/// Render a template
pub fn render_template(env: &Environment, template: &str, kontext: Value) -> Result<String> {
    let tmpl = env.get_template(template)?;
    let r = tmpl.render(kontext)?;
    Ok(r)
}

/// Load all project templaes into environment
/// from the layouts and includes directories
pub fn load_templates(env: &mut Environment, source: &mut Source, config: &Config) -> Result<()> {
    let project_name = config.project.to_owned();
    let template_dirs = config.template_dirs.to_owned();

    let mut all_templates: Vec<String> = vec![];
    for dir in &template_dirs {
        for entry in fs::read_dir(format!("{project_name}/{TEMPLATES_DIR}/{dir}/"))? {
            let file = entry?.file_name().into_string();
            match file {
                Ok(file) => {
                    if file.ends_with(".jinja") || file.ends_with(".j2") {
                        all_templates.push(format!("{dir}/{file}"))
                    }
                }
                Err(_) => bail!("error loading templates"),
            }
        }
    }
    for template in &all_templates {
        let template_string =
            fs::read_to_string(format!("{project_name}/{TEMPLATES_DIR}/{template}"))?;
        source.add_template(template, template_string)?;
        env.set_source(source.to_owned());
    }
    Ok(())
}
