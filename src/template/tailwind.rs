pub const CONFIG: &str = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '{{ project }}{{ path_sep }}{{ output_dir }}{{ path_sep }}**{{ path_sep }}*.{html,js}'
  ],
  safelist: [],
  theme: {
    extend: {},
  },
  plugins: [],
}"#;

pub const CSS: &str = r#"@tailwind base;
@tailwind components;
@tailwind utilities;
"#;
