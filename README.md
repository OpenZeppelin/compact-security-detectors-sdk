[![Build](https://github.com/OpenZeppelin/compact-security-detectors-sdk/actions/workflows/build.yml/badge.svg)](https://github.com/OpenZeppelin/compact-security-detectors-sdk/actions/workflows/build.yml)
[![Release](https://github.com/OpenZeppelin/compact-security-detectors-sdk/actions/workflows/release.yml/badge.svg)](https://github.com/OpenZeppelin/compact-security-detectors-sdk/actions/workflows/release.yml)

# Compact Security Detectors

A suite of tools and libraries for analyzing `Compact` language for security vulnerabilities within the Midnight platform.

## Repository Structure

This workspace includes the following crates:

- `sdk`: Core SDK for building the AST, codebase, and writing custom security detectors.
- `detectors`: Built-in security detectors for common vulnerability patterns.
- `compact-scanner`: CLI tool to scan `.compact` files and run detectors.

## Quickstart

1. Clone the repository:
   ```sh
   git clone https://github.com/OpenZeppelin/compact-security-detectors-sdk.git
   ```

2. Restore submodules:
   ```sh
   git submodule update --init --recursive
   ```

3. Build the project:
   ```sh
   cargo build
   ```

4. Scan a directory of `.compact` files:
   ```sh
   compact-scanner scan ./path/to/compact/files
   ```

## Documentation

Detailed developer guides for each crate:

- SDK: [README](./sdk/README.md)
- Detectors: [README](./detectors/README.md)
- CLI Scanner: [README](./compact-scanner/README.md)

## Architecture Overview

```mermaid
graph TD;
  subgraph Parser Layer
    I[Compact code] --> G[tree-sitter-compact]
  end
  subgraph Core SDK
    G --> C[sdk]
  end
  subgraph Analysis
    C --> D[detectors]
    C --> S[compact-scanner]
  end
  S -->|Results| O[Output JSON/Console]
```

## Contributing

See [contributing.md](./contributing.md) for guidelines.

## Style Guidelines

See [style guidelines](./style_guidelines.md) for coding standards and best practices.

## License

[AGPLv3](./LICENSE)
