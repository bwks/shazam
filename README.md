# Shazam
Shazam is a static site generator writen in Rust with 
a minimal set of features.

# TODO
* ~~Initialize Project~~
* ~~Load Config~~
* ~~Build site~~
* ~~HTTP server~~
* ~~ Rebuild on file change~~

# Features

## CSS
* TailwindCSS

## Javascript
* HighlightJS

## Template Engine
* Mini Jinja

## Process Watcher
* Overmind

## Folder Structure
```test
├── _site
│   ├── blog
│   │   └── index.html
│   ├── css
│   │   └── app.css
│   ├── error
│   ├── favicon
│   ├── font
│   ├── img
│   ├── index.html
│   └── js
├── assets
│   ├── css
│   │   └── input.css
│   ├── error
│   ├── favicon
│   ├── font
│   ├── img
│   └── js
├── blog
├── config
├── data
│   └── data.json
└── templates
    ├── includes
    │   └── _footer.jinja
    └── layouts
        ├── base.jinja
        └── blog.jinja
```
