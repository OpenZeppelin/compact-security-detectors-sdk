#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes, passes::Node};

use super::{
    expression::{BinaryExpressionOperator, Identifier},
    literal::Version,
};

ast_enum! {
    pub enum Directive {
        Pragma(Rc<Pragma>),
        Include(Rc<Include>),
    }
}

ast_nodes! {
    pub struct Pragma {
        pub version: Rc<Version>,
        pub value: Rc<Identifier>,
        pub operator: BinaryExpressionOperator,
    }
    pub struct Include {}
}

impl Node for Pragma {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
impl Node for Include {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
