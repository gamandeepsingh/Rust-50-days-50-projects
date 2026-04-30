# Day 07 — Async Await

A small Rust program that fetches multiple URLs concurrently using async/await with `tokio`, `reqwest`, and `futures`.

## What it does

- Defines an async function to fetch a URL body
- Creates multiple HTTP requests
- Runs all requests concurrently with `futures::future::join_all`
- Prints each result or error

## Run

```bash
cargo run
```

## Notes

- Uses `#[tokio::main]` to run async code
- `join_all` waits for all tasks to complete
- Good example of concurrent I/O in Rust without spawning OS threads
