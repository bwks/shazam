# Shazam
Shazam is a static site generator for masochists

## License
[MIT](LICENSE).

## Features
* `TailwindCSS` - CSS framework
* `HighlightJS` - Code block syntax highlighting
* `Mini Jinja` - Post templates
* `Development Server` - Built with Axum
* `Overmind` - Process monitoring
* `Refelx` - Rebuild site in file change

## Getting Started
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

## Docker
Build the container using `docker compose`
```
docker compose \
  -f docker/docker-compose.yaml \
  --env-file docker/.env \
  build \
    --build-arg APP_USER=$USER \
    --build-arg APP_NAME=test \
    --build-arg APP_USER_ID=$UID \
    --build-arg APP_GROUP_ID=$GID \
    --build-arg HOME_DIR=$HOME
```

If you have not yet initialized a project, then run the following to 
create the project files/folders in your current directory.
```
export APP_NAME="test" \
  && docker container run -itd --name=shazam-tmp shazam ash \
  && docker container cp shazam-tmp:$HOME/$APP_NAME $APP_NAME \
  && docker container cp shazam-tmp:$HOME/config config \
  && docker container kill shazam-tmp \
  && docker container rm shazam-tmp
```

Bring the container up
```
docker compose \
  -f docker-compose.yaml \
  --env-file .env \
  up
```

Shut the container down when you are done.
```
docker compose \
  -f docker-compose.yaml \
  --env-file .env \
  down
```