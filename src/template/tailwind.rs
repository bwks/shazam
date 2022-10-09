pub const CONFIG: &str = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './{{ project }}/{{ output_dir }}/**/*.{html,js}'
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}"#;

pub const CSS: &str = r#"@tailwind base;
@tailwind components;
@tailwind utilities;
"#;
