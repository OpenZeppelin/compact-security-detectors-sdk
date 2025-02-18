use std::rc::Rc;

use crate::{
    ast_enum, ast_nodes,
    passes::{Node, NodeKind, SameScopeNode, SymbolNode},
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
        Literal(Literal),
        Identifier(Rc<Identifier>),
    }
}

impl From<&Expression> for NodeKind {
    fn from(expr: &Expression) -> Self {
        match expr {
            Expression::Conditional(cond) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(cond.clone()))
            }
            Expression::Binary(binary) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(binary.clone()))
            }
            Expression::Cast(cast) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(cast.clone()))
            }
            Expression::IndexAccess(index_access) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(index_access.clone()))
            }
            Expression::MemberAccess(member_access) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(member_access.clone()))
            }
            Expression::FunctionCall(function_call) => {
                NodeKind::SameScopeNode(SameScopeNode::Composite(function_call.clone()))
            }
            Expression::Literal(literal) => literal.into(),
            Expression::Identifier(identifier) => {
                NodeKind::SameScopeNode(SameScopeNode::Symbol(identifier.clone()))
            }
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
        pub target_type: Rc<Expression>, //TODO what type?
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

ast_enum! {
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
}
