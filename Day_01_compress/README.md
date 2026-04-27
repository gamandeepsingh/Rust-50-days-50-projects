# Day 01 — Compress

A small Rust CLI that compresses any input file into a gzip file using `flate2`.

## What it does

- Reads a source file
- Writes a compressed gzip file to the target path
- Prints source size, target size, and elapsed time

## Run

```bash
cargo run -- <source_file> <target_file>
```

Example:

```bash
cargo run -- book.pdf abc.pdf
```

## Notes

- The target file is created as gzip-compressed output.
- Use any file type you want, not just PDFs.
