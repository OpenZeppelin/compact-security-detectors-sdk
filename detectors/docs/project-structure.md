 # Project Structure

 This section describes the layout of the repository and key files.

 - **Cargo.toml**  
   Project metadata, dependencies, and build settings.

 - **build.rs**  
   Build script that generates Rust code for detector report templates. The generated file is included in `src/lib.rs` via `include!`.

 - **metadata/**  
   YAML configuration files for detectors. See [Writing Detectors](writing-detectors.md#metadata-configuration) for details.

 - **src/**  
   - `lib.rs`  
     Main library file containing detector definitions using the `detectors!` macro.  
   - `utils.rs`  
     Contains the `detector!` and `detectors!` macros for defining and registering detectors.  
   - `detector-report-templates.rs` (generated)  
     Generated code with templates for detector reports, included via `include!` in `lib.rs`.

 - **docs/**  
   Developer documentation (this directory).

 - **target/**  
   Build output directory (created after running `cargo build`).

 Refer to [Writing Detectors](writing-detectors.md) for how to add new security detectors, and [Testing](testing.md) for details on writing and running tests.