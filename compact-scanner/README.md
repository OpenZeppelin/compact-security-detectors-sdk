# Compact Scanner

CLI tool for scanning `.compact` files using Compact security detectors.

## Overview

The **compact-scanner** is a command-line tool designed to scan `.compact` files for security vulnerabilities using the
Compact security detectors. It provides a simple interface to run various detectors and output results in JSON format.

## CLI Usage

> Description of command-line interface options and flags.

### Modes

- `metadata` : Print scanner metadata as JSON and exit.
- `scan <PATH>` : Scan files in the specified directory or file. The scanner will recursively scan all files in the
  directory and its subdirectories.

### Options

- `--detectors <NAME>...` : Optional list of detector names to run. If omitted, all available detectors will run. You
  can use `all` to run all detectors.
- `--project-root <PATH>` : Optional project root path to calculate relative file paths in output.

### Examples

```bash
# Print metadata
compact-scanner metadata

# Scan files in a directory
compact-scanner scan src/compact_files

# Run specific detectors
compact-scanner scan src/file.compact --detectors DetectorA DetectorB

# Specify project root for relative paths
compact-scanner scan src --project-root .
```

## Detectors Integration

> How **compact-scanner** discovers and executes security detectors.

Located in `src/main.rs`, the function:

```rust
fn available_detectors() -> Vec<CompactDetector> {
    all_detectors()
        .into_iter()
        .chain(custom_detectors())
        .collect()
}
```

### Built-in vs. Custom Detectors

- **Built-in**: Provided by `compact-security-detectors::all_detectors()`.
- **Custom**: Placeholder via `custom_detectors()` (currently returns empty).

### Selecting Detectors

- Omit `--detectors`: run all discovered detectors.
- Provide `--detectors NAME...`: filter by detector `name()` property.

### Execution Flow

1. Build the in-memory codebase: `build_codebase(files)` from the SDK.
2. Iterate over selected detectors and run `detector.check(&codebase)`.
3. Collect `DetectorResult` for detectors that return findings.

## JSON Output Format

> Format of JSON produced by **compact-scanner**.

### Metadata Output

When invoked with `metadata`, the JSON includes:

```json
{
  "name": "compact_scanner",
  "description": "Static analyzer for Midnight network Compact source code files",
  "version": "<version>",
  "org": "OpenZeppelin",
  "detectors": [
    {
      "id": "DetectorName",
      "description": "Detector Description",
      "report": {
        "severity": "<severity>",
        "tags": [
          "tag1",
          "tag2"
        ],
        "template": {
          /* YAML converted to JSON */
        }
      }
    }
  ]
}
```

### Scan Results Output

When scanning code files, the JSON structure is:

```json
{
  "errors": [],
  "files_scanned": [
    "relative/path1.compact",
    "path2.compact"
  ],
  "detector_responses": {
    "DetectorName": {
      "finding": {
        "instances": [
          {
            "file_path": "path/to/file.compact",
            "offset_start": 123,
            "offset_end": 456,
            "suggested_fixes": [],
            "extras": {}
          }
        ]
      },
      "errors": [],
      "metadata": {}
    }
  }
}
```

- `errors`: Scanner-level errors (empty on success).
- `files_scanned`: Array of scanned file paths relative to `--project-root` if provided.
- `detector_responses`: Map of detector IDs to their individual output.

## Contributing

See [contributing.md](../contributing.md) for guidelines.

## Style Guidelines

See [style guidelines](../style_guidelines.md) for coding standards and best practices.

## License

[AGPLv3](../LICENSE)

## Experimental and Risk-Prone Feature

> This functionality is intended for controlled testing environments only. It has not undergone extensive validation,
> and its behavior may be unpredictable. Please ensure that you fully understand the potential risks and implications
> before using it in any production scenario.


For those who do not want to re-compile the `scanner` or `detectors` crates, dynamic library loading is available
with the `--load` flag.

Usage:

```bash
compact-scanner scan <PATH> --load <PATH_TO_DETECTOR_LIBRARY>
```

This will load the dynamic library at runtime. The library must contain the `CompactDetector` trait implementation and
be compiled with the same Rust and `sdk` versions as the scanner.
External detector must export the "external_detector" symbol.
External detector must implement `DetectorReportTemplate` trait.

See the example [external detector](../examples/external-detector/src/lib.rs) for more details.
