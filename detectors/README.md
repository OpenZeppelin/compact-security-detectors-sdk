# Compact Security Detectors

Compact security detectors library for the Midnight platform.

## Overview

This crate provides a set of built-in security detectors for the `Compact` language, enabling analysis for vulnerabilities.

### High-Level Workflow

1. Define new detectors in `src/`.
2. Use the `detector!` and `detectors!` macros from `compact_security_detectors_sdk`.
3. Add tests in the `#[cfg(test)]` section.
4. Update `metadata/` if your detectors require new configuration.

## Project Structure

- **build.rs**
Build script that generates Rust code for detector report templates. The generated file is included in `src/lib.rs` via `include!`.
- **metadata/**
YAML configuration files for detectors.
- **src/**
  - `lib.rs` Main library file containing detector definitions using the `detectors!` macro.
  - `detector-report-templates.rs` (generated) Generated code with templates for detector reports, included via `include!` in `lib.rs`.
 
## Usage

Example:
```rust
use compact_security_detectors::all_detectors;
// build codebase with sdk...
let detectors = all_detectors();
for det in detectors {
    // run det.check(&codebase)
}
```

## Writing Detectors

This guide covers how to implement new security detectors in the codebase.

## Detector Macros

Two macros are provided by `compact_security_detectors_sdk` crate:

- `detector!`  Defines a single detector by implementing the `Detector` trait.
- `detectors!` Wraps multiple `detector!` invocations.

## Defining a Detector

Example:

```rust
detector! {
    #[type_name = MyCustomDetector]
    pub fn my_custom_detector(
        codebase: &Codebase<SealedState>
    ) -> Option<Vec<DetectorResult>> {
        // Your detector logic here
    }
}
```

**Arguments:**
- `#[type_name = ...]` specifies the struct name implementing the detector.
- Function signature must match `Detector::check`.
- Return `Some(Vec<DetectorResult>)` on findings, or `None` if no issues are found.

### Registering Detectors

```rust
compact_security_detectors_sdk::detectors! {
    #[type_name = CustomDetector1]
    fn custom_detector1(...) { /* ... */ }

    #[type_name = CustomDetector2]
    fn custom_detector2(...) { /* ... */ }
}
```

This expands to multiple `detector!` definitions and include to the `all_detectors()` factory.

### Metadata Configuration

Detector-specific configuration can be added under the `metadata/` directory in YAML.

## Unit Tests

Tests are defined inline in detectors under `#[cfg(test)] mod tests`.

Example test:

```rust
#[test]
fn test_my_detector() {
    let detector = MyCustomDetector;
    let src_code = "export circuit example(): [] { ... }";
    let mut data = HashMap::new();
    data.insert("example.compact".to_string(), src_code.to_string());
    let codebase = build_codebase(&data).unwrap();
    let result = detector.check(codebase.as_ref());
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 1);
}
```

## Contributing

See [contributing.md](../contributing.md) for guidelines.

## Style Guidelines

See [style guidelines](../style_guidelines.md) for coding standards and best practices.

## License

[AGPLv3](../LICENSE)
