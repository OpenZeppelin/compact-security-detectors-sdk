use std::{cell::RefCell, collections::HashMap};

use midnight_security_detectors_sdk::{
    ast::{definition::Definition, expression::Expression, node_type::NodeType, ty::Type},
    codebase::{Codebase, SealedState},
    DetectorResult,
};

include!(concat!(env!("OUT_DIR"), "/detector-report-templates.rs"));
mod utils;

detectors! {
    #[type_name = AssertionErrorMessageConsistency]
    fn assertion_error_message_consistency(
        codebase: &RefCell<Codebase<SealedState>>,
    ) -> Option<Vec<DetectorResult>> {
        let codebase = codebase.borrow();
        let mut errors = Vec::new();
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
                errors.push(DetectorResult {
                    file_path: codebase.find_node_file(assert_node.id).unwrap().fname,
                    offset_start: assert_node.location.offset_start,
                    offset_end: assert_node.location.offset_end,
                    extra: {
                        let mut map = HashMap::new();
                        map.insert("ASSERTION_MESSAGE".to_string(), parent_name);
                        Some(map)
                    },
                });
            }
        }
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    #[type_name = ArrayLoopBoundCheck]
    fn array_loop_bound_check(
        codebase: &RefCell<Codebase<SealedState>>,
    ) -> Option<Vec<DetectorResult>> {
        let codebase = codebase.borrow();
        let mut errors = Vec::new();
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
                            errors.push(
                                DetectorResult {
                                    file_path: codebase.find_node_file(index_access.id).unwrap().fname,
                                    offset_start: index_access.location.offset_start,
                                    offset_end: index_access.location.offset_end,
                                    extra: {
                                        let mut map = HashMap::new();
                                        map.insert("ARRAY_INDEX_ACCESS".to_string(), parent_name);
                                        Some(map)
                                    },
                                },
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

#[cfg(test)]
mod tests {
    use midnight_security_detectors_sdk::{build_codebase, Detector};

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
        let codebase = build_codebase(&data).unwrap();
        let result = detector.check(&codebase);
        assert!(result.is_some());
        assert_eq!(result.as_ref().unwrap().len(), 1, "{result:?}");
        let detector_result = result.as_ref().unwrap().first().unwrap();
        assert_eq!(detector_result.file_path, "test.compact");
        assert_eq!(detector_result.offset_start, 153);
        assert_eq!(detector_result.offset_end, 184);
        assert_eq!(detector_result.extra, {
            let mut map = HashMap::new();
            map.insert("#PARENT_NAME".to_string(), "set_admin".to_string());
            Some(map)
        });
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
        let codebase = build_codebase(&data).unwrap();
        let result = detector.check(&codebase);
        assert!(result.is_some());
        assert_eq!(result.as_ref().unwrap().len(), 1, "{result:?}");
        let detector_result = result.as_ref().unwrap().first().unwrap();
        assert_eq!(detector_result.file_path, "test.compact");
        assert_eq!(detector_result.offset_start, 132);
        assert_eq!(detector_result.offset_end, 139);
        assert_eq!(detector_result.extra, {
            let mut map = HashMap::new();
            map.insert("#PARENT_NAME".to_string(), "contains".to_string());
            Some(map)
        });
    }
}
