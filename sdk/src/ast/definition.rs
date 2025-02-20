#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes, passes::Node};

ast_enum! {
    pub enum Definition {
        Module(Rc<Module>),
        Circuit(Rc<Circuit>),
    }
}

ast_nodes! {
    pub struct Module {}
    pub struct Circuit {}
}

impl Node for Module {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}

impl Node for Circuit {
    fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
        vec![]
    }
}
