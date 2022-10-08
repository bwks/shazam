pub const BASE: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">

    <link rel="stylesheet" href="./style.css">
    <link rel="icon" href="./favicon.ico" type="image/x-icon">
    <script src="index.js"></script>
    <title>{{ project }}</title>

  </head>
  <body>
    <main>
      <h1>{{ project }} site</h1>
      {% block content %}
      {% endblock content %}
    </main>
  </body>
  {% include "includes/_footer.jinja" %}
</html>
"#;

pub const BLOG: &str = r#"{% extends "layouts/base.jinja" %}
{% block content %}
  <p>Blog Template</p>
{% endblock content %}
"#;

pub const FOOTER: &str = r#"<footer>
  <p>FOOTER</p>
</footer>
"#;
