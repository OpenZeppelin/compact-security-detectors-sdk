#![warn(clippy::pedantic)]
use crate::{
    ast::{
        builder::build_ast,
        node::NodeKind,
        node_type::NodeType,
        program::Program,
        statement::{Assert, Statement},
        ty::Type,
    },
    passes::{build_symbol_table, SymbolTable},
    storage::NodesStorage,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData, rc::Rc};

#[allow(dead_code)]
trait CodebaseOpen {}
#[allow(dead_code)]
trait CodebaseSealed {}

pub struct OpenState;
impl CodebaseOpen for OpenState {}

pub struct SealedState;
impl CodebaseSealed for SealedState {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceCodeFile {
    pub(crate) fname: String,
    pub(crate) ast: Rc<Program>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Codebase<S> {
    pub(crate) storage: NodesStorage,
    #[serde(skip)]
    pub(crate) fname_ast_map: HashMap<String, SourceCodeFile>,
    pub(crate) symbol_tables: HashMap<String, Rc<SymbolTable>>,
    pub(crate) _state: PhantomData<S>,
}

impl Codebase<OpenState> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: NodesStorage::default(),
            fname_ast_map: HashMap::new(),
            symbol_tables: HashMap::new(),
            _state: PhantomData,
        }
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
    pub fn add_file(&mut self, fname: &str, source_code: &str) {
        let compact_language = tree_sitter_compact::LANGUAGE.into();
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&compact_language)
            .expect("Error loading Inference grammar");
        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();
        let ast = build_ast(self, &root_node, source_code).unwrap();
        let source_code_file = SourceCodeFile {
            fname: fname.to_string(),
            ast,
        };
        self.fname_ast_map
            .insert(source_code_file.fname.clone(), source_code_file);
    }

    pub(crate) fn add_node(&mut self, node: NodeType, parent: u128) {
        self.storage.add_node(node, parent);
    }

    /// Seals the codebase, preventing further modifications.
    ///
    /// # Errors
    ///
    /// This function will return an error if building the symbol table fails.
    ///
    /// # Panics
    ///
    /// This function will panic if the symbol table for a file path is not found.
    pub fn seal(mut self) -> Result<Codebase<SealedState>> {
        let mut symbol_tables = HashMap::new();
        for (file_path, source_code_file) in &self.fname_ast_map {
            let symbol_table =
                build_symbol_table(Rc::new(NodeKind::from(&source_code_file.ast)), None)?;
            symbol_tables.insert(file_path.clone(), symbol_table);

            // println!("{}", &symbol_tables.get(file_path).unwrap());
        }
        self.storage.seal();
        Ok(Codebase {
            storage: self.storage,
            fname_ast_map: self.fname_ast_map,
            symbol_tables,
            _state: PhantomData,
        })
    }
}

impl Codebase<SealedState> {
    pub fn files(&self) -> impl Iterator<Item = SourceCodeFile> + '_ {
        self.fname_ast_map.values().cloned()
    }

    #[must_use = "Use this function to get a type for a symbol (Identifier)"]
    pub fn get_symbol_type_by_id(&self, file_path: &str, id: u128) -> Option<Type> {
        self.symbol_tables
            .get(file_path)
            .and_then(|table| table.lookdown_by_id(id))
    }

    pub fn list_assert_nodes(&self) -> impl Iterator<Item = Rc<Assert>> {
        let mut res = Vec::new();
        for item in &self.storage.nodes {
            if let NodeType::Statement(Statement::Assert(assert_stmt)) = item {
                res.push(assert_stmt.clone());
            }
        }
        res.into_iter()
    }
}
