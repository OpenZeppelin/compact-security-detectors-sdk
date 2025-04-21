 # midnight-security-detectors-sdk

 Core SDK for building and running security detectors on `Compact` language circuits.

 **Version:** 0.0.1

 ## Overview

 Provides:
 - AST and codebase builder for Compact.
 - `Detector` trait and helper macros for writing detectors.
 - Utilities for integrating detectors and reporting results.

 ## Documentation

 Developer guide: `docs/index.md`

 ## Usage

 Add to your `Cargo.toml`:
 ```toml
 midnight-security-detectors-sdk = { path = "../sdk", version = "0.0.1" }
 ```

 Example:
 ```rust
 use midnight_security_detectors_sdk::{build_codebase, detector};
 let codebase = build_codebase(data)?;
 ```

 ## Testing

 Run tests:
 ```sh
 cargo test -p sdk
 ```