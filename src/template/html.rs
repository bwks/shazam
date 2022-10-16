pub const SITE_TEMPLATE: &str = r#"{% from "includes/page-header.jinja" import page_header %}
{% from "includes/link-to.jinja" import link_to %}
{% from "includes/tags.jinja" import tags %}
<!DOCTYPE html>
<html lang="en" class="h-full">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">

    <link rel="stylesheet" href="/css/app.css">
    <link rel="icon" href="./favicon.ico" type="image/x-icon">
    <script src="/js/app.js"></script>

    <link rel="stylesheet"
      href="//cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.6.0/build/styles/tokyo-night-dark.min.css">
    <script src="//cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.6.0/build/highlight.min.js"></script>

    <!-- Initialize highlight.js -->
    <script>hljs.initHighlightingOnLoad();</script>

    <title>{{ project }}</title>
  </head>
  <body class="h-full antiailiased container mx-auto">
    {% block page_header %}
      {{ page_header(heading=project + " Site") }}
    {% endblock page_header %}
    <div class="pl-5">
      {% block content %}
      {% endblock content %}
    </div>
  </body>
  <footer>
    <div class="text-lg text-indigo-500 pl-5">
      {% block page_footer %}
        {% include "includes/footer.jinja" %}
      {% endblock page_footer %}
    </div>
  </footer>
</html>

"#;

pub const SITE_INDEX_TEMPLATE: &str = r#"{% extends "layouts/site.jinja" %}
{% block content %}
  <div>
    <h3 class="font-bold text-2xl">Post Categories</h3>
  </div>
  {% for content in config.content_dirs %}
    <div class="py-2 max-w-sm">
      <div class="block p-4 rounded-lg shadow-lg bg-white border-2 max-w-xs">
        <a class="no-underline hover:underline" href="/{{ content }}/">{{ content | capitalize }}</a>
      </div>
    </div>
  {% endfor %}
{% endblock content %}
"#;

pub const BLOG_INDEX_TEMPLATE: &str = r#"{% extends "layouts/site.jinja" %}
{% block page_header %}
  {{ page_header(heading="Blog Posts") }}
{% endblock page_header %}
{% block content %}
  <div class="text-fuchsia-500">
    {% for post in posts %}
      <div class="py-2">
        <div class="block p-4 rounded-lg shadow-lg bg-white border-2 max-w-xs">
          <a class="no-underline hover:underline" href="/blog/{{ post.title | parameterize }}">{{ post.title | title_case }}</a>
          <p class="text-gray-400 text-sm">
            published: {{ post.published_date }}
          </p>
          <p class="text-gray-800">
            {{ post.description }}
          </p>
        </div>
      </div>
    {% endfor %}
  </div>
{% endblock content %}
{% block page_footer %}
  {{ link_to(link="/", description="Back to home") }}
{% endblock page_footer %}
"#;

pub const BLOG_POST_TEMPLATE: &str = r#"{% extends "layouts/blog.jinja" %}
{% block page_header %}
  {{ page_header(heading=post.title, published_date=post.published_date) }}
{% endblock page_header %}
{% block content %}
  <div class="px-5">
    {% include "includes/lorem-ipsum.jinja" %}
  </div>
  <div class="px-5 py-3">
    <pre><code class="language-rust hljs">
fn main() {
    // Print text to the console
    println!("Hello World!");
}
    </code></pre>
  </div>
  {% if post.tags %}
    {{ tags(tags=post.tags) }}
  {% endif %}
{% endblock content %}
{% block page_footer %}
  {{ link_to(link="/blog", description="Back to blogs") }}
{% endblock page_footer %}
"#;

pub const FOOTER_TEMPLATE: &str = r#"<p>I'm a footer</p>"#;

pub const LOREM_IPSUM_TEMPLATE: &str = r#"<p class="py-2">
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Nec dui nunc mattis enim ut tellus elementum sagittis vitae. Sagittis orci a scelerisque purus semper eget duis at tellus. Libero enim sed faucibus turpis. Nulla aliquet enim tortor at auctor. Venenatis cras sed felis eget velit aliquet. Viverra maecenas accumsan lacus vel facilisis. Sit amet nisl suscipit adipiscing bibendum. Mi tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Elementum integer enim neque volutpat. Pellentesque sit amet porttitor eget dolor morbi non arcu. Sed ullamcorper morbi tincidunt ornare massa eget. Orci dapibus ultrices in iaculis nunc. Venenatis tellus in metus vulputate eu. At auctor urna nunc id cursus metus aliquam eleifend.
</p>
<p class="py-2">
  Elementum tempus egestas sed sed risus pretium. Vitae ultricies leo integer malesuada nunc vel risus commodo. Tellus molestie nunc non blandit massa enim nec dui. Non consectetur a erat nam at. Sapien eget mi proin sed libero enim sed faucibus turpis. Sit amet est placerat in egestas. Pellentesque id nibh tortor id aliquet. Lacus sed turpis tincidunt id aliquet risus. Dolor morbi non arcu risus. Tortor posuere ac ut consequat semper.
</p>
"#;

pub const PAGE_HEADER_TEMPLATE: &str = r#"{% macro page_header(heading, published_date="") %}
<div class="pt-5 pl-5">
  <h1 class="text-5xl pb-3 font-black">{{ heading | title_case }}</h1>
  {% if published_date %}
    <p class="text-gray-500">published: {{ published_date | human_date }}</p>
  {% endif %}
</div>
{% endmacro %}
{% set alias = page_header %}
"#;

pub const LINK_TO_TEMPLATE: &str = r#"{% macro link_to(link, description="") %}
<div class="">
  <a class="no-underline hover:underline" href="{{ link }}">{{ description if description else link }}</a>
</div>
{% endmacro %}
{% set alias = link_to %}
"#;

pub const TAGS_TEMPLATE: &str = r#"{% macro tags(tags) %}
<div class="py-3">
  {% for tag in tags %}
      <div class="inline-block px-2.5 py-1 bg-rose-200 text-rose-800 text-s font-semibold italic leading-tight lowercase rounded shadow-md">{{ tag }}</div>
    {% endfor %}
</div>
{% endmacro %}
{% set alias = tags %}
"#;
