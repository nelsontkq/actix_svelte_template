# Actix Svelte Template

Here's an actix-web template that uses SvelteKit built and served as static files.

## Features

- [Actix web](https://actix.rs/) server
- [Diesel.rs](https://diesel.rs) ORM with migrations, and r2d2 for connection pooling
- [SvelteKit](https://kit.svelte.dev/) for frontend, served as static files


## Setup

We use Vite's proxy in the dev environment and serve svelte as static files in production.

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

### Script
```bash
npm run dev
```

All traffic to localhost:3000/api/* will be forwarded to the actix web project, and anything else to the SvelteKit frontend.

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