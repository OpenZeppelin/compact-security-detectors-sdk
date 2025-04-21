<!-- docs/json_output_format.md -->

# JSON Output Format

> Format of JSON produced by **compact-scanner**.

## Metadata Output

When invoked with `--metadata`, the JSON includes:
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
        "tags": ["tag1", "tag2"],
        "template": { /* YAML converted to JSON */ }
      }
    }
  ]
}
```

## Scan Results Output

When scanning code files, the JSON structure is:
```json
{
  "errors": [],
  "files_scanned": ["relative/path1.compact", "path2.compact"],
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

For details on flags and options, see [CLI Usage](cli_usage.md).