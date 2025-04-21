<!-- docs/overview.md -->
[← Back to Index](index.md)

# Overview

The `midnight-security-detectors-sdk` is a Rust library that provides tooling to parse, analyze, and detect security-related issues in source code written in the Compact DSL using Tree-sitter.

## Key Components

- AST Construction via the `ast` module
- Codebase representation with symbol tables (`codebase` module)
- Generic detector framework (`Detector` and `DetectorReportTemplate` traits)
- Link resolution for imports and function calls
- Storage abstraction for AST nodes

This SDK is designed to be extended with custom detectors that implement the `Detector` trait.

[Next: Installation](installation.md)