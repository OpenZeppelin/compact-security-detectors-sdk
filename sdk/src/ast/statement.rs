use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::{
    declaration::Pattern,
    expression::{Expression, Identifier, Sequence},
    literal::{Literal, Nat, Str},
    node::{Node, NodeKind, SymbolNode},
    ty::Type,
};

ast_enum! {
    pub enum Statement {
        Assign(Rc<Assign>),
        Assert(Rc<Assert>),
        @scope Block(Rc<Block>),
        Const(Rc<Const>),
        ExpressionSequence(Rc<Sequence>),
        @raw Expression(Expression),
        If(Rc<If>),
        For(Rc<For>),
        @symbol Var(Rc<Var>),
        Return(Rc<Return>),
    }
}

ast_nodes! {
    pub struct Assign {
        pub target: Expression,
        pub value: Expression,
        pub operator: AssignOperator,
    }

    pub struct Assert {
        pub condition: Expression,
        pub msg: Option<Rc<Str>>,
    }

    pub struct Block {
        pub statements: Vec<Statement>,
    }

    pub struct Const {
        pub pattern: Pattern,
        pub value: Expression,
        pub ty: Option<Type>,
    }

    pub struct If {
        pub condition: Expression,
        pub then_branch: Rc<Block>,
        pub else_branch: Option<Rc<Block>>,
    }

    pub struct For {
        pub counter: Rc<Identifier>,
        pub range: Option<(Rc<Nat>, Rc<Nat>)>,
        pub limit: Option<Expression>,
        pub body: Rc<Block>,
    }

    pub struct Var {
        pub ident: Rc<Identifier>,
        pub value: Expression,
        pub ty_: Option<Expression>,
    }
    pub struct Return {
        pub value: Option<Expression>,
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum AssignOperator {
    Simple,
    Add,
    Sub,
}

impl Node for Assign {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.target)),
            Rc::new(NodeKind::from(&self.value)),
        ]
    }
}

impl Node for Const {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.pattern)),
            Rc::new(NodeKind::from(&self.value.clone())),
        ]
    }
}

impl Node for Return {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        if let Some(value) = &self.value {
            vec![Rc::new(NodeKind::from(&value.clone()))]
        } else {
            vec![]
        }
    }
}

impl Node for If {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![
            Rc::new(NodeKind::from(&self.condition)),
            Rc::new(NodeKind::from(&Statement::Block(self.then_branch.clone()))),
        ];
        if let Some(else_branch) = &self.else_branch {
            children.push(Rc::new(NodeKind::from(&Statement::Block(
                else_branch.clone(),
            ))));
        }
        children
    }
}

impl Node for For {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![Rc::new(NodeKind::from(&Expression::Identifier(
            self.counter.clone(),
        )))];
        if let Some((start, end)) = &self.range {
            children.push(Rc::new(NodeKind::from(&Literal::Nat(start.clone()))));
            children.push(Rc::new(NodeKind::from(&Literal::Nat(end.clone()))));
        }
        if let Some(limit) = &self.limit {
            children.push(Rc::new(NodeKind::from(limit)));
        }
        children.push(Rc::new(NodeKind::from(&Statement::Block(
            self.body.clone(),
        ))));
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

impl SymbolNode for Var {
    fn id(&self) -> u128 {
        self.ident.id
    }
    fn name(&self) -> String {
        self.ident.name.clone()
    }
    fn type_expr(&self) -> Option<Expression> {
        self.ty_.clone()
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
