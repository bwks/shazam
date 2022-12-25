pub const PROCFILE: &str = r#"web: ./shazam build && ./shazam serve
css: ./tailwindcss -c {{ config_dir }}/{{ tailwind_config_file }} -i {{ project }}/assets/css/input.css -o {{ project }}/{{ output_dir }}/css/app.css --watch
build: ./reflex -r "\.jinja$" ./overmind restart web
"#;
pub const PROCFILE_DEV: &str = r#"web: cargo run -- build && cargo run -- serve
css: ./tailwindcss -c {{ config_dir }}/{{ tailwind_config_file }} -i {{ project }}/assets/css/input.css -o {{ project }}/{{ output_dir }}/css/app.css --watch
build: ./reflex -r "\.jinja$" ./overmind restart web
"#;
