<!-- docs/detectors.md -->
[← Back to Index](index.md)

# Writing Detectors

Custom detectors implement two traits:

1. `Detector`:
   - Defines the `check(&self, codebase: &RefCell<Codebase<SealedState>>) -> Option<Vec<DetectorResult>>` method.
   - Analysis logic goes here.

2. `DetectorReportTemplate`:
   - Provides metadata: name, description, severity, tags.
   - Template methods for report formatting.

## Example

```rust
use compact_security_detectors_sdk::{Detector, DetectorReportTemplate, DetectorResult};
use std::cell::RefCell;

pub struct ExampleDetector;

impl Detector for ExampleDetector {
    fn check(&self, codebase: &RefCell<_>) -> Option<Vec<DetectorResult>> {
        // analysis logic...
        None
    }
}

impl DetectorReportTemplate for ExampleDetector {
    fn name(&self) -> String { "ExampleDetector".into() }
    fn description(&self) -> String { "An example detector".into() }
    fn severity(&self) -> String { "LOW".into() }
    fn tags(&self) -> Vec<String> { vec!["example".into()] }
    fn title_single_instance(&self) -> String { "Found issue".into() }
    fn title_multiple_instance(&self) -> String { "Found multiple issues".into() }
    fn opening(&self) -> String { "Opening message".into() }
    fn body_single_file_single_instance(&self) -> String { "Detailed message".into() }
    fn body_single_file_multiple_instance(&self) -> String { "Detailed multiple message".into() }
    fn body_multiple_file_multiple_instance(&self) -> String { "Multi-file message".into() }
    fn body_list_item_single_file(&self) -> String { "- List item".into() }
    fn body_list_item_multiple_file(&self) -> String { "- Multi-file list item".into() }
    fn closing(&self) -> String { "Closing message".into() }
    fn template(&self) -> String { "{{ name }}".into() }
}

let detectors: Vec<CompactDetector> = vec![
    Box::new(ExampleDetector),
    // Add more detectors here
];

[Previous: Modules](modules.md)  
[Next: Testing](testing.md)