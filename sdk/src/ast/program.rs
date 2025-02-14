#![warn(clippy::pedantic)]
use super::{definition::Module, directive::Directive};

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub modules: Vec<Module>,
}
