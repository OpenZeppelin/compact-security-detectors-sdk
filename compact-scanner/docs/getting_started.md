<!-- docs/getting_started.md -->
# Getting Started

This guide covers building, installing, and running **compact-scanner** locally.

## Prerequisites
- Rust toolchain (stable) – install via https://rustup.rs
- `cargo` and `rustc` should be available in your `PATH`

## Building
```bash
cargo build --release
```

## Running
```bash
# Scan .compact files in a directory
./target/release/compact-scanner --code path/to/compact/files
```

## Example
```bash
cargo run -- --code examples/contract.compact
```

For additional options, see [CLI Usage](cli_usage.md).