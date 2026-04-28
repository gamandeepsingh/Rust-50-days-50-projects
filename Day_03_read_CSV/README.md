# Day 03 — Read CSV

A Rust program that parses and reads CSV files using the `csv` crate.

## What it does

- Opens a CSV file
- Iterates through each record
- Prints each row as a debug-formatted record

## Run

```bash
cargo run
```

## Notes

- Hardcoded to read `./customers.csv` in the current directory
- Uses Rust's error handling with `Result` types
- Simple record parsing with automatic error reporting
