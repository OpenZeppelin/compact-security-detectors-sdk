#![warn(clippy::pedantic)]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

use crate::ast::program::Program;

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
    pub(crate) ast: Program,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Codebase<S> {
    #[serde(skip)]
    pub(crate) fname_ast_map: HashMap<String, SourceCodeFile>,
    pub(crate) _state: PhantomData<S>,
}

impl Codebase<OpenState> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fname_ast_map: HashMap::new(),
            _state: PhantomData,
        }
    }

    pub fn add_file(&mut self, source_code_file: SourceCodeFile) {
        self.fname_ast_map
            .insert(source_code_file.fname.clone(), source_code_file);
    }

    #[must_use]
    pub fn seal(self) -> Codebase<SealedState> {
        Codebase {
            fname_ast_map: self.fname_ast_map,
            _state: PhantomData,
        }
    }
}

impl Codebase<SealedState> {
    pub fn files(&self) -> impl Iterator<Item = SourceCodeFile> + '_ {
        self.fname_ast_map.values().cloned()
    }
}
