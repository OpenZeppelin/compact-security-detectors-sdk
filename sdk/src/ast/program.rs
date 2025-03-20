#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_node, ast_node_impl};

use super::{
    declaration::{Constructor, Declaration},
    definition::{Circuit, Definition, Module},
    directive::Directive,
    node::{Node, NodeKind, SameScopeNode},
};

ast_node! {
    pub struct Program {
        pub directives: Vec<Directive>,
        pub declarations: Vec<Declaration>,
        pub definitions: Vec<Definition>,
        pub modules: Vec<Rc<Module>>,
    }
}

impl From<&Rc<Program>> for NodeKind {
    fn from(program: &Rc<Program>) -> Self {
        NodeKind::SameScopeNode(SameScopeNode::Composite(program.clone()))
    }
}

ast_node_impl! {
    impl Node for Program {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let modules: Vec<Rc<NodeKind>> = self
                .modules
                .iter()
                .map(|m| Rc::new(NodeKind::from(&Definition::Module(m.clone()))))
                .collect();
            let definitions: Vec<Rc<NodeKind>> = self
                .definitions
                .iter()
                .map(|d| Rc::new(NodeKind::from(d)))
                .collect();
            let declarations: Vec<Rc<NodeKind>> = self
                .declarations
                .iter()
                .map(|d| Rc::new(NodeKind::from(d)))
                .collect();
            let directives: Vec<Rc<NodeKind>> = self
                .directives
                .iter()
                .map(|d| Rc::new(NodeKind::from(d)))
                .collect();
            modules
                .into_iter()
                .chain(definitions)
                .chain(declarations)
                .chain(directives)
                .collect()
        }
    }
}

impl Program {
    #[must_use = "Use this function to get the circuits in the program file"]
    pub fn circuits(&self) -> Vec<Rc<Circuit>> {
        self.definitions
            .iter()
            .filter(|d| matches!(d, Definition::Circuit(_)))
            .filter_map(|d| {
                if let Definition::Circuit(circuit) = d {
                    Some(Rc::clone(circuit))
                } else {
                    None
                }
            })
            .collect()
    }

    #[must_use]
    pub fn constructors(&self) -> Vec<Rc<Constructor>> {
        self.declarations
            .iter()
            .filter(|d| matches!(d, Declaration::Constructor(_)))
            .filter_map(|d| {
                if let Declaration::Constructor(constructor) = d {
                    Some(Rc::clone(constructor))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum CompactNode {
    Directive(Directive),
    Declaration(Declaration),
    Definition(Definition),
    Module(Rc<Module>),
    Comment(String),
}
