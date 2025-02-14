#![warn(clippy::pedantic)]
use crate::ast_nodes;

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Literal {
    Nat(Nat),
    Str(Str),
    Version(Version),
}

ast_nodes! {
    pub struct Nat {}
    pub struct Str {}
    pub struct Version {}
}
