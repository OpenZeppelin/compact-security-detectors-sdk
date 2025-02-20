#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes, passes::Node};

ast_enum! {
    pub enum Directive {
        Pragma(Rc<Pragma>),
        Include(Rc<Include>),
    }
}

ast_nodes! {
    pub struct Pragma {}
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
