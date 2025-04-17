 # Getting Started

 This guide will help you set up your development environment, build the project, and run tests.

 ## Prerequisites

 - Rust (1.60 or later)
 - Cargo (comes with Rust)
 - A Unix-like shell (Linux, macOS, or WSL on Windows)

 ## Cloning the Repository

 ```bash
 git clone <repository-url>
 cd <repository-name>
 ```

 ## Building the Project

 Run:

 ```bash
 cargo build
 ```

 This will compile the library and generate detector report templates via the `build.rs` script.

 ## Running Tests

 To run the full test suite:

 ```bash
 cargo test
 ```

 ## High-Level Workflow

 1. Define new detectors in `src/lib.rs` (or separate modules).
 2. Use the `detector!` and `detectors!` macros from `src/utils.rs`.
 3. Add tests in the `#[cfg(test)]` section of `src/lib.rs` or in a `tests/` directory.
 4. Update `metadata/` if your detectors require new configuration.