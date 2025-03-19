use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

use super::{
    function::Function,
    literal::Literal,
    node::{Node, NodeKind, SameScopeNode, SymbolNode},
    ty::Type,
};

ast_enum! {
    pub enum Expression {
        Conditional(Rc<Conditional>),
        Binary(Rc<Binary>),
        Unary(Rc<Unary>),
        Cast(Rc<Cast>),
        Disclose(Rc<Disclose>),
        IndexAccess(Rc<IndexAccess>),
        Sequence(Rc<Sequence>),
        Map(Rc<Map>),
        Fold(Rc<Fold>),
        MemberAccess(Rc<MemberAccess>),
        FunctionCall(Rc<FunctionCall>),
        Struct(Rc<StructExpr>),
        @raw Function(Function),
        @raw TypeExpression(Type),
        @raw Default(Type),
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
                } else if let Some(type_expr) = cond.as_any().downcast_ref::<Type>() {
                    Expression::TypeExpression(type_expr.clone())
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

ast_enum! {
    pub enum StructExprArg {
        @raw Expression(Expression),
        NamedField(Rc<StructNamedField>),
        @raw Update(Expression),
    }
}

ast_nodes! {
    /// E.g. `const a = bool ? 1 : 2`
    pub struct Conditional {
        pub condition: Expression,
        pub then_branch: Expression,
        pub else_branch: Expression,
    }

    pub struct Binary {
        pub left: Expression,
        pub right: Expression,
        pub operator: BinaryExpressionOperator,
    }

    pub struct Unary {
        pub operand: Expression,
        pub operator: UnaryExpressionOperator,
    }

    pub struct Cast {
        pub expression: Expression,
        pub target_type: Type,
    }

    pub struct Disclose {
        pub expression: Expression,
    }

    pub struct IndexAccess {
        pub base: Expression,
        pub index: Expression,
    }

    pub struct Map {
        pub function: Function,
        pub expressions: Vec<Expression>,
    }

    pub struct MemberAccess {
        pub base: Expression,
        pub member: Rc<Identifier>,
        pub arguments: Option<Vec<Expression>>,
    }

    pub struct Fold {
        pub function: Function,
        pub initial_value: Expression,
        pub expressions: Vec<Expression>,
    }

    pub struct FunctionCall {
        pub function: Expression,
        pub arguments: Vec<Expression>,
    }

    pub struct Sequence {
        pub expressions: Vec<Expression>,
    }

    pub struct Identifier {
        pub name: String,
    }

    pub struct StructExpr {
        pub ty: Type,
        pub args: Vec<StructExprArg>,
    }

    pub struct StructNamedField {
        pub name: Rc<Identifier>,
        pub value: Expression,
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

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum UnaryExpressionOperator {
    Neg,
    Not,
}

impl Node for Conditional {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.condition)),
            Rc::new(NodeKind::from(&self.then_branch)),
            Rc::new(NodeKind::from(&self.else_branch)),
        ]
    }
}

impl Node for Binary {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.left)),
            Rc::new(NodeKind::from(&self.right)),
        ]
    }
}

impl Node for Unary {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![Rc::new(NodeKind::from(&self.operand))]
    }
}

impl Node for Cast {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.expression)),
            Rc::new(NodeKind::from(&self.target_type)),
        ]
    }
}

impl Node for Disclose {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![Rc::new(NodeKind::from(&self.expression))]
    }
}

impl Node for IndexAccess {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&self.base)),
            Rc::new(NodeKind::from(&self.index)),
        ]
    }
}

impl Node for Sequence {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        self.expressions
            .iter()
            .map(|expr| Rc::new(NodeKind::from(expr)))
            .collect()
    }
}

impl Node for Map {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![Rc::new(NodeKind::from(&self.function))];
        children.extend(
            self.expressions
                .iter()
                .map(|expr| Rc::new(NodeKind::from(expr))),
        );
        children
    }
}

impl Node for MemberAccess {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![Rc::new(NodeKind::from(&self.base))]
    }
}

impl Node for Fold {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![Rc::new(NodeKind::from(&self.function))];
        children.push(Rc::new(NodeKind::from(&self.initial_value)));
        children.extend(
            self.expressions
                .iter()
                .map(|expr| Rc::new(NodeKind::from(expr))),
        );
        children
    }
}

impl Node for FunctionCall {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![Rc::new(NodeKind::from(&self.function))];
        children.extend(
            self.arguments
                .iter()
                .map(|arg| Rc::new(NodeKind::from(arg))),
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
    fn id(&self) -> u128 {
        self.id
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn type_expr(&self) -> Option<Expression> {
        None
    }
}

impl Node for StructExpr {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = vec![Rc::new(NodeKind::from(&self.ty))];
        children.extend(self.args.iter().map(|field| Rc::new(NodeKind::from(field))));
        children
    }
}

impl Node for StructNamedField {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone()))),
            Rc::new(NodeKind::from(&self.value)),
        ]
    }
}
