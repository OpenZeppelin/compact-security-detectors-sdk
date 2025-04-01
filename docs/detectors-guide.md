# Midnight Security detectors Developer Guide

This document serves as a guide for developers to create and maintain security detectors for the Midnight platform.

- [Midnight Security detectors Developer Guide](#midnight-security-detectors-developer-guide)
  - [Crates breakdown](#crates-breakdown)
  - [Writing a new detector example](#writing-a-new-detector-example)
    - [Compact code](#compact-code)
    - [Formulate the detector](#formulate-the-detector)
    - [Implementation](#implementation)
    - [Execution](#execution)
    - [Base and custom detectors](#base-and-custom-detectors)

## Crates breakdown

**sdk**: The [SDK](../sdk/) crate contains the core logic to build a Compact language model and exposes an API for developers to write secuirty detectors.
**detectors-runner**: The [detectors-runner](../detectors-runner/) crate is responsible for executing the security detectors.
**detectors**: The [detectors](../detectors/) crate contains the security detectors that are executed by the detectors-runner. It is a workspace that contains all the detectors and their dependencies.

## Writing a new detector example

### Compact code

```compact
export circuit set_admin(new_admin: Bytes<32>): [] {
    const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
    assert admin == pad(32, "") "";
    admin = new_admin;
    return [];
}
```

### Formulate the detector

If assert message is not provided or too short, we notify a user about a non informative assert message.

### Implementation

```rust
use std::{cell::RefCell, collections::HashMap};

use midnight_security_detectors_sdk::{
    ast::{definition::Definition, node_type::NodeType},
    codebase::{Codebase, SealedState},
    detector,
};

// The detector itself is defined as a struct, which implements the detector trait.
pub struct AssertionErrorMessageConsistency;

// The detector is implemented as a trait, which defines the detector's name, description, and check method.
impl detector for AssertionErrorMessageConsistency {

    // The detector's name exists for a better user nofification about which detector was violated.
    fn name(&self) -> String {
        "Assertion Error Message Consistency".to_string()
    }

    // The detector's description is used for the reporting purposes.
    fn description(&self) -> String {
        "Without a clear error message, debugging failures in this critical admin-setting function becomes difficult.".to_string()
    }

    // The logic of what and how the detector checks is implemented in the check method.
    fn check(
        &self,
        codebase: &RefCell<Codebase<SealedState>>, // The `codebase` is an instance of the Codebase struct, which is used to access the detectors SDK nodes.
    ) -> Option<HashMap<String, Vec<(usize, usize)>>> {
        let codebase = codebase.borrow();
        let mut errors = HashMap::new();
        for assert_node in codebase.list_assert_nodes() { // Since we are interested in assert nodes, we list them first.
            if assert_node.message().is_none() // Check if the assert node has a message or the message is long enough to be considered informative.
                || assert_node.message().unwrap().is_empty()
                || assert_node.message().unwrap().trim().is_empty()
                || assert_node.message().unwrap().len() < 3
            {
                let parent = codebase.get_parent_container(assert_node.id); // Find a parent to locate the problem accurately.
                let parent_name = match parent {
                    Some(NodeType::Definition(Definition::Circuit(c))) => c.name(), // For instance, if the parent is a circuit, we get its name.
                    Some(NodeType::Definition(Definition::Module(m))) => m.name(), // If the parent is a module, we get its name.
                    _ => String::new(),
                };
                errors.insert( // Report the error by incerting the finding instance into the errors map.
                    parent_name,
                    vec![(
                        assert_node.location.start_line,
                        assert_node.location.start_column,
                    )],
                );
            }
        }
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
}
```

The code is copied from the [lib.rs](../detectors/src/lib.rs) file.


### Execution

For detectors execution use `detectors-runner`.

```rust
use midnight_security_detectors::all_detectors;
use midnight_security_detectors_sdk::{build_codebase, detector};

...

let codebase = build_codebase(data).unwrap(); // The `data` is a HashMap of <source_file_path, source_file_content>.
let mut detectors = all_detectors();
...
for detector in detectors {
        let detector_result = detector.check(&codebase);
        if let Some(errors) = detector_result {
            for (container_name, locations) in errors.iter() {
                for (line, col) in locations.iter() {
                    println!(
                        "[{}]: in {container_name} detected an error at [{line}:{col}]",
                        detector.name()
                    );
                }
            }
        }
    }
```

As an execution result for the detector above, we will get the following output:

```
[Assertion Error Message Consistency]: in set_admin detected an error at [3:13]
```

P.S. there the stdout is used for the demonstration purposes.

### Base and custom detectors

There is an assumption that there will be a set of base detector so users can experiment and develop their own detectors.
It is possible since the `detector` itself is a trait. To "plug" a new detector a `detectors-runner` has a function:

```rust
#[allow(clippy::let_and_return, unused_mut)]
fn custom_detectors() -> Vec<Box<dyn detector>> {
    let mut detectors: Vec<Box<dyn detector>> = Vec::new();
    //Import and add your detectors here
    detectors
}
```

so before running the detectors execution, it does:

```rust
let mut detectors = all_detectors();
detectors.extend(custom_detectors());
for detector in detectors {
    ...
}
```

A user can implement a `detector` trait, compile it and add the compiled library to the `detectors-runner` dependencies. Then, the detector should be added to the `custom_detectors` function.