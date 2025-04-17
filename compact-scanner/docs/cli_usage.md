<!-- docs/cli_usage.md -->

# CLI Usage

> Description of command-line interface options and flags.

## Flags

- `-m`, `--metadata` : Print scanner metadata as JSON and exit.
- `-r`, `--recursive` : (Currently unused) Intended for recursive directory scanning.

## Options

- `--code <PATH>...` : One or more file or directory paths to scan. Only files with `.compact` extension are processed.
- `--detectors <NAME>...` : Optional list of detector names to run. If omitted, all available detectors will run.
- `--project-root <PATH>` : Optional project root path to calculate relative file paths in output.

## Examples

```bash
# Print metadata
compact-scanner --metadata

# Scan files in a directory
compact-scanner --code src/compact_files

# Run specific detectors
compact-scanner --code src/file.compact --detectors DetectorA DetectorB

# Specify project root for relative paths
compact-scanner --code src --project-root .
```

For output format details, see [JSON Output Format](json_output_format.md).