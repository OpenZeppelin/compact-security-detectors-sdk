#![warn(clippy::pedantic)]

use anyhow::Result;
use ast::builder::build_ast;
use codebase::{Codebase, SealedState};
use std::{cell::RefCell, collections::HashMap};
pub mod ast;
pub mod codebase;
mod passes;

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
        let source_code_file = parse_content(&fname, &source_code).unwrap();
        codebase.add_file(source_code_file);
    }
    Ok(RefCell::new(codebase.seal()))
}

/// Parses the content of a source code file and returns a `SourceCodeFile` object.
///
/// # Errors
///
/// This function will return an error if the AST cannot be built from the source code.
///
/// # Panics
///
/// This function will panic if there is an error loading the Inference grammar.
fn parse_content(fname: &str, source_code: &str) -> anyhow::Result<codebase::SourceCodeFile> {
    let compact_language = tree_sitter_compact::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&compact_language)
        .expect("Error loading Inference grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    let ast = build_ast(&root_node, source_code)?;
    let source_code_file = codebase::SourceCodeFile {
        fname: fname.to_string(),
        ast,
    };
    Ok(source_code_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
        let codebase = build_codebase(files).unwrap();
        let _ = codebase.borrow();
    }
}
