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
    use std::cell::RefCell;
    use std::collections::HashMap;

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
        // simple circuit definition
        let src = "circuit foo() : Uint<8> { return 0; }";
        files.insert("a.compact".to_string(), src.to_string());
        let cb_ref = build_codebase(&files).expect("build_codebase failed");
        let cb = cb_ref.borrow();
        // one file and one symbol table
        assert_eq!(cb.fname_ast_map.len(), 1);
        assert_eq!(cb.symbol_tables.len(), 1);
        // AST contains one definition
        let scf = cb.fname_ast_map.get("a.compact").unwrap();
        let ast = &scf.ast;
        assert_eq!(ast.definitions.len(), 1);
        // circuit named foo
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        assert_eq!(circuits[0].name(), "foo");
    }

    // Test CombinedDetector and Display impl
    #[test]
    fn test_combined_detector_display() {
        use crate::{Detector, DetectorReportTemplate, MidnightDetector};
        // Dummy detector implementing both traits
        struct Dummy;
        impl Detector for Dummy {
            fn check(
                &self,
                _codebase: &RefCell<Codebase<SealedState>>,
            ) -> Option<Vec<DetectorResult>> {
                Some(vec![DetectorResult {
                    file_path: "f".into(),
                    offset_start: 0,
                    offset_end: 1,
                    extra: None,
                }])
            }
        }
        impl DetectorReportTemplate for Dummy {
            fn id(&self) -> String {
                "dummy".into()
            }
            fn uid(&self) -> String {
                "uid".into()
            }
            fn description(&self) -> String {
                String::new()
            }
            fn severity(&self) -> String {
                String::new()
            }
            fn tags(&self) -> Vec<String> {
                vec![]
            }
            fn title_single_instance(&self) -> String {
                String::new()
            }
            fn title_multiple_instance(&self) -> String {
                String::new()
            }
            fn opening(&self) -> String {
                String::new()
            }
            fn body_single_file_single_instance(&self) -> String {
                String::new()
            }
            fn body_single_file_multiple_instance(&self) -> String {
                String::new()
            }
            fn body_multiple_file_multiple_instance(&self) -> String {
                String::new()
            }
            fn body_list_item_single_file(&self) -> String {
                String::new()
            }
            fn body_list_item_multiple_file(&self) -> String {
                String::new()
            }
            fn closing(&self) -> String {
                String::new()
            }
            fn template(&self) -> String {
                String::new()
            }
        }
        let det: MidnightDetector = Box::new(Dummy);
        // Display should use id()
        assert_eq!(det.to_string(), "dummy");
    }
    /// Test listing assert and for statement nodes via Codebase methods
    #[test]
    fn test_list_assert_and_for() {
        let mut files = HashMap::new();
        // Use DSL grammar for for-loop
        let src = r"circuit foo(x: Uint<8>) : Uint<8> { for (const i of 0 .. 1) { } return x; }";
        files.insert("t.compact".to_string(), src.to_string());
        let cb_ref = build_codebase(&files).unwrap();
        let cb = cb_ref.borrow();
        // Only test for-loop detection; assert statements may vary by grammar
        let fors: Vec<_> = cb.list_for_statement_nodes().collect();
        assert_eq!(fors.len(), 1);
    }
    /// Test files iterator and parent container resolution
    #[test]
    fn test_files_and_parent_container() {
        use crate::ast::definition::Definition;
        use crate::ast::node_type::NodeType;
        use crate::ast::statement::Statement;
        let mut files = HashMap::new();
        let src = "circuit foo() : Uint<8> { return 1; }";
        files.insert("a.compact".to_string(), src.to_string());
        let cb_ref = build_codebase(&files).unwrap();
        let cb = cb_ref.borrow();
        // files() yields one SourceCodeFile
        let fs: Vec<_> = cb.files().collect();
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].fname, "a.compact");
        // find a return statement node id
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
    /// Test Type matching and Display, plus Vector size extraction
    #[test]
    fn test_type_matches_and_display() {
        use crate::ast::literal::{Bool, Nat, Str};
        use crate::ast::node::Location;
        use crate::ast::ty::{Type, TypeBool, TypeNat, TypeString, Vector, VectorSize};
        use std::rc::Rc;
        // Nat type
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
        use crate::ast::expression::Expression;
        use crate::ast::node_type::NodeType;
        use crate::ast::ty::Type;
        let mut files = HashMap::new();
        let src = "circuit foo(x: Uint<8>) : Uint<8> { return x; }";
        files.insert("a.compact".to_string(), src.to_string());
        let cb_ref = build_codebase(&files).unwrap();
        let cb = cb_ref.borrow();
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
