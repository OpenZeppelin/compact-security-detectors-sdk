#![warn(clippy::pedantic)]
use crate::{
    ast::{
        builder::build_ast,
        definition::Definition,
        node::NodeKind,
        node_type::NodeType,
        program::Program,
        statement::{Assert, For, Statement},
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

    pub(crate) fn add_node(&mut self, node: NodeType, parent: u32) {
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
    pub fn get_symbol_type_by_id(&self, id: u32) -> Option<Type> {
        if let Some(file) = self.find_node_file(id) {
            self.symbol_tables
                .get(&file.fname)
                .and_then(|table| table.lookdown_by_id(id))
        } else {
            None
        }
    }

    pub fn list_assert_nodes(&self) -> impl Iterator<Item = Rc<Assert>> + '_ {
        self.list_nodes_cmp(|node| {
            if let NodeType::Statement(Statement::Assert(stmt)) = node {
                Some(stmt.clone())
            } else {
                None
            }
        })
    }

    pub fn list_for_statement_nodes(&self) -> impl Iterator<Item = Rc<For>> + '_ {
        self.list_nodes_cmp(|node| {
            if let NodeType::Statement(Statement::For(stmt)) = node {
                Some(stmt.clone())
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn get_parent_container(&self, id: u32) -> Option<NodeType> {
        let mut current_id = id;
        while let Some(route) = self.storage.find_parent_node(current_id) {
            current_id = route;
            if let Some(node) = self.storage.find_node(current_id) {
                if let NodeType::Definition(Definition::Circuit(_) | Definition::Module(_)) = node {
                    return self.storage.find_node(node.id());
                }
            }
        }
        None
    }

    pub fn get_children_cmp<F>(&self, id: u32, comparator: F) -> Vec<NodeType>
    where
        F: Fn(&NodeType) -> bool,
    {
        let mut result = Vec::new();
        let mut stack: Vec<NodeType> = Vec::new();

        if let Some(root_node) = self.storage.find_node(id) {
            stack.push(root_node.clone());
        }

        while let Some(current_node) = stack.pop() {
            if comparator(&current_node) {
                result.push(current_node.clone());
            }
            stack.extend(current_node.children());
        }

        result
    }

    fn list_nodes_cmp<'a, T, F>(&'a self, cast: F) -> impl Iterator<Item = T> + 'a
    where
        F: Fn(&NodeType) -> Option<T> + 'a,
        T: Clone + 'static,
    {
        self.storage.nodes.iter().filter_map(cast)
    }

    fn find_node_file(&self, id: u32) -> Option<SourceCodeFile> {
        if let Some((_, file)) = self
            .fname_ast_map
            .iter()
            .find(|(_, file)| file.ast.id == id)
        {
            Some(file.clone())
        } else {
            let mut node_id = id;
            while let Some(parent) = self.storage.find_parent_node(node_id) {
                if parent == 0 {
                    if let Some(file) = self.storage.find_node(node_id) {
                        match file {
                            NodeType::Program(f) => {
                                if let Some((fname, _)) = self
                                    .fname_ast_map
                                    .iter()
                                    .find(|(_, file)| Rc::ptr_eq(&file.ast, &f))
                                {
                                    return Some(SourceCodeFile {
                                        fname: fname.clone(),
                                        ast: f.clone(),
                                    });
                                }
                            }
                            _ => return None,
                        }
                    }
                }
                node_id = parent;
            }
            None
        }
    }
}
