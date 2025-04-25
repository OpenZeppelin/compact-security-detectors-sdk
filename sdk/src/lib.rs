#![warn(clippy::pedantic)]
#![recursion_limit = "1024"]
//! # Compact Security Detectors SDK root module
//! This module provides the main entry point for building a codebase and exposing SDK API.
//!
//! ## Public members
//!
//! - `ast` module contains the abstract syntax tree (AST) representation of the codebase.
//! - `detector` module contrains Detector trait framework and macro for implementing detectors.
//! - `codebase` module contains the Codebase struct and its methods for managing the codebase.
//!
//! The function `build_codebase` is the main entry point for building a codebase from source files.
//! It takes a map of file paths to source code strings and returns a `Result` containing a boxed `Codebase` in the `SealedState`.
//!
//! ## Example
//! ```
//! use std::collections::HashMap;
//! use compact_security_detectors_sdk::build_codebase;
//!
//! fn example_test() {
//!     let mut files = HashMap::new();
//!     // Use DSL grammar for for-loop
//!     let src = r"circuit foo(x: Uint<8>) : Uint<8> { for (const i of 0 .. 1) { } return x; }";
//!     files.insert("t.compact".to_string(), src.to_string());
//!     let cb = build_codebase(&files).unwrap();
//!     // Only test for-loop detection; assert statements may vary by grammar
//!     let fors: Vec<_> = cb.list_for_statement_nodes().collect();
//!     assert_eq!(fors.len(), 1);
//! }
//! ```
use anyhow::Result;
use codebase::{Codebase, SealedState};
use std::collections::HashMap;
pub mod ast;
mod builder_tests;
pub mod codebase;
pub mod detector;
mod passes;
mod storage;

/// Builds a codebase from the provided source files.
///
/// # Arguments
///
/// * `files` - A map where the keys are file paths (absolute) and the values are the corresponding source code strings.
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
) -> Result<Box<Codebase<SealedState>>> {
    let mut codebase = Codebase::new();
    for (file_path, source_code) in files {
        codebase.add_file(file_path, source_code);
    }
    Ok(Box::new(codebase.seal()?))
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        definition::Definition,
        expression::Expression,
        literal::{Bool, Nat, Str},
        node::Location,
        node_type::NodeType,
        statement::Statement,
        ty::{Type, TypeBool, TypeNat, TypeString, Vector, VectorSize},
    };

    use super::*;
    use std::{collections::HashMap, rc::Rc};

    #[test]
    #[ignore] // Skip this test in CI workflows
    fn test_build_codebase_from_corpus() {
        let directory = std::env::current_dir().unwrap();
        let corpus_directory = directory.parent().unwrap().join("corpus");
        let mut files = HashMap::new();
        for entry in std::fs::read_dir(corpus_directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = std::fs::read_to_string(path).unwrap();
            files.insert(file_name, content);
        }
        let _ = build_codebase(&files).unwrap();
    }

    #[test]
    fn test_build_codebase_simple() {
        let mut files = HashMap::new();
        let src = "circuit foo() : Uint<8> { return 0; }";
        files.insert("a.compact".to_string(), src.to_string());
        let cb = build_codebase(&files).expect("build_codebase failed");
        assert_eq!(cb.files.len(), 1);
        assert_eq!(cb.symbol_tables.len(), 1);
        let scf = cb
            .files
            .iter()
            .find(|f| f.file_path == "a.compact")
            .unwrap();
        let ast = &scf.ast;
        assert_eq!(ast.definitions.len(), 1);
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        assert_eq!(circuits[0].name(), "foo");
    }

    /// Test files iterator and parent container resolution
    #[test]
    fn test_files_and_parent_container() {
        let mut files = HashMap::new();
        let src = "circuit foo() : Uint<8> { return 1; }";
        files.insert("a.compact".to_string(), src.to_string());
        let cb = build_codebase(&files).unwrap();
        let fs: Vec<_> = cb.files().collect();
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].file_path, "a.compact");
        let ret_id = cb
            .storage
            .nodes
            .iter()
            .find_map(|n| {
                if let NodeType::Statement(Statement::Return(stmt)) = n {
                    Some(stmt.id)
                } else {
                    None
                }
            })
            .expect("No return statement found");
        let parent = cb
            .get_parent_container(ret_id)
            .expect("Parent container not found");
        // parent should be the circuit definition
        if let NodeType::Definition(Definition::Circuit(_)) = parent {
        } else {
            panic!("Expected circuit definition as parent");
        }
    }

    #[test]
    fn test_type_matches_and_display() {
        let nat_lit = Rc::new(Nat {
            id: 1,
            location: Location::default(),
            value: 5,
        });
        let tn = TypeNat::new(&nat_lit);
        let ty_nat = Type::Nat(Rc::new(tn));
        assert!(ty_nat.matches(&ty_nat));
        assert_eq!(ty_nat.to_string(), "nat");
        // Bool type
        let bool_lit = Rc::new(Bool {
            id: 2,
            location: Location::default(),
            value: true,
        });
        let tb = TypeBool::new(&bool_lit);
        let ty_bool = Type::Boolean(Rc::new(tb));
        assert!(ty_bool.matches(&ty_bool));
        assert_eq!(ty_bool.to_string(), "bool");
        // String type
        let str_lit = Rc::new(Str {
            id: 3,
            location: Location::default(),
            value: "hi".to_string(),
        });
        let ts = TypeString::new(&str_lit);
        let ty_str = Type::String(Rc::new(ts));
        assert_eq!(ty_str.to_string(), "string");
        // Vector size extraction
        let vec = Vector {
            id: 4,
            location: Location::default(),
            size: VectorSize::Nat(nat_lit.clone()),
            ty: ty_nat.clone(),
        };
        assert_eq!(vec.size_nat(), Some(5));
    }

    #[test]
    fn test_get_symbol_type_by_id() {
        let mut files = HashMap::new();
        let src = "circuit foo(x: Uint<8>) : Uint<8> { return x; }";
        files.insert("a.compact".to_string(), src.to_string());
        let cb = build_codebase(&files).unwrap();
        // find identifier for 'x'
        let x_id = cb
            .storage
            .nodes
            .iter()
            .find_map(|n| {
                if let NodeType::Expression(Expression::Identifier(ident)) = n {
                    if ident.name == "x" {
                        return Some(ident.id);
                    }
                }
                None
            })
            .expect("Identifier x not found");
        let ty = cb.get_symbol_type_by_id(x_id).expect("Type not found");
        assert!(matches!(ty, Type::Uint(_)));
    }
}
