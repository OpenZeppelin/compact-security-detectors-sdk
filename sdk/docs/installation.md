<!-- docs/installation.md -->
[← Back to Index](index.md)

# Installation

## Prerequisites

- Rust toolchain (1.60+)
- `cargo` package manager
- C toolchain (for Tree-sitter if building from source)

## Clone the Repository

```bash
git clone https://github.com/OpenZeppelin/compact-security-detectors-sdk.git
cd compact-security-detectors-sdk
```

## Build

```bash
cargo build --release
```

## Add to Your Project

Add the following to your `Cargo.toml`:

```toml
[dependencies]
compact-security-detectors-sdk = { git = "https://github.com/OpenZeppelin/compact-security-detectors-sdk.git" }
```

[Previous: Overview](overview.md)  
[Next: Getting Started](getting_started.md)