#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

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
