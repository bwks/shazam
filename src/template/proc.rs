pub const PROCFILE: &str = r#"web: cargo run -- build && cargo run -- serve
css: ./tailwindcss -i {{ project }}/assets/css/input.css -o {{ project }}/{{ output_dir }}/css/app.css --watch
"#;
