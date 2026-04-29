# Day 04 — Read JSON

A Rust program that parses JSON files into structured types using `serde_json`.

## What it does

- Reads a JSON file
- Deserializes JSON into a Rust struct
- Prints the parsed data

## Run

```bash
cargo run
```

## Notes

- Reads from `data.json` in the current directory
- Uses `serde` and `serde_json` for serialization/deserialization
- Define your own struct types with `#[derive(Deserialize)]`
- Strongly typed, so invalid JSON or mismatched structure will error
