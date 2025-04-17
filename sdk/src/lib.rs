#![warn(clippy::pedantic)]
#![recursion_limit = "1024"]

use anyhow::Result;
use codebase::{Codebase, SealedState};
use std::{cell::RefCell, collections::HashMap, fmt::Display};
pub mod ast;
mod builder_tests;
pub mod codebase;
mod passes;
mod storage;

pub trait CombinedDetector: Detector + DetectorReportTemplate {}

impl<T: Detector + DetectorReportTemplate> CombinedDetector for T {}

pub type MidnightDetector = Box<dyn CombinedDetector>;

#[derive(Debug, Clone)]
pub struct DetectorResult {
    pub file_path: String,
    pub offset_start: u32,
    pub offset_end: u32,
    pub extra: Option<HashMap<String, String>>,
}

pub trait Detector {
    fn check(&self, codebase: &RefCell<Codebase<SealedState>>) -> Option<Vec<DetectorResult>>;
}

pub trait DetectorReportTemplate {
    fn id(&self) -> String;
    fn uid(&self) -> String;
    fn description(&self) -> String;
    fn severity(&self) -> String;
    fn tags(&self) -> Vec<String>;
    fn title_single_instance(&self) -> String;
    fn title_multiple_instance(&self) -> String;
    fn opening(&self) -> String;
    fn body_single_file_single_instance(&self) -> String;
    fn body_single_file_multiple_instance(&self) -> String;
    fn body_multiple_file_multiple_instance(&self) -> String;
    fn body_list_item_single_file(&self) -> String;
    fn body_list_item_multiple_file(&self) -> String;
    fn closing(&self) -> String;
    fn template(&self) -> String;
}

impl Display for dyn CombinedDetector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

/// Builds a codebase from the provided source files.
///
/// # Errors
///
/// This function will return an error if the source code cannot be parsed.
///
/// # Panics
///
/// This function will panic if there is an error loading the Inference grammar.
pub fn build_codebase<H: std::hash::BuildHasher>(
    files: &HashMap<String, String, H>,
) -> Result<RefCell<Codebase<SealedState>>> {
    let mut codebase = Codebase::new();
    for (fname, source_code) in files {
        codebase.add_file(fname, source_code);
    }
    Ok(RefCell::new(codebase.seal()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Skip this test in CI workflows
    fn test_build_codebase() {
        let directory = std::env::current_dir().unwrap();
        let corpus_directory = directory.parent().unwrap().join("corpus");
        let mut files = HashMap::new();
        for entry in std::fs::read_dir(corpus_directory).unwrap() {
            if entry.as_ref().unwrap().path().is_dir() {
                continue;
            }
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = std::fs::read_to_string(path).unwrap();
            files.insert(file_name, content);
        }
        let _ = build_codebase(&files).unwrap();
    }
}
