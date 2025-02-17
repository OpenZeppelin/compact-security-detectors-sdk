use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

ast_enum! {
    pub enum Expression {
        Conditional(Rc<Conditional>),
        Binary(Rc<Binary>),
        Cast(Rc<Cast>),
        IndexAccess(Rc<IndexAccess>),
        MemberAccess(Rc<MemberAccess>),
        FunctionCall(Rc<FunctionCall>),
    }
}

ast_nodes! {
    pub struct Conditional {
        pub condition: Rc<Expression>,
        pub then_branch: Rc<Expression>,
        pub else_branch: Rc<Expression>,
    }

    pub struct Binary {
        pub left_operand: Rc<Expression>,
        pub right_operand: Rc<Expression>,
        pub operator: BinaryExpressionOperator,
    }

    pub struct Cast {
        pub expression: Rc<Expression>,
        pub target_type: String, //TODO what type?
    }

    pub struct IndexAccess {
        pub array: Rc<Expression>,
        pub index: Rc<Expression>,
    }

    pub struct MemberAccess {
        pub expression: Rc<Expression>,
        pub member: String,
    }

    pub struct FunctionCall {
        pub function: Rc<Expression>,
        pub arguments: Vec<Rc<Expression>>,
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
