use std::rc::Rc;

use crate::{
    ast_enum, ast_nodes,
    passes::{Node, NodeKind, SameScopeNode, SymbolNode, Type},
};

use super::literal::Literal;

ast_enum! {
    pub enum Expression {
        Conditional(Rc<Conditional>),
        Binary(Rc<Binary>),
        Cast(Rc<Cast>),
        IndexAccess(Rc<IndexAccess>),
        MemberAccess(Rc<MemberAccess>),
        FunctionCall(Rc<FunctionCall>),
        TypeExpressoin(Rc<Type>),
        @raw Literal(Literal),
        @symbol Identifier(Rc<Identifier>),
    }
}

impl From<&NodeKind> for Expression {
    fn from(node: &NodeKind) -> Self {
        match node {
            NodeKind::SameScopeNode(SameScopeNode::Composite(cond)) => {
                if let Some(cond) = cond.as_any().downcast_ref::<Rc<Conditional>>() {
                    Expression::Conditional(cond.clone())
                } else if let Some(binary) = cond.as_any().downcast_ref::<Rc<Binary>>() {
                    Expression::Binary(binary.clone())
                } else if let Some(cast) = cond.as_any().downcast_ref::<Rc<Cast>>() {
                    Expression::Cast(cast.clone())
                } else if let Some(index_access) = cond.as_any().downcast_ref::<Rc<IndexAccess>>() {
                    Expression::IndexAccess(index_access.clone())
                } else if let Some(member_access) = cond.as_any().downcast_ref::<Rc<MemberAccess>>()
                {
                    Expression::MemberAccess(member_access.clone())
                } else if let Some(function_call) = cond.as_any().downcast_ref::<Rc<FunctionCall>>()
                {
                    Expression::FunctionCall(function_call.clone())
                } else if let Some(type_expr) = cond.as_any().downcast_ref::<Rc<Type>>() {
                    Expression::TypeExpressoin(type_expr.clone())
                } else {
                    unreachable!()
                }
            }
            NodeKind::SameScopeNode(SameScopeNode::Symbol(identifier)) => {
                if let Some(identifier) = identifier.as_any().downcast_ref::<Rc<Identifier>>() {
                    Expression::Identifier(identifier.clone())
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

ast_nodes! {
    /// E.g. `const a = bool ? 1 : 2`
    pub struct Conditional {
        pub condition: Rc<Expression>,
        pub then_branch: Rc<Expression>,
        pub else_branch: Rc<Expression>,
    }

    pub struct Binary {
        pub left_operand: Expression,
        pub right_operand: Expression,
        pub operator: BinaryExpressionOperator,
    }

    pub struct Cast {
        pub expression: Rc<Expression>,
        pub target_type: Rc<Expression>,
    }

    pub struct IndexAccess {
        pub array: Rc<Expression>,
        pub index: Rc<Expression>,
    }

    pub struct MemberAccess {
        pub base: Rc<Expression>,
        pub member: String,
    }

    pub struct FunctionCall {
        pub function: Rc<Expression>,
        pub arguments: Vec<Rc<Expression>>,
    }

    pub struct Identifier {
        pub name: String,
    }
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum BinaryExpressionOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Shl,
    Shr,
}

impl Node for Conditional {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&*self.condition)),
            Rc::new(NodeKind::from(&*self.then_branch)),
            Rc::new(NodeKind::from(&*self.else_branch)),
        ]
    }
}

impl Node for Binary {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.left_operand)),
            Rc::new(NodeKind::from(&self.right_operand)),
        ]
    }
}

impl Node for Cast {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&*self.expression)),
            Rc::new(NodeKind::from(&*self.target_type)),
        ]
    }
}

impl Node for IndexAccess {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&*self.array)),
            Rc::new(NodeKind::from(&*self.index)),
        ]
    }
}

impl Node for MemberAccess {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![Rc::new(NodeKind::from(&*self.base))]
    }
}

impl Node for FunctionCall {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![Rc::new(NodeKind::from(&*self.function))];
        children.extend(
            self.arguments
                .iter()
                .map(|arg| Rc::new(NodeKind::from(&**arg))),
        );
        children
    }
}

impl Node for Identifier {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl SymbolNode for Identifier {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn type_expr(&self) -> Option<&Expression> {
        None
    }
}
