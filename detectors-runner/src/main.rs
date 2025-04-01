use std::collections::HashMap;

use midnight_security_detectors::all_detectors;
use midnight_security_detectors_sdk::{build_codebase, Detector};

fn main() {
    let contract_content = r#"export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            assert admin == pad(32, "") || admin == current_proof;
            admin = new_admin;
            return [];
        }"#;

    let mut data = HashMap::new();
    data.insert("test.compact".to_string(), contract_content.to_string());
    let codebase = build_codebase(data).unwrap();
    let mut detectors = all_detectors();
    detectors.extend(custom_detectors());
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
}

#[allow(clippy::let_and_return, unused_mut)]
fn custom_detectors() -> Vec<Box<dyn Detector>> {
    let mut detectors: Vec<Box<dyn Detector>> = Vec::new();
    //Import and add your detectors here
    detectors
}
