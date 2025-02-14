#![warn(clippy::pedantic)]
use crate::ast_nodes;

use super::definition::Definition;

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum Declaration {
    Import(Import),
    Export(Export),
    External(External),
    Witness(Witness),
    Ledger(Ledger),
    Ctor(Ctor),
    Contract(Contract),
    Struct(Struct),
    Enum(Enum),
    Definition(Definition),
}

ast_nodes! {
    pub struct Import {}

    pub struct Export {}

    pub struct External {}

    pub struct Witness {}

    pub struct Ledger {}

    pub struct Ctor {}

    pub struct Contract {}

    pub struct Struct {}

    pub struct Enum {}
}
