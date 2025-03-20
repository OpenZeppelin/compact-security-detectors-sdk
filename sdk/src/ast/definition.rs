#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast::statement::Statement, ast_enum, ast_nodes};

use super::{
    declaration::{Argument, Declaration, PatternArgument},
    expression::{Expression, Identifier},
    node::{Node, NodeKind},
    program::CompactNode,
    statement::Block,
    ty::Type,
};

ast_enum! {
    pub enum Definition {
        @scope Module(Rc<Module>),
        @scope Circuit(Rc<Circuit>),
        @scope Structure(Rc<Structure>),
        @scope Enum(Rc<Enum>),
    }
}

ast_nodes! {
    pub struct Module {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub nodes: Vec<CompactNode>,
    }

    pub struct Circuit {
        pub name: Rc<Identifier>,
        pub arguments: Vec<Rc<PatternArgument>>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub is_exported: bool,
        pub is_pure: bool,
        pub ty: Type,
        pub body: Option<Rc<Block>>,
    }

    pub struct Structure {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub fields: Vec<Rc<Argument>>,
    }

    pub struct Enum {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub options: Vec<Rc<Identifier>>,
    }
}

impl Node for Module {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Circuit {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let name = Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())));
        let arguments: Vec<Rc<NodeKind>> = self
            .arguments
            .iter()
            .map(|arg| Rc::new(NodeKind::from(&Declaration::PatternArgument(arg.clone()))))
            .collect();
        let ty = Rc::new(NodeKind::from(&self.ty));
        let body = if let Some(body) = &self.body {
            vec![Rc::new(NodeKind::from(&Statement::Block(body.clone())))]
        } else {
            vec![]
        };
        vec![name]
            .into_iter()
            .chain(arguments)
            .chain(vec![ty])
            .chain(body)
            .rev()
            .collect()
    }
}

impl Circuit {
    #[must_use = "This method returns the name of the circuit"]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl Node for Structure {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Enum {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
