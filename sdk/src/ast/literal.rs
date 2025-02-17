#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

ast_enum! {
    pub enum Literal {
        Nat(Rc<Nat>),
        Str(Rc<Str>),
        Version(Rc<Version>),
    }
}

ast_nodes! {
    pub struct Nat {}
    pub struct Str {}
    pub struct Version {}
}
