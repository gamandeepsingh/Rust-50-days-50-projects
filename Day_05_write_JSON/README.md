# Day 05 — Write JSON

A small Rust program that serializes a Rust struct into JSON using `serde` and `serde_json`.

## What it does

- Defines a struct with fields like name, age, and location
- Uses `#[derive(Serialize)]` to enable JSON serialization
- Converts the struct to a prettified JSON string
- Prints the JSON output to console

## Run

```bash
cargo run
```

## Notes

- Uses `serde_json::to_string_pretty()` for formatted JSON output
- Optional fields are supported using `Option<T>`
- `serde` is a powerful framework for serializing and deserializing Rust data structures
