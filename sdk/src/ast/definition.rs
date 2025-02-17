#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

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
