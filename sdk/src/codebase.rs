#![warn(clippy::pedantic)]
use crate::{
    ast::{node::NodeKind, program::Program, ty::Type},
    passes::{build_symbol_table, SymbolTable},
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
    #[serde(skip)]
    pub(crate) fname_ast_map: HashMap<String, SourceCodeFile>,
    pub(crate) symbol_tables: HashMap<String, Rc<SymbolTable>>,
    pub(crate) _state: PhantomData<S>,
}

impl Codebase<OpenState> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fname_ast_map: HashMap::new(),
            symbol_tables: HashMap::new(),
            _state: PhantomData,
        }
    }

    pub fn add_file(&mut self, source_code_file: SourceCodeFile) {
        self.fname_ast_map
            .insert(source_code_file.fname.clone(), source_code_file);
    }

    /// Seals the codebase, preventing further modifications.
    ///
    /// # Errors
    ///
    /// This function will return an error if building the symbol table fails.
    pub fn seal(self) -> Result<Codebase<SealedState>> {
        let mut symbol_tables = HashMap::new();
        for (file_path, source_code_file) in &self.fname_ast_map {
            let symbol_table =
                build_symbol_table(Rc::new(NodeKind::from(&source_code_file.ast)), None)?;
            symbol_tables.insert(
                file_path.clone(),
                build_symbol_table(Rc::new(NodeKind::from(&source_code_file.ast)), None)?,
            );

            println!("{symbol_table}");
        }
        Ok(Codebase {
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
}
