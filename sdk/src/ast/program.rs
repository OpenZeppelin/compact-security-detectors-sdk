#![warn(clippy::pedantic)]
use std::rc::Rc;

use super::{
    declaration::Declaration,
    definition::{Definition, Module},
    directive::Directive,
    statement::Statement,
};

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Program {}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CompactNode {
    Directive(Directive),
    Declaration(Declaration),
    Definition(Definition),
    Statement(Statement),
    Module(Rc<Module>),
}
