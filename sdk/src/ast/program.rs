#![warn(clippy::pedantic)]
use super::{
    declaration::Declaration, definition::Module, directive::Directive, expression::Expression,
};

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub modules: Vec<Module>,
}

pub enum CompactNode {
    Directive(Directive),
    Declaration(Declaration),
    Expression(Expression),
}
