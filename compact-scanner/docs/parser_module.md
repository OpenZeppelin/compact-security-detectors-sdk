<!-- docs/parser_module.md -->

# Parser Module

This module defines the command-line interface for **compact-scanner** using `clap`.

Located at `src/parser.rs`:
```rust
use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short = 'm', long = "metadata", action = clap::ArgAction::SetTrue)]
    pub(crate) metadata: bool,

    #[clap(long = "code", required = false, value_parser, num_args = 1..)]
    pub(crate) code: Option<Vec<std::path::PathBuf>>,

    #[clap(long = "detectors", required = false, value_parser, num_args = 1..)]
    pub(crate) detectors: Option<Vec<String>>,

    #[clap(long = "project-root", required = false, value_parser)]
    pub(crate) project_root: Option<std::path::PathBuf>,

    #[clap(short = 'r', long = "recursive", action = clap::ArgAction::SetTrue)]
    pub(crate) recursive: bool,
}
```

Key fields:
- `metadata`: Flag to output scanner metadata and exit.
- `code`: List of input file or directory paths (`.compact` files).
- `detectors`: Optional filter for detector names.
- `project_root`: Base path for relative file paths in output.
- `recursive`: Currently unused; directory scanning is recursive by default.

Parsed arguments are applied in `src/main.rs` to control scanning behavior. See [CLI Usage](cli_usage.md).