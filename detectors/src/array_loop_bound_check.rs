use std::collections::HashMap;

use compact_security_detectors_sdk::{
    ast::{
        declaration::Declaration, definition::Definition, expression::Expression,
        node_type::NodeType, ty::Type,
    },
    codebase::{Codebase, SealedState},
    detector::DetectorResult,
};

compact_security_detectors_sdk::detector! {

    #[type_name = ArrayLoopBoundCheck]
    fn array_loop_bound_check(
        codebase: &Codebase<SealedState>,
    ) -> Option<Vec<DetectorResult>> {
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
                            let mut parent_type = "circuit";
                            let parent_name = match parent {
                                Some(NodeType::Definition(Definition::Circuit(c))) => c.name(),
                                Some(NodeType::Declaration(Declaration::Constructor(_))) => {
                                    parent_type = "constructor";
                                    String::default()
                                }
                                _ => String::from("Unknown"),
                            };
                            errors.push(
                                DetectorResult {
                                    file_path: codebase.find_node_file(index_access.id).unwrap().file_path,
                                    offset_start: index_access.location.offset_start,
                                    offset_end: index_access.location.offset_end,
                                    extra: {
                                        let mut map = HashMap::new();
                                        map.insert("ARRAY_INDEX_ACCESS".to_string(), index_access.location.source.clone());
                                        map.insert("PARENT_NAME".to_string(), parent_name);
                                        map.insert("PARENT_TYPE".to_string(), parent_type.to_string());
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
    use super::*;
    use compact_security_detectors_sdk::build_codebase;
    use std::collections::HashMap;

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
        let result = detector.check(codebase.as_ref());
        assert!(result.is_some());
        assert_eq!(result.as_ref().unwrap().len(), 1, "{result:?}");
        let detector_result = result.as_ref().unwrap().first().unwrap();
        assert_eq!(detector_result.file_path, "test.compact");
        assert_eq!(detector_result.offset_start, 132);
        assert_eq!(detector_result.offset_end, 139);
        assert_eq!(detector_result.extra, {
            let mut map = HashMap::new();
            map.insert("ARRAY_INDEX_ACCESS".to_string(), "arr[11]".to_string());
            map.insert("PARENT_NAME".to_string(), "contains".to_string());
            map.insert("PARENT_TYPE".to_string(), "circuit".to_string());
            Some(map)
        });
    }
}
