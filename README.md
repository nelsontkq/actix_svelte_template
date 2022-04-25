# Actix Svelte Template

Here's a actix-web template that uses SvelteKit built as static files.

## Features

- [Actix web](https://actix.rs/) server
- [Diesel.rs](https://diesel.rs) ORM with migrations, and using r2d2 for connection pooling
- [SvelteKit](https://kit.svelte.dev/) for frontend, served as static files


## Setup

### install diesel cli and run migrations


```bash
sudo apt install sqlite3 libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
diesel migration run
```

### Run

```bash
cargo run
```
