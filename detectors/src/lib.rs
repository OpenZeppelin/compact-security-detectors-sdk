use std::{cell::RefCell, collections::HashMap};

use midnight_security_detectors_sdk::{
    ast::{definition::Definition, expression::Expression, node_type::NodeType, ty::Type},
    codebase::{Codebase, SealedState},
    Detector,
};

pub struct AssertionErrorMessageConsistency;

impl Detector for AssertionErrorMessageConsistency {
    fn name(&self) -> String {
        "Assertion Error Message Consistency".to_string()
    }

    fn description(&self) -> String {
        "Without a clear error message, debugging failures in this critical admin-setting function becomes difficult.".to_string()
    }

    fn check(
        &self,
        codebase: &RefCell<Codebase<SealedState>>,
    ) -> Option<HashMap<String, Vec<(u32, u32)>>> {
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
                    let arr_type = codebase.get_symbol_type_by_id(index_access.base.id());
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

pub fn all_detectors() -> Vec<Box<dyn Detector>> {
    vec![Box::new(AssertionErrorMessageConsistency)]
}

#[cfg(test)]
mod tests {
    use midnight_security_detectors_sdk::build_codebase;

    use super::*;

    #[test]
    fn test_all_detectors() {
        let detector = AssertionErrorMessageConsistency;
        let src = "export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            assert admin == pad(32, \"\") \"\";
            admin = new_admin;
            return [];
        }";
        let mut data = HashMap::new();
        data.insert("test.compact".to_string(), src.to_string());
        let codebase = build_codebase(data).unwrap();
        let result = detector.check(&codebase);
        assert!(result.is_some());
        assert!(result.unwrap().contains_key("set_admin"));
    }

    #[test]
    fn test_array_loop_bound_check() {
        let detector = ArrayLoopBoundCheck;
        let src = "export circuit contains(arr: Vector<10, Address>, addr: Address): Bool {
            for (const i of 0 .. 10) {
                if (arr[11] == addr) {
                    return true;
                }
            }
            return false;
        }";
        let mut data = HashMap::new();
        data.insert("test.compact".to_string(), src.to_string());
        let codebase = build_codebase(data).unwrap();
        let result = detector.check(&codebase);
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
    }
}

// const sk: Uint8Array = yesIKnowTheSecurityImplicationsOfThis_encryptionSecretKey().yesIKnowTheSecurityImplicationsOfThis_serialize(0);
// const h: Value = persistentHash(0, val);
// return disclose(proof);
