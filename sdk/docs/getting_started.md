<!-- docs/getting_started.md -->
[← Back to Index](index.md)

# Getting Started

This section walks through a minimal example to build a codebase and run a detector.

## Build a Codebase

```rust
use midnight_security_detectors_sdk::{build_codebase, DetectorResult};

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
for detector in detectors.iter() {
    if let Some(results) = detector.check(&codebase) {
        for result in results {
            println!("{:?}", result);
        }
    }
}
```

[Previous: Installation](installation.md)  
[Next: Architecture](architecture.md)