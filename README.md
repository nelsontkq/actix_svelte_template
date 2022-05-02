# Actix Svelte Template

Here's an actix-web template that uses SvelteKit built and served as static files.

## Features

- [Actix web](https://actix.rs/) server
- [Diesel.rs](https://diesel.rs) ORM with migrations, and r2d2 for connection pooling
- [SvelteKit](https://kit.svelte.dev/) for frontend, served as static files


## Setup

### Dev Requirements

- [ ] cargo: `curl https://sh.rustup.rs -sSf | sh`
- [ ] diesel_cli: `cargo install diesel_cli --no-default-features --features sqlite`
- [ ] node: https://nodejs.org/en/download/current/
- [ ] sqlite3: `apt install sqlite3 libsqlite3-dev`

### run initial migrations

```bash
diesel setup
diesel migration run
```

## Debug

We use nginx as a reverse proxy when developing locally to allow cross-origin requests.
This isn't required in production as the SvelteKit frontend is compiled to static files.

### Script
```bash
./debug.sh
```

This will create a nginx proxy which will forward /api to the actix web project, and anything else to the SvelteKit frontend.

### VS Code

I've added a working `launch.json` that will setup the nginx proxy, and debug both front and backend.

### Build

You can build the project with cargo. The `build.rs` will automatically compile the frontend to static files in the ./client/build directory.

```bash
cargo build --release
```

For convenience a Dockerfile was created which handles compiling the frontend to static files and building the Actix Web server into a 20mb Alpine image.

```bash
docker build -t actix-svelte-template .
docker run -d -p 8080:8080 actix-svelte-template
```