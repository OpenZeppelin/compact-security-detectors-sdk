# Midnight Security detectors Developer Guide

This document serves as a guide for developers to create and maintain security detectors for the Midnight platform.

- [Midnight Security detectors Developer Guide](#midnight-security-detectors-developer-guide)
  - [Crates breakdown](#crates-breakdown)
  - [Writing a new detector example](#writing-a-new-detector-example)
    - [AssertionErrorMessageConsistency](#assertionerrormessageconsistency)
      - [Compact code](#compact-code)
      - [Formulate the detector](#formulate-the-detector)
      - [Implementation](#implementation)
      - [Execution](#execution)
    - [ArrayLoopBoundCheck](#arrayloopboundcheck)
      - [Compact code](#compact-code-1)
      - [Formulate the detector](#formulate-the-detector-1)
      - [Implementation](#implementation-1)
    - [Base and Custom Detectors](#base-and-custom-detectors)

## Crates breakdown

* **sdk**: The [SDK](../sdk/) crate contains the core logic to build a Compact language model and exposes an API for developers to write secuirty detectors.

* **detectors-runner**: The [detectors-runner](../detectors-runner/) crate is responsible for executing the security detectors.

* **detectors**: The [detectors](../detectors/) crate contains the security detectors that are executed by the detectors-runner. It is a workspace that contains all the detectors and their dependencies.

## Writing a new detector example

### AssertionErrorMessageConsistency 

#### Compact code

```compact
export circuit set_admin(new_admin: Bytes<32>): [] {
    const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
    assert admin == pad(32, "") "";
    admin = new_admin;
    return [];
}
```

#### Formulate the detector

If an assert message is not provided or is too short, the detector notifies the user about an uninformative assert message.

#### Implementation

```rust
use std::{cell::RefCell, collections::HashMap};

use midnight_security_detectors_sdk::{
    ast::{definition::Definition, node_type::NodeType},
    codebase::{Codebase, SealedState},
    detector,
};

pub struct AssertionErrorMessageConsistency;

impl detector for AssertionErrorMessageConsistency {
    fn name(&self) -> String {
        "Assertion Error Message Consistency".to_string()
    }

    fn description(&self) -> String {
        "Without a clear error message, debugging failures in this critical admin-setting function becomes difficult.".to_string()
    }

    fn check(
        &self,
        codebase: &RefCell<Codebase<SealedState>>,
    ) -> Option<HashMap<String, Vec<(usize, usize)>>> {
        let codebase = codebase.borrow();
        let mut errors = HashMap::new();
        for assert_node in codebase.list_assert_nodes() {
            if assert_node
                .message()
                .map(|msg| msg.trim().is_empty() || msg.len() < 3)
                .unwrap_or(true)
            {
                let parent = codebase.get_parent_container(assert_node.id);
                let parent_name = match parent {
                    Some(NodeType::Definition(Definition::Circuit(c))) => c.name(),
                    Some(NodeType::Definition(Definition::Module(m))) => m.name(),
                    _ => String::new(),
                };
                errors.insert(
                    parent_name,
                    vec![
                        (
                            assert_node.location.start_line,
                            assert_node.location.start_column,
                        )
                    ],
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

#### Execution

For detector execution, use `detectors-runner`.

```rust
use midnight_security_detectors::all_detectors;
use midnight_security_detectors_sdk::{build_codebase, detector};

let codebase = build_codebase(data).unwrap();
let mut detectors = all_detectors();
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

### ArrayLoopBoundCheck

#### Compact code

```compact
export circuit contains(arr: Vector<10, Address>, addr: Address): Bool {
    for (const i of 0 .. 10) {
        if (arr[11] == addr) {
            return true;
        }
    }
    return false;
}
```

#### Formulate the detector

The detector checks if the loop is out of bounds. If the loop is out of bounds, it notifies the user.

#### Implementation

```rust
pub struct ArrayLoopBoundCheck;

impl Detector for ArrayLoopBoundCheck {
    fn name(&self) -> String {
        "Array Loop Bound Check".to_string()
    }

    fn description(&self) -> String {
        "This detector checks for potential out-of-bounds access in array loops.".to_string()
    }

    fn check(
        &self,
        codebase: &RefCell<Codebase<SealedState>>,
    ) -> Option<HashMap<String, Vec<(u32, u32)>>> {
        let codebase = codebase.borrow();
        let mut errors = HashMap::new();
        for for_stmt in codebase.list_for_statement_nodes() {
            let index_access_expressions = codebase.get_children_cmp(for_stmt.id, |n| {
                matches!(n, NodeType::Expression(Expression::IndexAccess(_)))
            });
            let upper_bound = for_stmt.upper_bound_nat();
            if upper_bound.is_none() {
                continue;
            }
            let upper_bound = upper_bound.unwrap();

            for index_access in index_access_expressions {
                if let NodeType::Expression(Expression::IndexAccess(index_access)) = index_access {
                    let arr_type =
                        codebase.get_symbol_type_by_id(index_access.base.id());
                    if let Some(Type::Vector(t_vec)) = arr_type {
                        if t_vec.size_nat().unwrap_or(0) >= upper_bound {
                            let parent = codebase.get_parent_container(index_access.id);
                            let parent_name = match parent {
                                Some(NodeType::Definition(Definition::Circuit(c))) => c.name(),
                                Some(NodeType::Definition(Definition::Module(m))) => m.name(),
                                _ => String::new(),
                            };
                            errors.insert(
                                parent_name,
                                vec![(
                                    index_access.location.start_line,
                                    index_access.location.start_column,
                                )],
                            );
                        }
                    }
                }
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

### Base and Custom Detectors

The `detector` trait allows users to create custom detectors. To integrate a new detector, add it to the `custom_detectors` function in `detectors-runner`:

```rust
#[allow(clippy::let_and_return, unused_mut)]
fn custom_detectors() -> Vec<Box<dyn detector>> {
    let mut detectors: Vec<Box<dyn detector>> = Vec::new();
    // Import and add your detectors here
    detectors
}
```

Before running the detectors, the `detectors-runner` combines all detectors:

```rust
let mut detectors = all_detectors();
detectors.extend(custom_detectors());
for detector in detectors {
    // Execution logic
}
```