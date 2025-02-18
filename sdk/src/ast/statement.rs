use std::rc::Rc;

use crate::{
    ast_enum, ast_nodes,
    passes::{Node, NodeKind, SameScopeNode},
};

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

impl From<&Statement> for NodeKind {
    fn from(stmt: &Statement) -> Self {
        match stmt {
            Statement::Assign(assign) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(assign.clone()))
            }
            Statement::Assert(assert) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(assert.clone()))
            }
            Statement::Return(r#return) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(r#return.clone()))
            }
            Statement::Block(block) => NodeKind::NewScope(block.clone()),
            Statement::If(r#if) => NodeKind::SameScopeNode(SameScopeNode::Composite(r#if.clone())),
            Statement::Var(var) => NodeKind::SameScopeNode(SameScopeNode::Composite(var.clone())),
        }
    }
}

ast_nodes! {
    pub struct Assign {
        pub target: Expression,
        pub value: Expression,
        pub operator: AssignOperator,
    }

    pub struct Return {
        pub value: Option<Expression>,
    }

    pub struct If {
        pub condition: Expression,
        pub then_branch: Statement,
        pub else_branch: Option<Statement>,
    }

    pub struct For {
        pub init: Option<Statement>,
        pub condition: Option<Expression>,
        pub update: Option<Statement>,
        pub body: Statement,
    }

    pub struct Assert {
        pub condition: Expression,
    }

    pub struct Var {
        pub name: String,
        pub value: Expression,
        pub ty_: Option<Expression>,
    }

    pub struct Block {
        pub statements: Vec<Statement>,
    }
}

ast_enum! {
    pub enum AssignOperator {
        Simple,
        Add,
        Sub,
    }
}

impl Node for Assign {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.target)),
            Rc::new(NodeKind::from(&self.value)),
        ]
    }
}

impl Node for Return {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        self.value
            .iter()
            .map(|expr| Rc::new(NodeKind::from(expr)))
            .collect()
    }
}

impl Node for If {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![
            Rc::new(NodeKind::from(&self.condition)),
            Rc::new(NodeKind::from(&self.then_branch)),
        ];
        if let Some(else_branch) = &self.else_branch {
            children.push(Rc::new(NodeKind::from(else_branch)));
        }
        children
    }
}

impl Node for Assert {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![Rc::new(NodeKind::from(&self.condition))]
    }
}

impl Node for Var {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![Rc::new(NodeKind::from(&self.value))]
    }
}

impl Node for Block {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        self.statements
            .iter()
            .map(|stmt| Rc::new(NodeKind::from(stmt)))
            .collect()
    }
}
