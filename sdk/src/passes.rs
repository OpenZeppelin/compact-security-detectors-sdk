#![allow(clippy::all)]
#![allow(warnings)]
use anyhow::{anyhow, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::{expression::Expression, statement::Statement};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
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

pub trait NodeSymbolNode: Node + SymbolNode {}

impl<T> NodeSymbolNode for T where T: Node + SymbolNode {}

pub enum SameScopeNode {
    Symbol(Rc<dyn NodeSymbolNode>),
    Composite(Rc<dyn Node>),
}

pub trait Node {
    fn children(&self) -> Vec<Rc<NodeKind>>;
}

pub trait SymbolNode {
    fn name(&self) -> Option<String>;
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
    let mut nodes = if let NodeKind::NewScope(node) = node_kind.as_ref() {
        node.children()
    } else {
        vec![node_kind]
    };
    while let Some(node) = nodes.pop() {
        match node.as_ref() {
            NodeKind::SameScopeNode(node) => match node {
                SameScopeNode::Symbol(node) => {
                    let symbol_name = node.name().ok_or_else(|| anyhow!("Missing symbol name"))?;
                    if symbol_table.symbols.borrow().contains_key(&symbol_name) {
                        return Err(anyhow!("Symbol {symbol_name} already exists"));
                    }
                    symbol_table.symbols.borrow_mut().insert(
                        symbol_name.clone(),
                        Symbol {
                            name: symbol_name,
                            ty: Type::Unknown,
                        },
                    );
                }
                SameScopeNode::Composite(node) => {
                    for child in node.children() {
                        nodes.push(child.clone());
                    }
                }
            },
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

// fn infer_expr(expr: &Expression, env: &Rc<SymbolTable>) -> Result<Type> {
//     match expr {
//         Expr::IntLiteral(_) => Ok(Type::Int),
//         Expr::BoolLiteral(_) => Ok(Type::Bool),
//         Expr::Var(name) => env
//             .lookup(name)
//             .ok_or_else(|| Err(anyhow!("Undefined variable: {name}")))
//             .map(|sym| Ok(sym.ty)),
//         &Expression::Binary(bin_expr) => {
//             let left = infer_expr(&bin_expr.left_operand, env)?;
//             let right = infer_expr(&bin_expr.right_operand, env)?;
//         }
//     }
// }

// pub fn infer_types(stmt: &Statement, env: &Rc<SymbolTable>) -> Result<()> {
//     match stmt {
//         Stmt::VarDecl { name, init } => {
//             let inferred = infer_expr(init, env)?;
//             {
//                 let mut syms = env.symbols.borrow_mut();
//                 if let Some(sym) = syms.get_mut(name) {
//                     sym.ty = inferred.clone();
//                 } else {
//                     return Err(anyhow!("Symbol {name} not found during type inference"));
//                 }
//             }
//             Ok(())
//         }
//         Statement::Block(stmts) => {
//             let block_env = Rc::new(SymbolTable::new(Some(env.clone())));
//             for s in stmts {
//                 infer_types(s, &block_env)?;
//             }
//             Ok(())
//         }
//     }
// }
