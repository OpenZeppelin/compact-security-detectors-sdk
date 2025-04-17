use std::{cell::RefCell, collections::HashMap};

use midnight_security_detectors_sdk::{
    ast::{declaration::Declaration, definition::Definition, node_type::NodeType},
    codebase::{Codebase, SealedState},
    DetectorResult,
};

use crate::detector;

detector! {
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
                let mut parent_type = "circuit";
                let parent_name = match parent {
                    Some(NodeType::Definition(Definition::Circuit(c))) => c.name(),
                    Some(NodeType::Declaration(Declaration::Constructor(_))) => {
                        parent_type = "constructor";
                        String::default()
                    }
                    _ => String::new(),
                };
                errors.push(DetectorResult {
                    file_path: codebase.find_node_file(assert_node.id).unwrap().fname,
                    offset_start: assert_node.location.offset_start,
                    offset_end: assert_node.location.offset_end,
                    extra: {
                        let mut map = HashMap::new();
                        map.insert("PARENT_NAME".to_string(), parent_name);
                        map.insert("PARENT_TYPE".to_string(), parent_type.to_string());
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
}
