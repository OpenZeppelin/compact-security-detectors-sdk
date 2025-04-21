<!-- docs/detectors_integration.md -->

# Detectors Integration

> How **compact-scanner** discovers and executes security detectors.

Located in `src/main.rs`, the function:
```rust
fn available_detectors() -> Vec<MidnightDetector> {
    all_detectors()
        .into_iter()
        .chain(custom_detectors())
        .collect()
}
```

## Built-in vs. Custom Detectors
- **Built-in**: Provided by `midnight-security-detectors::all_detectors()`.
- **Custom**: Placeholder via `custom_detectors()` (currently returns empty).

## Selecting Detectors
- Omit `--detectors`: run all discovered detectors.
- Provide `--detectors NAME...`: filter by detector `name()` property.

## Execution Flow
1. Build the in-memory codebase: `build_codebase(files)` from the SDK.
2. Iterate over selected detectors and run `detector.check(&codebase)`.
3. Collect `DetectorResult` for detectors that return findings.

Results are formatted into JSON. See [JSON Output Format](json_output_format.md).