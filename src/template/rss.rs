pub const RSS_FEED_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <atom:link href="{{ config.project }}.com" rel="self" type="application/rss+xml"/>
    <title>{{ config.project | title_case }} Blog</title>
    <link>{{ config.project }}.com/blog/</link>
    <description>Super awesome blog</description>
    <copyright>Copyright {{ config.owner }} {{ now() | date(format="%Y") }} All Rights Reserved</copyright>
    <language>en-US</language>
    <ttl>60</ttl>
    {%- for post in posts.by_content.blog %}
    {%-   if post.publish %}
    <item>
      <title>{{ post.title }}</title>
      <link>{{ config.project }}.com/blog/{{ post.title | slugify }}</link>
      <guid isPermaLink="false">{{ config.project }}.com/blog/{{ post.title | slugify }}-{% if post.updated_date %}{{ post.updated_date }}{% else %}{{ post.published_date }}{% endif %}</guid>
      <description>{{ post.description }}</description>
      <content:encoded><![CDATA[<p>{{ config.project }}.com/blog/{{ post.title | slugify }}</p>]]></content:encoded>
      <pubDate>{{ post.published_date }}</pubDate>
      <category>{{ post.category }}</category>
    </item>
    {%-   endif %}
    {%- endfor %}
  </channel>
</rss>"#;
