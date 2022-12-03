pub const SITE_LAYOUT: &str = r#"{% import "_macros/page-header.jinja" as page_header %}
{% import "_macros/link-to.jinja" as link_to %}
{% import "_macros/tags.jinja" as tags %}
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
    <script>hljs.highlightAll();</script>

    <title>{{ config.project }}</title>
  </head>

  <body class="antiailiased grid place-items-center">
    {% block page_header %}
      {{ page_header::page_header(heading=config.project) }}
    {% endblock page_header %}

    <div class="w-3/4 px-10">
      {% block page_content %}
      {% endblock page_content %}
    </div>

    {% block page_tags %}
    {% endblock page_tags %}
  </body>

  <footer>
    <div class="text-lg text-indigo-500">
      {% block page_footer %}
        {% include "_includes/footer.jinja" %}
      {% endblock page_footer %}
    </div>
  </footer>
</html>
"#;

pub const BLOG_LAYOUT: &str = r#"{% extends "_layouts/site.jinja" %}

{% block page_header %}
  {% if post %}
    {{ page_header::page_header(heading=post.title, published_date=post.published_date) }}
  {% else %}
    {{ page_header::page_header(heading="Blog Posts") }}
  {% endif %}
{% endblock page_header %}

{% block page_content %}
{% endblock page_content %}

{% block page_tags %}
  {% if post %}
    {{ tags::tags(tags=post.tags) }}
  {% endif %}
{% endblock page_tags %}

{% block page_footer %}
  {{ link_to::link_to(link="/", description="Back to home") }}
{% endblock page_footer %}
"#;

pub const FOOTER_INCLUDE: &str = r#"<p>I'm a footer</p>"#;

pub const LOREM_IPSUM_INCLUDE: &str = r#"<p class="py-2">
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Nec dui nunc mattis enim ut tellus elementum sagittis vitae. Sagittis orci a scelerisque purus semper eget duis at tellus. Libero enim sed faucibus turpis. Nulla aliquet enim tortor at auctor. Venenatis cras sed felis eget velit aliquet. Viverra maecenas accumsan lacus vel facilisis. Sit amet nisl suscipit adipiscing bibendum. Mi tempus imperdiet nulla malesuada pellentesque elit eget gravida cum. Elementum integer enim neque volutpat. Pellentesque sit amet porttitor eget dolor morbi non arcu. Sed ullamcorper morbi tincidunt ornare massa eget. Orci dapibus ultrices in iaculis nunc. Venenatis tellus in metus vulputate eu. At auctor urna nunc id cursus metus aliquam eleifend.
</p>
<p class="py-2">
  Elementum tempus egestas sed sed risus pretium. Vitae ultricies leo integer malesuada nunc vel risus commodo. Tellus molestie nunc non blandit massa enim nec dui. Non consectetur a erat nam at. Sapien eget mi proin sed libero enim sed faucibus turpis. Sit amet est placerat in egestas. Pellentesque id nibh tortor id aliquet. Lacus sed turpis tincidunt id aliquet risus. Dolor morbi non arcu risus. Tortor posuere ac ut consequat semper.
</p>
"#;

pub const PAGE_HEADER_MACRO: &str = r#"{% macro page_header(heading, published_date="") %}
<div class="pt-5">
  <h1 class="text-5xl pb-3 font-black">{{ heading | title }}</h1>
  {% if published_date %}
    <p class="text-gray-500 italic">published: {{ published_date | human_date }}</p>
  {% endif %}
</div>
{% endmacro %}
"#;

pub const LINK_TO_MACRO: &str = r#"{% macro link_to(link, description="") %}
<div class="">
  <a class="no-underline hover:underline" href="{{ link }}">{% if description %}{{ description }}{% else %}{{ link }}{% endif %}</a>
</div>
{% endmacro %}
"#;

pub const TAGS_MACRO: &str = r#"{% macro tags(tags) %}
{% if tags %}
  <div class="py-3">
    {% for tag in tags %}
      <div class="inline-block px-2.5 py-1 bg-rose-200 text-rose-800 text-s font-semibold italic leading-tight lowercase rounded shadow-md">{{ tag }}</div>
    {% endfor %}
  </div>
{% endif %}
{% endmacro %}
"#;

pub const SITE_INDEX_TEMPLATE: &str = r#"{% extends "_layouts/site.jinja" %}
{% block page_content %}
  <div class="grid place-items-center">
    <h3 class="font-medium text-2xl">Super Awesome Content</h3>
  </div>
  {% for content in config.content_dirs %}
    <div class="py-2">
      <div class="block p-4 rounded-lg shadow-lg bg-white border-2">
        <a class="text-fuchsia-500 font-semibold text-xl no-underline hover:underline" href="/{{ content }}/">{{ content | capitalize }}</a>
      </div>
    </div>
  {% endfor %}
{% endblock page_content %}
"#;

pub const BLOG_INDEX_TEMPLATE: &str = r#"{% extends "_layouts/blog.jinja" %}
{% block page_content %}
  <div class="">
    {% for key, value in posts.by_category %}
      {% for post in value %}
        {% if post.publish %}
          <div class="py-2">
            <div class="block p-4 rounded-lg shadow-lg bg-white border-2">
              <a class="text-fuchsia-500 font-semibold text-xl no-underline hover:underline" href="/blog/{{ post.title | slugify }}">{{ post.title | title }}</a>
              <p class="text-gray-400 text-md italic">
                published: {{ post.published_date }}
              </p>
              <p class="text-gray-800 text-lg">
                {{ post.description }}
              </p>
              <div class="">
                {{ tags::tags(tags=post.tags) }}
              </div>
            </div>
          </div>
        {% endif %}
      {% endfor %}
    {% endfor %}
  </div>
{% endblock page_content %}
"#;

pub const BLOG_POST_TEMPLATE: &str = r#"{% extends "_layouts/blog.jinja" %}
{% block page_content %}
  <div class="px-5 text-lg text-gray-800">
    {% include "_includes/lorem-ipsum.jinja" %}
  </div>
  <div class="px-5 py-3">
    <pre><code class="language-rust hljs">
fn main() {
    // Print text to the console
    println!("Hello World!");
}
    </code></pre>
  </div>
{% endblock page_content %}
{% block page_footer %}
  {{ link_to::link_to(link="/blog", description="Back to blogs") }}
{% endblock page_footer %}
"#;
