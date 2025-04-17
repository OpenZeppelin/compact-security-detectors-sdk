<!-- docs/testing.md -->
[← Back to Index](index.md)

# Testing

## Run Tests

```bash
cargo test
```

Tests are located in:

- `builder_tests.rs` (ignored in CI by default)
- Unit tests within modules.

## Linting

Enforce coding standards:

```bash
cargo clippy --all-targets -- -D warnings
```

## CI Integration

- Configure GitHub Actions or similar to run tests and clippy on push.

[Previous: Writing Detectors](detectors.md)  
[Next: Contributing](contributing.md)