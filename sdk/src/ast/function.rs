use std::rc::Rc;

use crate::{ast_enum, ast_nodes, ast_nodes_impl};

use super::{
    declaration::{GArgument, Pattern, PatternArgument},
    expression::{Expression, Identifier},
    node::{Node, NodeKind},
    statement::Block,
    ty::Type,
};

ast_enum! {
    pub enum Function {
        Named(Rc<NamedFunction>),
        Anonymous(Rc<AnonymousFunction>),
    }
}

ast_enum! {
    pub enum FunctionArgument {
        @raw Pattern(Pattern),
        PatternArgument(Rc<PatternArgument>),
    }
}

ast_nodes! {
    pub struct NamedFunction {
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<GArgument>>,
    }

    pub struct AnonymousFunction {
        pub arguments: Vec<FunctionArgument>,
        pub return_type: Option<Type>,
        pub body: Option<Rc<Block>>,
        pub expr_body: Option<Expression>,
    }
}

ast_nodes_impl! {
    impl Node for NamedFunction {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = vec![];
            res.push(Rc::new(NodeKind::from(&Expression::Identifier(
                self.name.clone(),
            ))));
            if let Some(generic_arguments) = &self.generic_parameters {
                for arg in generic_arguments {
                    res.push(Rc::new(NodeKind::from(&arg.clone())));
                }
            }
            res
        }
    }

    impl Node for AnonymousFunction {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = vec![];
            for arg in &self.arguments {
                res.push(Rc::new(NodeKind::from(arg)));
            }
            if let Some(return_type) = &self.return_type {
                res.push(Rc::new(NodeKind::from(&return_type.clone())));
            }
            res
        }
    }
}

impl NamedFunction {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name.name
    }
}
