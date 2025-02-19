#![allow(clippy::all)]
#![allow(warnings)]
#![feature(trait_upcasting)]
use anyhow::{anyhow, Result};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::{
    expression::{BinaryExpressionOperator, Expression},
    literal::Literal,
    statement::Statement,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    String,
    Vector(u128, Box<Type>),
    #[default]
    Unknown,
}

#[derive(Debug)]
enum TypeError {
    Undefined(String),
    Mismatch(Type, Type),
}

pub enum NodeKind {
    SameScopeNode(SameScopeNode),
    NewScope(Rc<dyn Node>),
}

pub trait NodeSymbolNode: Node + SymbolNode + Any {}

impl<T> NodeSymbolNode for T where T: Node + SymbolNode + Any {}

impl<'a> From<&'a Rc<dyn NodeSymbolNode>> for &'a dyn Node {
    fn from(node: &'a Rc<dyn NodeSymbolNode>) -> Self {
        node as &'a dyn Node
    }
}

impl Node for Rc<dyn NodeSymbolNode> {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        match self.as_any().downcast_ref::<SameScopeNode>() {
            Some(SameScopeNode::Composite(comp_node)) => comp_node.children(),
            _ => vec![],
        }
    }
}

impl dyn NodeSymbolNode {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum SameScopeNode {
    Symbol(Rc<dyn NodeSymbolNode>),
    Composite(Rc<dyn Node>),
}

pub trait Node: Any {
    fn children(&self) -> Vec<Rc<NodeKind>>;
}

impl dyn Node {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait SymbolNode {
    fn name(&self) -> String;
    fn type_expr(&self) -> Option<&Expression> {
        None
    }
}

impl SymbolNode for crate::ast::statement::Var {
    fn name(&self) -> String {
        self.name.name.clone()
    }
    fn type_expr(&self) -> Option<&Expression> {
        self.ty_.as_ref().or(Some(&self.value))
    }
}

#[derive(Default, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: Type,
}

pub struct SymbolTable {
    pub symbols: RefCell<HashMap<String, Symbol>>,
    pub parent: Option<Rc<SymbolTable>>,
    pub children: RefCell<Vec<Rc<SymbolTable>>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Rc<SymbolTable>>) -> Self {
        Self {
            symbols: RefCell::new(HashMap::new()),
            children: RefCell::new(Vec::new()),
            parent,
        }
    }

