#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes, passes::Node};

use super::definition::Definition;

ast_enum! {
    pub enum Declaration {
        Import(Rc<Import>),
        Export(Rc<Export>),
        External(Rc<External>),
        Witness(Rc<Witness>),
        Ledger(Rc<Ledger>),
        @scope Ctor(Rc<Ctor>),
        @scope Contract(Rc<Contract>),
        @scope Struct(Rc<Struct>),
        @scope Enum(Rc<Enum>),
        @raw Definition(Definition),
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

impl Node for Import {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}

impl Node for Export {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for External {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Witness {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Ledger {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Ctor {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Contract {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Struct {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Enum {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
