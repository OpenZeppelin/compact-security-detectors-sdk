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
    pub struct Bool {}
    pub struct Str {}
    pub struct Version {
        pub major: Nat,
        pub minor: Nat,
        pub bugfix: Option<Nat>,
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