    #[allow(clippy::map_entry)]
    pub fn insert(&self, name: String, symbol: Symbol) -> Result<()> {
        let mut syms = self.symbols.borrow_mut();
        if syms.contains_key(&name) {
            Err(anyhow!("Symbol {name} already exists"))
        } else {
            syms.insert(name, symbol);
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        let syms = self.symbols.borrow();
        if let Some(sym) = syms.get(name) {
            Some(sym.clone())
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
}

pub fn build_symbol_table(
    node_kind: Rc<NodeKind>,
    parent: Option<Rc<SymbolTable>>,
) -> anyhow::Result<Rc<SymbolTable>> {
    let symbol_table = Rc::new(SymbolTable::new(parent));
    let nodes: Vec<Rc<NodeKind>> = if let NodeKind::NewScope(node) = node_kind.as_ref() {
        node.children()
    } else {
        vec![node_kind]
    };
    for node in &nodes {
        if let NodeKind::SameScopeNode(same) = node.as_ref() {
            if let SameScopeNode::Symbol(sym_node) = same {
                if let Some(var) = sym_node
                    .as_any()
                    .downcast_ref::<crate::ast::statement::Var>()
                {
                    let symbol_name = var.name();
                    if symbol_table.symbols.borrow().contains_key(&symbol_name) {
                        return Err(anyhow!("Symbol {symbol_name} already exists"));
                    }
                    let ty = if let Some(type_expr) = sym_node.type_expr() {
                        infer_expr(type_expr, &symbol_table)?
                    } else {
                        Type::Unknown
                    };
                    symbol_table.symbols.borrow_mut().insert(
                        symbol_name.clone(),
                        Symbol {
                            name: symbol_name,
                            ty,
                        },
                    );
                }
            }
        }
    }
    for node in nodes {
        match node.as_ref() {
            NodeKind::SameScopeNode(same) => {
                if let SameScopeNode::Composite(comp_node) = same {
                    for child in comp_node.children() {
                        let _ = build_symbol_table(child, Some(symbol_table.clone()))?;
                    }
                }
            }
            NodeKind::NewScope(node) => {
                let child_scope = build_symbol_table(
                    Rc::new(NodeKind::NewScope(node.clone())),
                    Some(symbol_table.clone()),
                )?;
                symbol_table.children.borrow_mut().push(child_scope);
            }
        }
    }
    Ok(symbol_table)
}

fn infer_expr(expr: &Expression, env: &Rc<SymbolTable>) -> Result<Type> {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Nat(_) => Ok(Type::Int),
            Literal::Bool(_) => Ok(Type::Bool),
            Literal::Str(_) => Ok(Type::String),
            Literal::Version(_) => Ok(Type::Unknown),
        },
        Expression::Binary(bin_expr) => {
            let left = infer_expr(&bin_expr.left_operand, env)?;
            let right = infer_expr(&bin_expr.right_operand, env)?;
            match bin_expr.operator {
                BinaryExpressionOperator::Add
                | BinaryExpressionOperator::Sub
                | BinaryExpressionOperator::Mul
                | BinaryExpressionOperator::Div
                | BinaryExpressionOperator::Mod
                | BinaryExpressionOperator::Pow
                | BinaryExpressionOperator::BitAnd
                | BinaryExpressionOperator::BitOr
                | BinaryExpressionOperator::BitXor
                | BinaryExpressionOperator::BitNot
                | BinaryExpressionOperator::Shl
                | BinaryExpressionOperator::Shr => {
                    if left == right {
                        Ok(left)
                    } else {
                        Err(anyhow!("Type mismatch"))
                    }
                }
                BinaryExpressionOperator::Eq
                | BinaryExpressionOperator::Ne
                | BinaryExpressionOperator::Lt
                | BinaryExpressionOperator::Le
                | BinaryExpressionOperator::Gt
                | BinaryExpressionOperator::Ge
                | BinaryExpressionOperator::And
                | BinaryExpressionOperator::Or => {
                    if left == right {
                        Ok(Type::Bool)
                    } else {
                        Err(anyhow!("Type mismatch"))
                    }
                }
            }
        }
        Expression::Conditional(conditional) => {
            let then_type = infer_expr(&conditional.then_branch, env)?;
            let else_type = infer_expr(&conditional.else_branch, env)?;
            if then_type == else_type {
                Ok(then_type)
            } else {
                Err(anyhow!("Type mismatch"))
            }
        }
        Expression::Cast(cast) => infer_expr(&cast.target_type, env),
        Expression::IndexAccess(index_access) => {
            let vec_type = infer_expr(&index_access.array, env)?;
            if let Type::Vector(_, ty) = vec_type {
                Ok(*ty)
            } else {
                Err(anyhow!("Type mismatch"))
            }
        }
        Expression::MemberAccess(member_access) => infer_expr(&member_access.base, env),
        Expression::FunctionCall(function_call) => infer_expr(&function_call.function, env),
        Expression::Identifier(identifier) => {
            let symbol = env
                .lookup(&identifier.name)
                .ok_or_else(|| anyhow!("Undefined identifier"))?;
            Ok(symbol.ty)
        }
    }
}

pub fn infer_types(stmt: &Statement, env: &Rc<SymbolTable>) -> Result<()> {
    match stmt {
        Statement::Assign(assign) => todo!(),
        Statement::Assert(assert) => todo!(),
        Statement::Return(_) => todo!(),
        Statement::Block(block) => todo!(),
        Statement::If(_) => todo!(),
        Statement::Var(var) => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{
        expression::{Binary, Identifier},
        literal::{Nat, Str},
        statement::{Block, Return, Var},
    };
    use anyhow::{anyhow, Result};

    use super::*;

    #[test]
    fn test_build_symbol_table() -> anyhow::Result<()> {
        let block_stmt = Block {
            statements: vec![
                Statement::Var(Rc::new(Var {
                    id: 0,
                    location: Default::default(),
                    name: Rc::new(Identifier {
                        id: 6,
                        location: Default::default(),
                        name: String::from("a"),
                    }),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 1,
                        location: Default::default(),
                    }))),
                    ty_: None,
                })),
                Statement::Var(Rc::new(Var {
                    id: 2,
                    location: Default::default(),
                    name: Rc::new(Identifier {
                        id: 6,
                        location: Default::default(),
                        name: String::from("b"),
                    }),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 3,
                        location: Default::default(),
                    }))),
                    ty_: None,
                })),
                Statement::Return(Rc::new(Return {
                    id: 4,
                    location: Default::default(),
                    value: Some(Expression::Binary(Rc::new(Binary {
                        id: 5,
                        location: Default::default(),
                        left_operand: Expression::Identifier(Rc::new(Identifier {
                            id: 6,
                            location: Default::default(),
                            name: String::from("a"),
                        })),
                        right_operand: Expression::Identifier(Rc::new(Identifier {
                            id: 7,
                            location: Default::default(),
                            name: String::from("b"),
                        })),
                        operator: BinaryExpressionOperator::Add,
                    }))),
                })),
            ],
            id: 8,
            location: Default::default(),
        };
        let symbol_table = build_symbol_table(
            Rc::new(NodeKind::from(&Statement::Block(Rc::new(block_stmt)))),
            None,
        )?;
        assert!(symbol_table.parent.is_none());
        assert_eq!(symbol_table.symbols.borrow().len(), 2);
        assert_eq!(symbol_table.children.borrow().len(), 0);
        let a = symbol_table.lookup("a").unwrap();
        let b = symbol_table.lookup("b").unwrap();
        Ok(())
    }

    #[test]
    fn test_infer_expr() -> Result<()> {
        let block_stmt = Block {
            statements: vec![
                Statement::Var(Rc::new(Var {
                    id: 0,
                    location: Default::default(),
                    name: Rc::new(Identifier {
                        id: 6,
                        location: Default::default(),
                        name: String::from("a"),
                    }),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 1,
                        location: Default::default(),
                    }))),
                    ty_: None,
                })),
                Statement::Var(Rc::new(Var {
                    id: 2,
                    location: Default::default(),
                    name: Rc::new(Identifier {
                        id: 6,
                        location: Default::default(),
                        name: String::from("b"),
                    }),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 3,
                        location: Default::default(),
                    }))),
                    ty_: None,
                })),
                Statement::Return(Rc::new(Return {
                    id: 4,
                    location: Default::default(),
                    value: Some(Expression::Binary(Rc::new(Binary {
                        id: 5,
                        location: Default::default(),
                        left_operand: Expression::Identifier(Rc::new(Identifier {
                            id: 6,
                            location: Default::default(),
                            name: String::from("a"),
                        })),
                        right_operand: Expression::Identifier(Rc::new(Identifier {
                            id: 7,
                            location: Default::default(),
                            name: String::from("b"),
                        })),
                        operator: BinaryExpressionOperator::Add,
                    }))),
                })),
            ],
            id: 8,
            location: Default::default(),
        };
        let symbol_table = build_symbol_table(
            Rc::new(NodeKind::from(&Statement::Block(Rc::new(block_stmt)))),
            None,
        )?;
        let a = symbol_table.lookup("a").unwrap();
        let b = symbol_table.lookup("b").unwrap();
        let expr = Expression::Binary(Rc::new(Binary {
            id: 5,
            location: Default::default(),
            left_operand: Expression::Identifier(Rc::new(Identifier {
                id: 6,
                location: Default::default(),
                name: String::from("a"),
            })),
            right_operand: Expression::Identifier(Rc::new(Identifier {
                id: 7,
                location: Default::default(),
                name: String::from("b"),
            })),
            operator: BinaryExpressionOperator::Add,
        }));
        let ty = infer_expr(&expr, &symbol_table)?;
        assert_eq!(ty, Type::Int);
        Ok(())
    }
}
