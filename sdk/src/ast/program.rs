#![warn(clippy::pedantic)]
use super::{
    declaration::Declaration, definition::Module, directive::Directive, statement::Statement,
};

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub modules: Vec<Module>,
}

pub enum CompactNode {
    Directive(Directive),
    Declaration(Declaration),
    Statement(Statement),
}
