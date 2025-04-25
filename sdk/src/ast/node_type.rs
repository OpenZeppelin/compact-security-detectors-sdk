use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::ast::literal::{Bool, Nat};

#[allow(clippy::wildcard_imports)]
use super::{
    declaration::*,
    definition::Definition,
    directive::Directive,
    expression::*,
    function::{Function, FunctionArgument},
    literal::Literal,
    node::{Location, Node, NodeKind, SameScopeNode},
    program::Program,
    statement::*,
    ty::{Type, VectorSize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Program(Rc<Program>),
    Statement(Statement),
    Declaration(Declaration),
    Definition(Definition),
    Directive(Directive),
    Expression(Expression),
    Function(Function),
    FunctionArgument(FunctionArgument),
    Literal(Literal),
    Type(Type),
    VectorSize(VectorSize),
    Pattern(Pattern),
    GArgument(GArgument),
    StructExprArg(StructExprArg),
    StructArgument(StructArgument),
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
            NodeType::FunctionArgument(node) => node.id(),
            NodeType::Literal(node) => node.id(),
            NodeType::Type(node) => node.id(),
            NodeType::VectorSize(node) => node.id(),
            NodeType::Pattern(node) => node.id(),
            NodeType::GArgument(node) => node.id(),
            NodeType::StructExprArg(node) => node.id(),
            NodeType::StructArgument(node) => node.id(),
        }
    }

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
            NodeType::FunctionArgument(node) => node.location(),
            NodeType::Literal(node) => node.location(),
            NodeType::Pattern(node) => node.location(),
            NodeType::GArgument(node) => node.location(),
            NodeType::StructExprArg(node) => node.location(),
            NodeType::Type(node) => node.location(),
            NodeType::VectorSize(node) => node.location(),
            NodeType::StructArgument(node) => node.location(),
        }
    }

    #[must_use]
    pub fn children(&self) -> Vec<NodeType> {
        let node_children: Vec<Rc<NodeKind>> = match self {
            NodeType::Program(node) => node.children(),
            NodeType::Statement(node) => node.children(),
            NodeType::Declaration(node) => node.children(),
            NodeType::Definition(node) => node.children(),
            NodeType::Directive(node) => node.children(),
            NodeType::Expression(node) => node.children(),
            NodeType::Function(node) => node.children(),
            NodeType::Literal(node) => node.children(),
            NodeType::Type(node) => node.children(),
            NodeType::Pattern(node) => node.children(),
            NodeType::GArgument(node) => node.children(),
            NodeType::StructExprArg(node) => node.children(),
            NodeType::FunctionArgument(node) => node.children(),
            NodeType::VectorSize(node) => node.children(),
            NodeType::StructArgument(node) => node.children(),
        };
        node_children
            .into_iter()
            .map(convert_nodekind_to_nodetype)
            .collect()
    }
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
fn convert_nodekind_to_nodetype(node_kind: Rc<NodeKind>) -> NodeType {
    match &*node_kind {
        NodeKind::NewScope(node_rc) => {
            let node_rc = node_rc.clone();
            if let Ok(block_node) = Rc::downcast::<Block>(node_rc.clone()) {
                return NodeType::Statement(Statement::Block(block_node));
            }
            if let Ok(func_node) = Rc::downcast::<Function>(node_rc.clone()) {
                return NodeType::Function((*func_node).clone());
            }
            if let Ok(program_node) = Rc::downcast::<Program>(node_rc.clone()) {
                return NodeType::Program(program_node);
            }
            panic!("Cannot convert NewScope node to NodeType: {node_rc:?}",);
        }
        NodeKind::SameScopeNode(SameScopeNode::Symbol(node_rc)) => {
            let node_rc = node_rc.clone();
            if let Ok(var_node) = Rc::downcast::<Var>(node_rc.clone()) {
                return NodeType::Statement(Statement::Var(var_node));
            }
            if let Ok(const_node) = Rc::downcast::<Const>(node_rc.clone()) {
                return NodeType::Statement(Statement::Const(const_node));
            }
            if let Ok(ident_node) = Rc::downcast::<Identifier>(node_rc.clone()) {
                return NodeType::Expression(Expression::Identifier(ident_node));
            }
            if let Ok(func_arg_node) = Rc::downcast::<FunctionArgument>(node_rc.clone()) {
                return NodeType::FunctionArgument((*func_arg_node).clone());
            }
            if let Ok(struct_arg_node) =
                Rc::downcast::<StructArgument>(Rc::new(node_rc.clone()) as Rc<dyn std::any::Any>)
            {
                return NodeType::StructArgument((*struct_arg_node).clone());
            }
            panic!("Cannot convert SameScopeNode to NodeType: {node_rc:?}",);
        }
        NodeKind::SameScopeNode(SameScopeNode::Composite(node_rc)) => {
            let node_rc = node_rc.clone();
            if let Ok(assign_node) = Rc::downcast::<Assign>(node_rc.clone()) {
                return NodeType::Statement(Statement::Assign(assign_node));
            }
            if let Ok(assert_node) = Rc::downcast::<Assert>(node_rc.clone()) {
                return NodeType::Statement(Statement::Assert(assert_node));
            }
            if let Ok(seq_node) = Rc::downcast::<Sequence>(node_rc.clone()) {
                return NodeType::Expression(Expression::Sequence(seq_node));
            }
            if let Ok(if_node) = Rc::downcast::<If>(node_rc.clone()) {
                return NodeType::Statement(Statement::If(if_node));
            }
            if let Ok(for_node) = Rc::downcast::<For>(node_rc.clone()) {
                return NodeType::Statement(Statement::For(for_node));
            }
            if let Ok(ret_node) = Rc::downcast::<Return>(node_rc.clone()) {
                return NodeType::Statement(Statement::Return(ret_node));
            }
            if let Ok(cond_node) = Rc::downcast::<Conditional>(node_rc.clone()) {
                return NodeType::Expression(Expression::Conditional(cond_node));
            }
            if let Ok(bin_node) = Rc::downcast::<Binary>(node_rc.clone()) {
                return NodeType::Expression(Expression::Binary(bin_node));
            }
            if let Ok(unary_node) = Rc::downcast::<Unary>(node_rc.clone()) {
                return NodeType::Expression(Expression::Unary(unary_node));
            }
            if let Ok(cast_node) = Rc::downcast::<Cast>(node_rc.clone()) {
                return NodeType::Expression(Expression::Cast(cast_node));
            }
            if let Ok(disclose_node) = Rc::downcast::<Disclose>(node_rc.clone()) {
                return NodeType::Expression(Expression::Disclose(disclose_node));
            }
            if let Ok(idx_node) = Rc::downcast::<IndexAccess>(node_rc.clone()) {
                return NodeType::Expression(Expression::IndexAccess(idx_node));
            }
            if let Ok(map_node) = Rc::downcast::<Map>(node_rc.clone()) {
                return NodeType::Expression(Expression::Map(map_node));
            }
            if let Ok(member_node) = Rc::downcast::<MemberAccess>(node_rc.clone()) {
                return NodeType::Expression(Expression::MemberAccess(member_node));
            }
            if let Ok(fold_node) = Rc::downcast::<Fold>(node_rc.clone()) {
                return NodeType::Expression(Expression::Fold(fold_node));
            }
            if let Ok(call_node) = Rc::downcast::<FunctionCall>(node_rc.clone()) {
                return NodeType::Expression(Expression::FunctionCall(call_node));
            }
            if let Ok(struct_expr_node) = Rc::downcast::<StructExpr>(node_rc.clone()) {
                return NodeType::Expression(Expression::Struct(struct_expr_node));
            }
            if let Ok(struct_named_field_node) = Rc::downcast::<StructNamedField>(node_rc.clone()) {
                return NodeType::StructExprArg(StructExprArg::NamedField(struct_named_field_node));
            }
            if let Ok(expr_node) = Rc::downcast::<Expression>(node_rc.clone()) {
                return NodeType::Expression((*expr_node).clone());
            }
            if let Ok(stmt_node) = Rc::downcast::<Statement>(node_rc.clone()) {
                return NodeType::Statement((*stmt_node).clone());
            }
            if let Ok(type_node) = Rc::downcast::<Type>(node_rc.clone()) {
                return NodeType::Type((*type_node).clone());
            }
            if let Ok(literal_node) = Rc::downcast::<Literal>(node_rc.clone()) {
                return NodeType::Literal((*literal_node).clone());
            }
            if let Ok(nat_node) = Rc::downcast::<Nat>(node_rc.clone()) {
                return NodeType::Literal(Literal::Nat(nat_node));
            }
            if let Ok(bool_node) = Rc::downcast::<Bool>(node_rc.clone()) {
                return NodeType::Literal(Literal::Bool(bool_node));
            }
            if let Ok(pattern_node) = Rc::downcast::<Pattern>(node_rc.clone()) {
                return NodeType::Pattern((*pattern_node).clone());
            }
            if let Ok(garg_node) = Rc::downcast::<GArgument>(node_rc.clone()) {
                return NodeType::GArgument((*garg_node).clone());
            }
            if let Ok(struct_expr_arg_node) = Rc::downcast::<StructExprArg>(node_rc.clone()) {
                return NodeType::StructExprArg((*struct_expr_arg_node).clone());
            }
            panic!("Cannot convert Composite node to NodeType: {node_rc:?}",);
        }
    }
}
