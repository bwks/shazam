use anyhow::Result;
use minijinja::value::Value;
use minijinja::Environment;

pub fn render_template(env: &Environment, template: &str, kontext: Value) -> Result<String> {
    let tmpl = env.get_template(template)?;
    let r = tmpl.render(kontext)?;
    Ok(r)
}
