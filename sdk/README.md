# Compact Security Detectors SDK

Core SDK for building and running security detectors on `Compact` language.

## Overview

The `compact-security-detectors-sdk` is a Rust library that provides tooling to parse, analyze, and detect security-related issues in source code written in the Compact DSL.

## Key Components

- AST Construction via the `ast` module
- Codebase representation with symbol tables (`codebase` module)
- Generic detector framework (`Detector` and `DetectorReportTemplate` traits)

## Installation

Add to your `Cargo.toml`:
```sh
cargo add compact-security-detectors-sdk
```

And then in the `rs` file:
 ```rust
use compact_security_detectors_sdk::{
    codebase::{Codebase, SealedState},
    detector::DetectorResult,
};
 ```

See detector examples in [`detectors`](../detectors/docs/index.md) directory.

## Testing

Run tests:
```sh
cargo test
```

or with coverage:
```sh
cargo tarpaulin
```

## Build a Codebase

```rust
use compact_security_detectors_sdk::{build_codebase, DetectorResult};

let mut files = std::collections::HashMap::new();
files.insert("example.comp".to_string(), r#"
circuit Main {
    signal x: u8;
    // ...
}
"#.to_string());

let codebase = build_codebase(&files).unwrap();
```

## Run a Detector

Implement the `Detector` and `DetectorReportTemplate` traits and register your detectors. The `check` method returns a list of `DetectorResult`.

```rust
if let Some(results) = detector.check(&codebase) {
    for result in results {
        println!("{:?}", result);
    }
}
```

## Writing Detectors

Custom detectors implement two traits:

1. `Detector`:
   - Defines the `check(&self, codebase: &Codebase<SealedState>) -> Option<Vec<DetectorResult>>` method.
   - Analysis logic goes here.
2. `DetectorReportTemplate`:
   - Provides metadata: name, description, severity, tags.
   - Template methods for report formatting.
2. `detector!` macro:
   - Simplifies detector creation.
   - Generates a struct implementing `Detector` and `DetectorReportTemplate`.

### Example

```rust
use compact_security_detectors_sdk::{
    codebase::{Codebase, SealedState},
    detector::DetectorResult,
};

compact_security_detectors_sdk::detector! {
    #[type_name = ExampleDetector]
    pub fn example_detector(
        codebase: &Codebase<SealedState>
    ) -> Option<Vec<DetectorResult>> {
        // Detector logic here
        Some(vec![])
    }
}
```

## Architecture

The SDK consists of several core layers:

1. AST Construction (`ast`):
   - Parses source code using Tree-sitter.
   - Builds an in-memory AST (`Program`, `NodeType`, etc.).
2. Codebase (`codebase`):
   - Manages a collection of source files and their ASTs.
   - Builds symbol tables and resolves imports and function calls.
3. Passes (`passes`):
   - Implements algorithms to build and merge symbol tables.
4. Storage (`storage`):
   - Stores all AST nodes in a flat structure with parent-child links.
5. Detector Framework:
   - Traits `Detector` and `DetectorReportTemplate` define the interface for writing detectors.
   - Combines check logic with reporting.

```text
Source files ➔ AST Builder ➔ NodesStorage ➔ Symbol Tables ➔ Codebase.Seal ➔ Detector.run
```

## Modules

This section provides a brief overview of the main modules in the SDK:

- **ast**: AST definitions and builder
  - `builder.rs`: Constructs AST from Tree-sitter parse tree.
  - `node.rs`, `node_type.rs`: Definitions of AST nodes.
  - Other submodules: `declarations`, `expressions`, `statements`...
- **codebase**: Core `Codebase` struct and APIs
  - `Codebase<OpenState>`: Add files and build AST.
  - `Codebase<SealedState>`: Seal codebase, build symbol tables, link imports and calls.
  - Public API: `build_codebase`.
- **passes**: Symbol table builder
  - `build_symbol_table`: Merges local and imported symbol tables.
- **storage**: `NodesStorage`
  - Flat storage of all AST nodes with parent-child relationships.
  - Used internally by `Codebase`.
- **root (lib.rs)**:
  - Public exports: `ast`, `codebase`, `build_codebase`, `Detector` traits, `DetectorResult`.
- **builder_tests.rs**: Internal tests for AST builder.
- **storage.rs**: Low-level storage implementation.

## Contributing

See [contributing.md](../contributing.md) for guidelines.

## Style Guidelines

See [style guidelines](../style_guidelines.md) for coding standards and best practices.

## License

[AGPLv3](../LICENSE)
