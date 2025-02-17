use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::expression::Expression;

ast_enum! {
    pub enum Statement {
        Assign(Rc<Assign>),
        Assert(Rc<Assert>),
        Return(Rc<Return>),
        Block(Rc<Block>),
        If(Rc<If>),
        Var(Rc<Var>),
    }
}

ast_nodes! {
    pub struct Assign {
        pub target: Rc<Expression>,
        pub value: Rc<Expression>,
        pub operator: AssignOperator,
    }

    pub struct Return {
        pub value: Option<Rc<Expression>>,
    }

    pub struct If {
        pub condition: Rc<Expression>,
        pub then_branch: Rc<Statement>,
        pub else_branch: Option<Rc<Statement>>,
    }

    pub struct For {
        pub init: Option<Rc<Statement>>,
        pub condition: Option<Rc<Expression>>,
        pub update: Option<Rc<Statement>>,
        pub body: Rc<Statement>,
    }

    pub struct Assert {
        pub condition: Rc<Expression>,
    }

    pub struct Var {
        pub name: String,
        pub value: Rc<Expression>,
        pub ty_: Option<Rc<Expression>>,
    }

    pub struct Block {
        pub statements: Vec<Rc<Statement>>,
    }
}

ast_enum! {
    pub enum AssignOperator {
        Simple,
        Add,
        Sub,
    }
}
