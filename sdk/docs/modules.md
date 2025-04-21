<!-- docs/modules.md -->
[← Back to Index](index.md)

# Modules

This section provides a brief overview of the main modules in the SDK:

- **ast**: AST definitions and builder
  - `builder.rs`: Constructs AST from Tree-sitter parse tree.
  - `node.rs`, `node_type.rs`: Definitions of AST nodes.
  - Other submodules: declarations, expressions, statements.
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

[Previous: Architecture](architecture.md)  
[Next: Writing Detectors](detectors.md)