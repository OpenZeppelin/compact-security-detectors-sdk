use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::{
    declaration::{Declaration, GArgument, Pattern},
    definition::Definition,
    directive::Directive,
    expression::{Expression, StructExprArg},
    function::Function,
    literal::Literal,
    node::Location,
    program::Program,
    statement::Statement,
    ty::Type,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum NodeType {
    Program(Rc<Program>),
    Statement(Statement),
    Declaration(Declaration),
    Definition(Definition),
    Directive(Directive),
    Expression(Expression),
    Function(Function),
    Literal(Literal),
    Type(Type),
    Pattern(Pattern),
    GArgument(GArgument),
    StructExprArg(StructExprArg),
}

impl NodeType {
    #[must_use]
    pub fn id(&self) -> u32 {
        match self {
            NodeType::Program(node) => node.id,
            NodeType::Statement(node) => node.id(),
            NodeType::Declaration(node) => node.id(),
            NodeType::Definition(node) => node.id(),
            NodeType::Directive(node) => node.id(),
            NodeType::Expression(node) => node.id(),
            NodeType::Function(node) => node.id(),
            NodeType::Literal(node) => node.id(),
            NodeType::Type(node) => node.id(),
            NodeType::Pattern(node) => node.id(),
            NodeType::GArgument(node) => node.id(),
            NodeType::StructExprArg(node) => node.id(),
        }
    }
}

impl NodeType {
    #[must_use]
    pub fn location(&self) -> Location {
        match self {
            NodeType::Program(node) => node.location.clone(),
            NodeType::Statement(node) => node.location(),
            NodeType::Declaration(node) => node.location(),
            NodeType::Definition(node) => node.location(),
            NodeType::Directive(node) => node.location(),
            NodeType::Expression(node) => node.location(),
            NodeType::Function(node) => node.location(),
            NodeType::Literal(node) => node.location(),
            NodeType::Type(node) => node.location(),
            NodeType::Pattern(node) => node.location(),
            NodeType::GArgument(node) => node.location(),
            NodeType::StructExprArg(node) => node.location(),
        }
    }
}
