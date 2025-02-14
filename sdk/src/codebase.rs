#![warn(clippy::pedantic)]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use crate::ast::declaration::Contract;

#[allow(dead_code)]
trait CodebaseOpen {}
#[allow(dead_code)]
trait CodebaseSealed {}

pub struct OpenState;
impl CodebaseOpen for OpenState {}

pub struct SealedState;
impl CodebaseSealed for SealedState {}

#[allow(dead_code)] //REMOVE
pub struct SourceCodeFile {
    pub(crate) fname: String,
    pub(crate) content: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)] //REMOVE
pub struct Codebase<S> {
    #[serde(skip)]
    pub(crate) fname_ast_map: Option<HashMap<String, SourceCodeFile>>,
    pub(crate) _state: PhantomData<S>,
}

impl Codebase<SealedState> {
    // pub fn files(&self) -> impl Iterator<Item = Rc<SourceCodeFile>> {
    //     todo!("Implement this method")
    // }

    // pub fn contracts(&self) -> impl Iterator<Item = Rc<Contract>> {
    //     todo!("Implement this method")
    // }
}
