#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::{
    expression::Identifier,
    literal::Version,
    node::{Node, NodeKind},
};

ast_enum! {
    pub enum Directive {
        Pragma(Rc<Pragma>),
    }
}

ast_nodes! {
    pub struct Pragma {
        pub version: Rc<Version>,
        pub value: Rc<Identifier>,
        pub operator: PragmaOperator,
    }
}

impl Node for Pragma {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum PragmaOperator {
    Gt,
    Ge,
    Eq,
}
