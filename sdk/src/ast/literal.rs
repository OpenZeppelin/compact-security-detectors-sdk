#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{
    ast_enum, ast_nodes,
    passes::{Node, NodeKind},
};

ast_enum! {
    pub enum Literal {
        Nat(Rc<Nat>),
        Bool(Rc<Bool>),
        Str(Rc<Str>),
        Version(Rc<Version>),
    }
}

ast_nodes! {
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
        pub minor: Rc<Nat>,
        pub bugfix: Option<Rc<Nat>>,
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
