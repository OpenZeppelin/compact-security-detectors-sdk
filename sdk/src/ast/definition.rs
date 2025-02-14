#![warn(clippy::pedantic)]
use crate::ast_nodes;

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Definition {
    Module(Module),
    Circuit(Circuit),
}

ast_nodes! {
    pub struct Module {}
    pub struct Circuit {}
}
