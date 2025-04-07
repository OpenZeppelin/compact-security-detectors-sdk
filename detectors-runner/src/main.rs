use std::collections::HashMap;

use clap::Parser;
use midnight_security_detectors::all_detectors;
use midnight_security_detectors_sdk::{build_codebase, MidnightDetector};
use parser::Cli;

mod parser;

fn main() {
    let args = Cli::parse();
    if args.detectors {
        println!("Available detectors:");
        for detector in available_detectors() {
            println!("- {}", detector.name());
        }
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
