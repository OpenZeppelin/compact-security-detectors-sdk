use std::rc::Rc;

use crate::{ast_enum, ast_nodes, ast_nodes_impl};

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
        @symbol Const(Rc<Const>),
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
        pub then_branch: Statement,
        pub else_branch: Option<Statement>,
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

ast_nodes_impl! {
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
                Rc::new(NodeKind::from(&self.then_branch)),
            ];
            if let Some(else_branch) = &self.else_branch {
                children.push(Rc::new(NodeKind::from(else_branch)));
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
    impl Node for Block {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            self.statements
                .iter()
                .map(|stmt| Rc::new(NodeKind::from(stmt)))
                .collect()
        }
    }
}

impl SymbolNode for Var {
    fn name(&self) -> String {
        self.ident.name.clone()
    }
    fn type_expr(&self) -> Option<Expression> {
        self.ty_.clone().or_else(|| Some(self.value.clone()))
    }
}

impl SymbolNode for Const {
    fn name(&self) -> String {
        self.pattern.location().source.clone()
    }

    fn type_expr(&self) -> Option<Expression> {
        match &self.ty {
            Some(ty) => Some(Expression::TypeExpression(ty.clone())),
            None => Some(self.value.clone()),
        }
    }
}

impl Assert {
    #[must_use]
    pub fn message(&self) -> Option<String> {
        self.msg.as_ref().map(|msg| msg.value.clone())
    }
}

impl For {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn upper_bound_nat(&self) -> Option<u64> {
        if self.limit.is_some() {
            if let Some(Expression::Literal(Literal::Nat(nat))) = self.limit.as_ref() {
                return Some(nat.value);
            }
            return None;
        }
        let (_, end) = self.range.as_ref().unwrap();
        Some(end.value)
    }
}
