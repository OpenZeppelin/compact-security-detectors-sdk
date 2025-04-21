 # Writing Detectors

 This guide covers how to implement new security detectors in the codebase.

 ## Detector Macros

 Two macros are provided in `src/utils.rs`:

 - `detector!`  
   Defines a single detector by implementing the `Detector` trait.

 - `detectors!`  
   Wraps multiple `detector!` invocations and generates an `all_detectors()` function returning all detectors.

 ## Defining a Detector

 Example:

 ```rust
 detector! {
     #[type_name = MyCustomDetector]
     pub fn my_custom_detector(
         codebase: &RefCell<Codebase<SealedState>>
     ) -> Option<Vec<DetectorResult>> {
         // Your detector logic here
     }
 }
 ```

 **Arguments:**
 - `#[type_name = ...]` specifies the struct name implementing the detector.
 - Function signature must match `Detector::check`.
 - Return `Some(Vec<DetectorResult>)` on findings, or `None` if no issues are found.

 ## Registering Detectors

 Use the `detectors!` macro in `src/lib.rs`:

 ```rust
 detectors! {
     #[type_name = CustomDetector1]
     fn custom_detector1(...) { /* ... */ }

     #[type_name = CustomDetector2]
     fn custom_detector2(...) { /* ... */ }
 }
 ```

 This expands to multiple `detector!` definitions and an `all_detectors()` factory.

 ## Metadata Configuration

 Detector-specific configuration can be added under the `metadata/` directory in YAML. See existing files in [metadata](../metadata) and reference in [Project Structure](project-structure.md#metadata-configuration).