#![warn(clippy::pedantic)]
use crate::ast_nodes;

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Directive {
    Pragma(Pragma),
    Include(Include),
}

ast_nodes! {
    pub struct Pragma {}
    pub struct Include {}
}
