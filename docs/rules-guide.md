# Midnight Security Rules Developer Guide

This document serves as a guide for developers to create and maintain security rules for the Midnight platform.

- [Midnight Security Rules Developer Guide](#midnight-security-rules-developer-guide)
  - [Crates breakdown](#crates-breakdown)
  - [Writing a new rule example](#writing-a-new-rule-example)
    - [Compact code](#compact-code)
    - [Formulate the rule](#formulate-the-rule)
    - [Implementation](#implementation)
    - [Execution](#execution)
    - [Base and custom rules](#base-and-custom-rules)

## Crates breakdown

**sdk**: The [SDK](../sdk/) crate contains the core logic to build a Compact language model and exposes an API for developers to write secuirty rules.
**rules-runner**: The [rules-runner](../rules-runner/) crate is responsible for executing the security rules.
**rules**: The [rules](../rules/) crate contains the security rules that are executed by the rules-runner. It is a workspace that contains all the rules and their dependencies.

## Writing a new rule example

### Compact code

```compact
export circuit set_admin(new_admin: Bytes<32>): [] {
    const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
    assert admin == pad(32, "") "";
    admin = new_admin;
    return [];
}
```

### Formulate the rule

If assert message is not provided or too short, we notify a user about a non informative assert message.

### Implementation

```rust
use std::{cell::RefCell, collections::HashMap};

use midnight_security_rules_sdk::{
    ast::{definition::Definition, node_type::NodeType},
    codebase::{Codebase, SealedState},
    Rule,
};

// The rule itself is defined as a struct, which implements the Rule trait.
pub struct AssertionErrorMessageConsistency;

// The rule is implemented as a trait, which defines the rule's name, description, and check method.
impl Rule for AssertionErrorMessageConsistency {

    // The rule's name exists for a better user nofification about which rule was violated.
    fn name(&self) -> String {
        "Assertion Error Message Consistency".to_string()
    }

    // The rule's description is used for the reporting purposes.
    fn description(&self) -> String {
        "Without a clear error message, debugging failures in this critical admin-setting function becomes difficult.".to_string()
    }

    // The logic of what and how the rule checks is implemented in the check method.
    fn check(
        &self,
        codebase: &RefCell<Codebase<SealedState>>, // The `codebase` is an instance of the Codebase struct, which is used to access the rules SDK nodes.
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

The code is copied from the [lib.rs](../rules/src/lib.rs) file.


### Execution

For rules execution use `rules-runner`.

```rust
use midnight_security_rules::all_rules;
use midnight_security_rules_sdk::{build_codebase, Rule};

...

let codebase = build_codebase(data).unwrap(); // The `data` is a HashMap of <source_file_path, source_file_content>.
let mut rules = all_rules();
...
for rule in rules {
        let rule_result = rule.check(&codebase);
        if let Some(errors) = rule_result {
            for (container_name, locations) in errors.iter() {
                for (line, col) in locations.iter() {
                    println!(
                        "[{}]: in {container_name} detected an error at [{line}:{col}]",
                        rule.name()
                    );
                }
            }
        }
    }
```

As an execution result for the rule above, we will get the following output:

```
[Assertion Error Message Consistency]: in set_admin detected an error at [3:13]
```

P.S. there the stdout is used for the demonstration purposes.

### Base and custom rules

There is an assumption that there will be a set of base rule so users can experiment and develop their own rules.
It is possible since the `Rule` itself is a trait. To "plug" a new rule a `rules-runner` has a function:

```rust
#[allow(clippy::let_and_return, unused_mut)]
fn custom_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    //Import and add your rules here
    rules
}
```

so before running the rules execution, it does:

```rust
let mut rules = all_rules();
rules.extend(custom_rules());
for rule in rulse {
    ...
}
```

A user can implement a `Rule` trait, compile it and add the compiled library to the `rules-runner` dependencies. Then, the rule should be added to the `custom_rules` function.