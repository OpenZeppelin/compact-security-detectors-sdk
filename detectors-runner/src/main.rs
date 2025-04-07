use std::collections::HashMap;

use clap::Parser;
use midnight_security_detectors::all_detectors;
use midnight_security_detectors_sdk::{build_codebase, MidnightDetector};
use parser::Cli;
use serde_json::json;

mod parser;

fn main() {
    let args = Cli::parse();
    if args.detectors {
        println!("{}", get_scanner_metadata());
        return;
    }
    let contract_content = r#"export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            assert admin == pad(32, "") || admin == current_proof;
            admin = new_admin;
            return [];
        }"#;

    let mut data = HashMap::new();
    data.insert("test.compact".to_string(), contract_content.to_string());
    let codebase = build_codebase(data).unwrap();
    for detector in available_detectors() {
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

fn available_detectors() -> Vec<MidnightDetector> {
    all_detectors()
        .into_iter()
        .chain(custom_detectors())
        .collect()
}

#[allow(clippy::let_and_return, unused_mut)]
fn custom_detectors() -> Vec<MidnightDetector> {
    let mut detectors: Vec<MidnightDetector> = Vec::new();
    detectors
        .into_iter()
        .map(|detector| detector as MidnightDetector)
        .collect()
}

fn get_scanner_metadata() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let org = "OpenZeppelin";
    let mut detectors = Vec::new();
    for detector in available_detectors() {
        let json_detector = json!({
            detector.name() : {
                "description": detector.description(),
                "report": {
                    "severity": detector.severity(),
                    "tags": detector.tags(),
                }
            }
        });
        detectors.push(json_detector);
    }
    let scanner_json = json!({
        "compact_scanner": {
            "version": version,
            "org": org,
            "detectors": {
                "detectors": detectors,
            }
        }
    });
    serde_json::to_string_pretty(&scanner_json).unwrap()
}
