use minijinja::value::Value;
use minijinja::Environment;

pub fn render_template(env: &Environment, template: &str, kontext: Value) -> String {
    let tmpl = env.get_template(template).unwrap();
    tmpl.render(kontext).unwrap()
}
