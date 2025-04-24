use compact_security_detectors_sdk::{
    ast::{
        declaration::Declaration, definition::Definition, expression::Expression,
        node_type::NodeType, ty::Type,
    },
    codebase::{Codebase, SealedState},
    detector::{CompactDetector, DetectorOpaque, DetectorReportTemplate, DetectorResult},
};
use std::collections::HashMap;

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
                                    file_path: codebase.find_node_file(index_access.id).unwrap().fname,
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

impl DetectorReportTemplate for ArrayLoopBoundCheck {
    fn id(&self) -> String {
        String::from("array-loop-bound-check")
    }
    fn uid(&self) -> String {
        String::from("3fTuAe")
    }
    fn description(&self) -> String {
        String::from("Detects potential out-of-bounds array index accesses within loops, which can cause runtime errors or unexpected behavior.")
    }
    fn severity(&self) -> String {
        String::from("medium")
    }
    fn tags(&self) -> Vec<String> {
        vec![
            String::from("audit"),
            String::from("reportable"),
            String::from("compact"),
        ]
    }
    fn title_single_instance(&self) -> String {
        String::from("Potential Out-of-Bounds Array Index Access Detected")
    }
    fn title_multiple_instance(&self) -> String {
        String::from(
            "The following potential out-of-bounds array index accesses were found in the code:",
        )
    }
    fn opening(&self) -> String {
        String::from("Accessing array elements outside their valid index range can lead to runtime errors, security vulnerabilities, or unpredictable program behavior. This issue typically occurs when a loop iterates beyond the array's defined bounds, resulting in unsafe memory access.")
    }
    fn body_single_file_single_instance(&self) -> String {
        String::from("In `$file_name`, a potential out-of-bounds array index access was detected in the `$PARENT_NAME` $PARENT_TYPE on line $instance_line. The array access statement `$ARRAY_INDEX_ACCESS` may exceed the array's valid index range.")
    }
    fn body_single_file_multiple_instance(&self) -> String {
        String::from("In `$file_name`, multiple potential out-of-bounds array index accesses were detected. Review each instance below to ensure array accesses remain within valid bounds.")
    }
    fn body_multiple_file_multiple_instance(&self) -> String {
        String::from("Across $total_files files, multiple potential out-of-bounds array index accesses were detected. Review each instance below to ensure array accesses remain within valid bounds.")
    }
    fn body_list_item_single_file(&self) -> String {
        {
            "{body_list_item}".to_string()
        }
    }
    fn body_list_item_multiple_file(&self) -> String {
        {
            "{body_list_item}".to_string()
        }
    }
    fn closing(&self) -> String {
        String::from("To resolve this issue, ensure that all array index accesses within loops are properly bounded and do not exceed the array's size. Failing to address this may result in runtime exceptions, data corruption, or exploitable vulnerabilities.")
    }
    fn template(&self) -> String {
        String::from(
            r#"template:
      title: Potential Out-of-Bounds Array Index Access Detected
      opening: Accessing array elements outside their valid index range can lead to runtime errors, security vulnerabilities, or unpredictable program behavior. This issue typically occurs when a loop iterates beyond the array's defined bounds, resulting in unsafe memory access.
      body-single-file-single-instance: |
        In `$file_name`, a potential out-of-bounds array index access was detected in the `$PARENT_NAME` $PARENT_TYPE on line $instance_line. The array access statement `$ARRAY_INDEX_ACCESS` may exceed the array's valid index range.
      body-single-file-multiple-instance: |
        In `$file_name`, multiple potential out-of-bounds array index accesses were detected. Review each instance below to ensure array accesses remain within valid bounds.
      body-multiple-file-multiple-instance: |
        Across $total_files files, multiple potential out-of-bounds array index accesses were detected. Review each instance below to ensure array accesses remain within valid bounds.
      body-list-item-intro: 'The following potential out-of-bounds array index accesses were found in the code:'
      body-list-item-always: '- The `$ARRAY_INDEX_ACCESS` statement in the `$PARENT_NAME` $PARENT_TYPE on line $instance_line of [`$file_name`]($instance_line_link)'
      closing: |
        To resolve this issue, ensure that all array index accesses within loops are properly bounded and do not exceed the array's size. Failing to address this may result in runtime exceptions, data corruption, or exploitable vulnerabilities.
"#,
        )
    }
}

#[no_mangle]
pub extern "C" fn external_detector() -> *mut DetectorOpaque {
    let detector: CompactDetector = Box::new(ArrayLoopBoundCheck);
    Box::into_raw(detector) as *mut DetectorOpaque
}
