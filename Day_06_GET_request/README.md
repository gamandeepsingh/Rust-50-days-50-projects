# Day 06 — GET Request

A small Rust program that sends an HTTP GET request and deserializes the JSON response with `reqwest`, `serde`, and `tokio`.

## What it does

- Sends a GET request to `https://jsonplaceholder.typicode.com/posts`
- Deserializes the JSON response into a Rust struct
- Prints the parsed data to the console

## Run

```bash
cargo run
```

## Notes

- Uses `tokio` for the async runtime
- Uses `reqwest` for the HTTP client
- Uses `serde::Deserialize` to map JSON into a Rust struct
