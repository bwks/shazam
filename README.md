# Shazam
Shazam is a static site generator for masochists

## License
[MIT](LICENSE).

## Features
Shazam's goal is to give you the tools to create a static website without getting in your way.

* Templating engine - [Tera](https://github.com/Keats/tera)
* Development server - [Axum](https://github.com/tokio-rs/axum)

A newly initialize project uses `tailwindcss` for stylying and `highlightjs` for code blocks.
You can however, use any CSS and/or code highlighting frameworks you like by importing them.

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
The native install requires downloading the variout binaries.

#### Overmind
Download Overmind which is used to manage `tailwind`, `reflex` and `shazam`.

```
curl -sLO https://github.com/DarthSim/overmind/releases/latest/download/overmind-v2.3.0-linux-amd64.gz \
  && gunzip overmind-v2.3.0-linux-amd64.gz \
  && chmod +x overmind-v2.3.0-linux-amd64 \
  && mv overmind-v2.3.0-linux-amd64 overmind
```

#### Tailwind CSS
Download the Tailwind CSS CLI to manage building the sites CSS files.

```
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
  && chmod +x tailwindcss-linux-x64 \
  && mv tailwindcss-linux-x64 tailwindcss
```

#### Reflex
Download Reflex which watches for template file changes and executes a rebuild on change.

```
curl -sLO https://github.com/cespare/reflex/releases/latest/download/reflex_linux_amd64.tar.gz \
  && tar -xvf reflex_linux_amd64.tar.gz \
  && chmod +x reflex_linux_amd64/reflex \
  && mv reflex_linux_amd64/reflex reflex \ 
  && rm -rf reflex_linux_amd64*
```

#### Shazam
Download the Shazam binary to manage the static site.

```
curl -sLO https://github.com/bwks/shazam/releases/latest/download/shazam-x86_64-unknown-linux-gnu.tar.gz  \
  && tar -xvf  shazam-x86_64-unknown-linux-gnu.tar.gz  \
  && chmod +x shazam \
  && rm shazam-x86_64-unknown-linux-gnu.tar.gz
```

### Initialize Project
Use the `shazam init` command to initialize a project.

```
./shazam init --name test --owner blah --owner-email blah@blah.blah
```

### Dev Server
Use `overmind` to start the dev server.
```
> ./overmind s

# output
system | Tmux socket name: overmind-test-dppG17E7b_AmVHmQ6fzMs
system | Tmux session ID: test
system | Listening at ./.overmind.sock
css    | Started with pid 291519...
build  | Started with pid 291522...
web    | Started with pid 291517...
web    | 2023-01-24T10:07:03.564088Z  INFO shazam: Project: `test` => building ...
web    | 2023-01-24T10:07:03.567816Z  INFO shazam: Project: `test` => build complete
web    | 2023-01-24T10:07:03.572529Z  INFO shazam: listening on 0.0.0.0:3000
css    |
css    | Rebuilding...
css    |
css    | Done in 182ms.
```

Now you can access the site via http from your browser.
