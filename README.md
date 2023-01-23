# Shazam
Shazam is a static site generator for masochists

## License
[MIT](LICENSE).

## Features
Shazam's goal is to give you the tools to create a static website without getting in your way.

* Templating engine - [https://github.com/Keats/tera](tera)
* Development server - [https://github.com/tokio-rs/axum](axum)

A newly initialize project uses `tailwindcss` for stylying and `highlightjs` for code blocks.
You can how ever you any CSS and/or code highlighting frameworks you like by importing them.

At them moment Shazam also uses a couple of other project until the features are implemented 
directly into Shazam.

* Process management - [Overmind](https://github.com/DarthSim/overmind)
* Monitor file changes - [Reflex](https://github.com/cespare/reflex)

> Note: At the moment, Shazam is only tested on Linux and Docker.
> It will probably work on MacOS.
> Native Windows supported is targeted in a future release.

## Getting Started

### Docker
The quickest way to get started is with Docker and the [Shazam Starter Template](https://github.com/bwks/shazam-starter). 


> It is assumed that you already have Docker with Compose already installed.
> This template was tested against Docker CE - 20.10.21

#### Clone the starter template
```
git clone --depth 1 git@github.com:bwks/shazam-starter.git <project-name>
```

#### Move to project directory
```
cd <project-name>
```

#### Remove Git History
```
rm -rf .git
```

#### Update project variables in `bin/dev` file
```
APP_NAME="<UPDATE>";
APP_OWNER="<UPDATE>";
APP_OWNER_EMAIL="<UPDATE>";
```

#### Initialize the project
```
bin/dev init
```

#### Start the dev server
```
bin/dev up
```

### Native install

* Install Overmind
* Download tailwindcss binary
* Download reflex binary
* Download shazam binary

### Initialize Project
Use the `shazam init <project-name>` command to initialize a project.
```
./shazam init test
```

This will build a project named `test` with the following structure.
```
test
├── _site
│   ├── blog
│   │   ├── index.html
│   │   └── test-blog
│   │       └── index.html
│   ├── css
│   ├── error
│   ├── favicon
│   ├── font
│   ├── img
│   ├── index.html
│   └── js
├── assets
│   ├── css
│   │   └── input.css
│   ├── error
│   ├── favicon
│   ├── font
│   ├── img
│   └── js
├── blog
│   └── test-blog.jinja
├── config
├── data
│   └── blog.json
└── templates
    ├── includes
    │   └── footer.jinja
    └── layouts
        ├── base.jinja
        └── blog.jinja
```

### Dev Server
Use `overmind` to start the dev server and begin tailwind file watcher.
```
> ./overmind s

# output
system | Tmux socket name: overmind-shazam-JKQwwgZxb6JUmMUlSZ9Pp
system | Tmux session ID: shazam
system | Listening at ./.overmind.sock
build  | Started with pid 763529...
css    | Started with pid 763527...
web    | Started with pid 763525...
web    |     Finished dev [unoptimized + debuginfo] target(s) in 0.05s
web    |      Running `target/debug/shazam build`
web    | Project: `test` => building ...
web    | Project: `test` => build complete
web    |     Finished dev [unoptimized + debuginfo] target(s) in 0.04s
web    |      Running `target/debug/shazam serve`
web    | listening on 0.0.0.0:3000
css    | 
css    | Rebuilding...
css    | Done in 160ms.
```

Now you can access the site via http from your browser.

### Run the dev server with cargo
```
./overmind s -f Procfile.dev
```

