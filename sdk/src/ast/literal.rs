#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::{
    expression::Expression,
    node::{Node, NodeKind},
};

ast_enum! {
    pub enum Literal {
        Array(Rc<Array>),
        Nat(Rc<Nat>),
        Bool(Rc<Bool>),
        Str(Rc<Str>),
        Version(Rc<Version>),
    }
}

ast_nodes! {
    pub struct Array {
        pub elements: Vec<Expression>,
    }
    pub struct Nat {
        pub value: u64,
    }
    pub struct Bool {
        pub value: bool,
    }
    pub struct Str {
        pub value: String,
    }
    pub struct Version {
        pub major: Rc<Nat>,
        pub minor: Option<Rc<Nat>>,
        pub bugfix: Option<Rc<Nat>>,
        pub operator: VersionOperator,
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum VersionOperator {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Neq,
}

impl Node for Array {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut res = vec![];
        for element in &self.elements {
            res.push(Rc::new(NodeKind::from(&element.clone())));
        }
        res
    }
}

impl Node for Nat {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Bool {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Str {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Version {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
