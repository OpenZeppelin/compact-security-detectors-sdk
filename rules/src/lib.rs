use std::{cell::RefCell, collections::HashMap};

use midnight_security_rules_sdk::{
    ast::{definition::Definition, node_type::NodeType},
    codebase::{Codebase, SealedState},
    Rule,
};

pub struct AssertionErrorMessageConsistency;

impl Rule for AssertionErrorMessageConsistency {
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
            if assert_node.message().is_none()
                || assert_node.message().unwrap().is_empty()
                || assert_node.message().unwrap().trim().is_empty()
                || assert_node.message().unwrap().len() < 3
            {
                let parent = codebase.get_parent_container(assert_node.id);
                let parent_name = match parent {
                    Some(NodeType::Definition(Definition::Circuit(c))) => c.name(),
                    Some(NodeType::Definition(Definition::Module(m))) => m.name(),
                    _ => String::new(),
                };
                errors.insert(
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

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(AssertionErrorMessageConsistency)]
}

#[cfg(test)]
mod tests {
    use midnight_security_rules_sdk::build_codebase;

    use super::*;

    #[test]
    fn test_all_rules() {
        let rule = AssertionErrorMessageConsistency;
        let src = "export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            assert admin == pad(32, \"\") || admin == current_proof;
            admin = new_admin;
            return [];
        }";
        let mut data = HashMap::new();
        data.insert("test.compact".to_string(), src.to_string());
        let codebase = build_codebase(data).unwrap();
        let result = rule.check(&codebase);
        assert!(result.is_some());
        assert!(result.unwrap().contains_key("set_admin"));
    }
}
