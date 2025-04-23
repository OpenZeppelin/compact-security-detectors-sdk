<!-- docs/cli_usage.md -->

# CLI Usage

> Description of command-line interface options and flags.

## Modes

- `metadata` : Print scanner metadata as JSON and exit.
- `scan <PATH>` : Scan files in the specified directory or file. The scanner will recursively scan all files in the directory and its subdirectories.

## Options

- `--detectors <NAME>...` : Optional list of detector names to run. If omitted, all available detectors will run. You can use `all` to run all detectors.
- `--project-root <PATH>` : Optional project root path to calculate relative file paths in output.

## Examples

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

For output format details, see [JSON Output Format](json_output_format.md).
