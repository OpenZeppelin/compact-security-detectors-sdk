#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::definition::Definition;

ast_enum! {
    pub enum Declaration {
        Import(Rc<Import>),
        Export(Rc<Export>),
        External(Rc<External>),
        Witness(Rc<Witness>),
        Ledger(Rc<Ledger>),
        Ctor(Rc<Ctor>),
        Contract(Rc<Contract>),
        Struct(Rc<Struct>),
        Enum(Rc<Enum>),
        Definition(Rc<Definition>),
    }
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
