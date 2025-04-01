#![warn(clippy::pedantic)]

use anyhow::Result;
use codebase::{Codebase, SealedState};
use std::{cell::RefCell, collections::HashMap};
pub mod ast;
mod builder_tests;
pub mod codebase;
mod passes;
mod storage;

pub trait Rule {
    fn check(
        &self,
        codebase: &RefCell<Codebase<SealedState>>,
    ) -> Option<HashMap<String, Vec<(usize, usize)>>>;

    fn name(&self) -> String;
    fn description(&self) -> String;
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
    files: HashMap<String, String, H>,
) -> Result<RefCell<Codebase<SealedState>>> {
    let mut codebase = Codebase::new();
    for (fname, source_code) in files {
        codebase.add_file(&fname, &source_code);
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
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = std::fs::read_to_string(path).unwrap();
            files.insert(file_name, content);
        }
        let codebase = build_codebase(files).unwrap().into_inner();
        let serialized = serde_json::to_string_pretty(&codebase).unwrap();
        let output_path = directory.join("output.json");
        std::fs::write(output_path, serialized).unwrap();
    }
}
