# Day 08 — API Calls (CRUD)

A small Rust program demonstrating basic CRUD-style API calls from the command line.

## What it does

- `cargo run -- get` — Send a GET request and print resources
- `cargo run -- create` — Send a POST request to create a resource
- `cargo run -- update --id 1` — Send a PUT/PATCH to update resource with id 1
- `cargo run -- delete --id 1` — Send a DELETE request for resource with id 1

## Run

Examples:

```bash
cargo run -- get
cargo run -- create
cargo run -- update --id 1
cargo run -- delete --id 1
```

## Notes

- Uses `reqwest`, `serde`, and an async runtime (`tokio`) for HTTP calls
- CLI parsing may use `structopt`/`clap` or manual arg handling
- Replace the target API URL inside `main.rs` as needed for your backend
