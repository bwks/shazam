pub const PROCFILE: &str = r#"web: .{{ path_sep }}shazam build && .{{ path_sep }}shazam serve
css: .{{ path_sep }}tailwindcss -i {{ project }}{{ path_sep }}assets{{ path_sep }}css{{ path_sep }}input.css -o {{ project }}{{ path_sep }}{{ output_dir }}{{ path_sep }}css{{ path_sep }}app.css --watch
build: .{{ path_sep }}reflex -r "\.jinja$" .{{ path_sep }}overmind restart web
"#;
pub const PROCFILE_DEV: &str = r#"web: cargo run -- build && cargo run -- serve
css: .{{ path_sep }}tailwindcss -i {{ project }}{{ path_sep }}assets{{ path_sep }}css{{ path_sep }}input.css -o {{ project }}{{ path_sep }}{{ output_dir }}{{ path_sep }}css{{ path_sep }}app.css --watch
build: .{{ path_sep }}reflex -r "\.jinja$" .{{ path_sep }}overmind restart web
"#;
