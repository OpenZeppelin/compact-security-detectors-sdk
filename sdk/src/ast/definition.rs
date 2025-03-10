#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::{
    declaration::Argument,
    expression::Identifier,
    node::{Node, NodeKind},
    program::CompactNode,
    statement::Block,
    ty::Type,
};

ast_enum! {
    pub enum Definition {
        Module(Rc<Module>),
        Circuit(Rc<Circuit>),
        Structure(Rc<Structure>),
        Enum(Rc<Enum>),
    }
}

ast_nodes! {
    pub struct Module {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub nodes: Vec<CompactNode>,
    }

    pub struct Circuit {
        pub name: Rc<Identifier>,
        pub arguments: Vec<Rc<Argument>>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub is_exported: bool,
        pub is_pure: bool,
        pub ty: Type,
        pub body: Option<Rc<Block>>,
    }

    pub struct Structure {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub fields: Vec<Rc<Field>>,
    }

    pub struct Field {
        pub name: Rc<Identifier>,
        pub ty: Type,
    }

    pub struct Enum {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub options: Vec<Rc<Identifier>>,
    }
}

impl Node for Module {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Circuit {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Structure {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Field {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Enum {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
