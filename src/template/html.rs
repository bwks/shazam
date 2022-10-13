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
        <h1 class="text-2xl font-black">{{ project | capitalize }} Site</h1>
      {% endblock page_header %}
      {% block content %}
        <a href="/blog/">Blogs</a>
      {% endblock content %}
    </main>
  </body>
  <footer>
    <div class="text-lg text-indigo-500">
      {% block footer_content %}
      {% endblock footer_content %}
    </div>
  </footer>
</html>
"#;

pub const BLOG: &str = r#"{% extends "layouts/base.jinja" %}
{% block page_header %}
  <h1 class="text-2xl font-black">Blog Posts</h1>
{% endblock page_header %}
{% block content %}
  <div class="text-fuchsia-500">
    {% for post in posts %}
      <p>
        <a href="/blog/{{ post.title | parameterize }}">{{ post.title | title_case }} | {{ post.published_date }}</a>
      </p>
    {% endfor %}
  </div>
{% endblock content %}
{% block footer_content %}
  <p><a href="/">Back to home</a></p>
{% endblock footer_content %}
"#;

pub const BLOG_POST: &str = r#"{% extends "layouts/blog.jinja" %}
{% block page_header %}
  <h1 class="text-2xl font-black">{{ post.title | title_case }}</h1>
  <p class="text-gray-500">published: {{ post.published_date | human_date }}</p>
{% endblock page_header %}
{% block content %}
  <p>
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Nec dui nunc mattis enim ut tellus elementum sagittis vitae. Sagittis orci a scelerisque purus semper eget duis at tellus. Libero enim sed faucibus turpis. Nulla aliquet enim tortor at auctor. Venenatis cras sed felis eget velit aliquet. Viverra maecenas accumsan lacus vel facilisis. Sit amet nisl suscipit adipiscing bibendum. Mi tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Elementum integer enim neque volutpat. Pellentesque sit amet porttitor eget dolor morbi non arcu. Sed ullamcorper morbi tincidunt ornare massa eget. Orci dapibus ultrices in iaculis nunc. Venenatis tellus in metus vulputate eu. At auctor urna nunc id cursus metus aliquam eleifend.
  </p>
  <p>
    Elementum tempus egestas sed sed risus pretium. Vitae ultricies leo integer malesuada nunc vel risus commodo. Tellus molestie nunc non blandit massa enim nec dui. Non consectetur a erat nam at. Sapien eget mi proin sed libero enim sed faucibus turpis. Sit amet est placerat in egestas. Pellentesque id nibh tortor id aliquet. Lacus sed turpis tincidunt id aliquet risus. Dolor morbi non arcu risus. Tortor posuere ac ut consequat semper.
  </p>
{% endblock content %}
{% block footer_content %}
  <p><a href="/blog">Back to blogs</a></p>
{% endblock footer_content %}
"#;

pub const FOOTER: &str = r#"<p><a href="/">Back to home</a></p>"#;
