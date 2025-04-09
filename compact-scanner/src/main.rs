use std::collections::HashMap;

use clap::Parser;
use midnight_security_detectors::all_detectors;
use midnight_security_detectors_sdk::{build_codebase, DetectorResult, MidnightDetector};
use parser::Cli;
use serde_json::json;

mod parser;

fn main() {
    let args = Cli::parse();
    if args.metadata {
        println!("{}", get_scanner_metadata());
        return;
    }

    if args.paths.is_some() {
        let mut corpus = HashMap::new();
        for path in &args.paths.unwrap() {
            if path.is_dir() {
                let mut stack = vec![path.clone()];
                while let Some(current_path) = stack.pop() {
                    for entry in std::fs::read_dir(current_path).unwrap() {
                        let entry = entry.unwrap();
                        let p = entry.path();
                        if p.is_dir() {
                            stack.push(p);
                        } else if p.is_file() && p.extension().unwrap_or_default() == "compact" {
                            let file_content = std::fs::read_to_string(&p).unwrap();
                            corpus.insert(p.to_string_lossy().to_string(), file_content);
                        }
                    }
                }
            } else if path.is_file() {
                if path.extension().unwrap_or_default() != "compact" {
                    continue;
                }
                let file_content = std::fs::read_to_string(path).unwrap();
                corpus.insert(path.to_string_lossy().to_string(), file_content);
            }
        }
        if !corpus.is_empty() {
            let result = execute_rules(&corpus, args.detectors);
            let mut delector_responses = Vec::new();
            for (detector_name, errors) in result {
                let instances = detector_result_to_json(errors, args.project_root.clone());
                let detector_response = json!({
                    detector_name : {
                        "finding": {
                            "instances": instances
                        }
                    },
                    "errors": null
                });
                delector_responses.push(detector_response);
            }
            let res = json!({
                "errors": [],
                "files_scanned": &corpus.keys().collect::<Vec<_>>()
                    .iter()
                    .map(|k| relative_file_path(k, &args.project_root))
                    .collect::<Vec<_>>(),
                "detector_responses": delector_responses,
            });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
    }
}

fn execute_rules(
    files: &HashMap<String, String>,
    rules: Option<Vec<String>>,
) -> HashMap<String, Vec<DetectorResult>> {
    let codebase = build_codebase(files).unwrap();
    let selected_detectors: Vec<_> = available_detectors()
        .into_iter()
        .filter(|detector| {
            if let Some(ref rules) = rules {
                rules.contains(&detector.name().to_string())
            } else {
                true
            }
        })
        .collect();

    let mut results = HashMap::new();
    for detector in selected_detectors {
        let detector_result = detector.check(&codebase);
        if let Some(errors) = detector_result {
            results.insert(detector.name().to_string(), errors);
        }
    }
    results
}

fn detector_result_to_json(
    errors: Vec<DetectorResult>,
    project_root: Option<std::path::PathBuf>,
) -> serde_json::Value {
    let mut json_errors = Vec::new();
    for error in errors {
        let file_path = relative_file_path(&error.file_path, &project_root);

        let json_error = json!({
            "file_path": file_path,
            "offset_start": error.offset_start,
            "offset_end": error.offset_end,
            "suggested_fixes": [],
            "extras": error.extras,
        });
        json_errors.push(json_error);
    }
    json!(json_errors)
}

fn relative_file_path(file_path: &str, project_root: &Option<std::path::PathBuf>) -> String {
    if let Some(root) = project_root {
        match std::path::Path::new(file_path).strip_prefix(root) {
            Ok(relative_path) => relative_path.to_string_lossy().to_string(),
            Err(_) => file_path.to_string(),
        }
    } else {
        file_path.to_string()
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
