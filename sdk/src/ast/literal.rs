#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{
    ast_enum, ast_nodes,
    passes::{Node, NodeKind, SameScopeNode},
};

ast_enum! {
    pub enum Literal {
        Nat(Rc<Nat>),
        Bool(Rc<Bool>),
        Str(Rc<Str>),
        Version(Rc<Version>),
    }
}

impl From<&Literal> for NodeKind {
    fn from(literal: &Literal) -> Self {
        match literal {
            Literal::Nat(nat) => NodeKind::SameScopeNode(SameScopeNode::Composite(nat.clone())),
            Literal::Bool(r#bool) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(r#bool.clone()))
            }
            Literal::Str(str) => NodeKind::SameScopeNode(SameScopeNode::Composite(str.clone())),
            Literal::Version(version) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(version.clone()))
            }
        }
    }
}

ast_nodes! {
    pub struct Nat {}
    pub struct Bool {}
    pub struct Str {}
    pub struct Version {}
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
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
