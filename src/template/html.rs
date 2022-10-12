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
      {% block page_header %}
        <h1 class="text-2xl font-black">{{ project }} site</h1>
      {% endblock page_header %}
      {% block content %}
        <a href="/blog/">Blogs</a>
      {% endblock content %}
    </main>
  </body>
  {% include "includes/footer.jinja" %}
</html>
"#;

pub const BLOG: &str = r#"{% extends "layouts/base.jinja" %}
{% block page_header %}
  <h1 class="text-2xl font-black">Blog Posts</h1>
{% endblock page_header %}
{% block content %}
  <div class="text-red-500">
    {% for post in posts %}
      <a href="/blog/{{ post.title | dasherize }}">{{ post.title | title_case }} | {{ post.published_date }}</a>
    {% endfor %}
  </div>
{% endblock content %}
"#;

pub const BLOG_POST: &str = r#"{% extends "layouts/blog.jinja" %}
{% block page_header %}
  <h1 class="text-2xl font-black">{{ post.title | title_case }}</h1>
  <p class="text-gray-500">published: {{ post.published_date | human_date }}</p>
{% endblock page_header %}
{% block content %}
  Blog content here
{% endblock content %}
"#;

pub const FOOTER: &str = r#"<footer>
  <p>FOOTER</p>
</footer>
"#;
