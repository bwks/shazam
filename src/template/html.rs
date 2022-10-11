pub const BASE: &str = r#"<!DOCTYPE html>
<html lang="en" class="h-full">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">

    <link rel="stylesheet" href="/css/app.css">
    <link rel="icon" href="./favicon.ico" type="image/x-icon">
    <script src="/js/app.js"></script>
    <title>{{ project }}</title>
  </head>
  <body class="h-full antiailiased container mx-auto">
    <main>
      <h1 class="text-2xl font-black">{{ project }} site</h1>
      {% block content %}
      {% endblock content %}
    </main>
  </body>
  {% include "includes/footer.jinja" %}
</html>
"#;

pub const BLOG: &str = r#"{% extends "layouts/base.jinja" %}
{% block content %}
  <div class="py-5">
    <h3 class="text-lg font-black">{{ title }}</h3>
  </div>
  <div class="text-red-500">
    {% block blog_content %}
    {% endblock blog_content %}
  </div>
  {% endblock content %}
"#;

pub const BLOG_POST: &str = r#"{% extends "layouts/blog.jinja" %}
{% block blog_content %}
  Blog content here
{% endblock blog_content %}
"#;

pub const FOOTER: &str = r#"<footer>
  <p>FOOTER</p>
</footer>
"#;
