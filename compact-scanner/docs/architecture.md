<!-- docs/architecture.md -->

# Architecture

> Overview of **compact-scanner** internal structure and dependencies.

- **src/main.rs**: Entry point; orchestrates scanning workflow.
- **src/parser.rs**: Defines CLI argument parsing with `clap`.
- **compact-security-detectors**: Provides built-in detectors via `all_detectors()`.
- **compact-security-detectors-sdk**: Supplies `build_codebase` and scanning APIs.

## Project Structure
```text
compact-scanner/
├── Cargo.toml         # crate manifest
├── src/
│   ├── main.rs        # CLI entrypoint and scan orchestration
│   └── parser.rs      # Command-line argument definitions
└── docs/              # Developer documentation (this directory)
```

## Dependencies
- **clap**: CLI parsing
- **serde_json**, **serde_yaml**: JSON/YAML serialization
- **compact-security-detectors**, **compact-security-detectors-sdk**: Static analysis engine

See [Parser Module](parser_module.md) and [Detectors Integration](detectors_integration.md) for more details.