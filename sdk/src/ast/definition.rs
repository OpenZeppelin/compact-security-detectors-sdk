#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::{
    declaration::Argument,
    expression::Identifier,
    node::{Node, NodeKind, Type},
    statement::Block,
};

ast_enum! {
    pub enum Definition {
        Module(Rc<Module>),
        Circuit(Rc<Circuit>),
    }
}

ast_nodes! {
    pub struct Module {}
    pub struct Circuit {
        pub name: Rc<Identifier>,
        pub arguments: Vec<Rc<Argument>>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub is_exported: bool,
        pub is_pure: bool,
        pub ty: Type,
        pub body: Option<Rc<Block>>,
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
