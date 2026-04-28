# Day 02 — Decompress

A Rust CLI that extracts files from a ZIP archive. Handles nested directories and preserves Unix file permissions.

## What it does

- Opens a ZIP archive
- Extracts all files and directories with their structure preserved
- Prints extraction details for each file
- Preserves Unix file permissions on supported systems

## Run

```bash
cargo run -- <zip_file>
```

Example:

```bash
cargo run -- 1.zip
```

## Notes

- Creates necessary directories automatically
- Skips directory entries gracefully
- Works with any valid ZIP archive format
