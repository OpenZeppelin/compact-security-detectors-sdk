use std::collections::HashMap;

use compact_security_detectors_sdk::{
    ast::{declaration::Declaration, definition::Definition, node_type::NodeType},
    codebase::{Codebase, SealedState},
    detector::DetectorResult,
};

compact_security_detectors_sdk::detector! {
    #[type_name = AssertionErrorMessageVerbose]
    fn assertion_error_message_verbose(
        codebase: &Codebase<SealedState>,
    ) -> Option<Vec<DetectorResult>> {
        let mut errors = Vec::new();
        for assert_node in codebase.list_assert_nodes() {
            if assert_node
                .message()
                .is_none_or(|msg| msg.trim().is_empty() || msg.len() < 3)
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
                    file_path: codebase.find_node_file(assert_node.id).unwrap().file_path,
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

#[cfg(test)]
mod tests {
    use compact_security_detectors_sdk::build_codebase;

    use super::*;

    #[test]
    fn test_all_detectors() {
        let detector = AssertionErrorMessageVerbose;
        let src = "export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            assert admin == pad(32, \"\") \"\";
            admin = new_admin;
            return [];
        }";
        let mut data = HashMap::new();
        data.insert("test.compact".to_string(), src.to_string());
        let codebase = build_codebase(&data).unwrap();
        let result = detector.check(codebase.as_ref());
        assert!(result.is_some());
        assert_eq!(result.as_ref().unwrap().len(), 1, "{result:?}");
        let detector_result = result.as_ref().unwrap().first().unwrap();
        assert_eq!(detector_result.file_path, "test.compact");
        assert_eq!(detector_result.offset_start, 153);
        assert_eq!(detector_result.offset_end, 184);
        assert_eq!(detector_result.extra, {
            let mut map = HashMap::new();
            map.insert("PARENT_NAME".to_string(), "set_admin".to_string());
            map.insert("PARENT_TYPE".to_string(), "circuit".to_string());
            Some(map)
        });
    }
}
