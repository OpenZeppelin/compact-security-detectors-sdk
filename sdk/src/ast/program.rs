#![warn(clippy::pedantic)]
use std::rc::Rc;

use super::{
    declaration::Declaration,
    definition::{Definition, Module},
    directive::Directive,
};

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub declarations: Vec<Declaration>,
    pub definitions: Vec<Definition>,
    pub modules: Vec<Rc<Module>>,
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CompactNode {
    Directive(Directive),
    Declaration(Declaration),
    Definition(Definition),
    Module(Rc<Module>),
    Comment(String),
}
