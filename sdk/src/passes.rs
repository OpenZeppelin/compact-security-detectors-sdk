#![allow(dead_code)]
use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter, Write},
    rc::Rc,
};

use crate::ast::{
    expression::{BinaryExpressionOperator, Expression},
    literal::Literal,
    node::{Location, NodeKind, SameScopeNode},
    ty::{Sum, Type, TypeBool, TypeNat, TypeString},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolTable {
    pub symbols: RefCell<HashMap<String, Option<Type>>>,
    pub id_type_map: RefCell<HashMap<u32, Option<Type>>>,
    #[serde(skip_serializing)]
    pub parent: Option<Rc<SymbolTable>>,
    pub children: RefCell<Vec<Rc<SymbolTable>>>,
}

impl SymbolTable {
    pub fn new(parent: Option<Rc<SymbolTable>>) -> Self {
        Self {
            symbols: RefCell::new(HashMap::new()),
            children: RefCell::new(Vec::new()),
            id_type_map: RefCell::new(HashMap::new()),
            parent,
        }
    }

    pub(crate) fn upsert(&self, id: u32, symbol: String, ty: Option<Type>) {
        self.symbols.borrow_mut().insert(symbol, ty.clone());
        self.id_type_map.borrow_mut().insert(id, ty);
    }

    #[allow(clippy::map_entry)]
    pub fn insert(&self, name: String, ty: Option<Type>) -> Result<()> {
        let mut syms = self.symbols.borrow_mut();
        if syms.contains_key(&name) {
            Err(anyhow!("Symbol {name} already exists"))
        } else {
            syms.insert(name, ty);
            Ok(())
        }
    }

    pub fn insert_by_id(&self, id: u32, ty: Option<Type>) -> Result<()> {
        let mut id_type_map = self.id_type_map.borrow_mut();
        if let std::collections::hash_map::Entry::Vacant(e) = id_type_map.entry(id) {
            e.insert(ty);
            Ok(())
        } else {
            Err(anyhow!("Symbol {id} already exists"))
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Type> {
        let syms = self.symbols.borrow();
        if let Some(sym) = syms.get(name) {
            sym.clone()
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }

    pub fn lookup_by_id(&self, id: u32) -> Option<Type> {
        let id_type_map = self.id_type_map.borrow();
        if let Some(sym) = id_type_map.get(&id) {
            sym.clone()
        } else if let Some(ref parent) = self.parent {
            parent.lookup_by_id(id)
        } else {
            None
        }
    }

    pub fn lookdown(&self, name: &str) -> Option<Type> {
        let syms = self.symbols.borrow();
        if let Some(sym) = syms.get(name) {
            sym.clone()
        } else {
            for child in self.children.borrow().iter() {
                if let Some(sym) = child.lookdown(name) {
                    return Some(sym);
                }
            }
            None
        }
    }

    pub fn lookdown_by_id(&self, id: u32) -> Option<Type> {
        let mut symbol_tables: Vec<Rc<SymbolTable>> = vec![Rc::new(self.clone())];
        loop {
            let mut next = Vec::new();
            for symbol_table in &symbol_tables {
                if let Some(ty) = symbol_table.lookup_by_id(id) {
                    return Some(ty);
                }
                for child in symbol_table.children.borrow().iter() {
                    next.push(child.clone());
                }
            }
            if next.is_empty() {
                break;
            }
            symbol_tables = next;
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.borrow().is_empty()
    }
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut root = self;
        while let Some(ref parent) = root.parent {
            root = parent;
        }
        let mut res = String::new();
        let mut symbol_tables: Vec<Rc<SymbolTable>> = vec![Rc::new(root.clone())];
        res.push_str("Symbol Table\n");
        let mut level = 0;
        loop {
            let mut next = Vec::new();
            let pad = "  ".repeat(level);
            for symbol_table in &symbol_tables {
                writeln!(res, "{pad}+----------------------+-----------------+")?;
                writeln!(res, "{pad}| Name                 | Type            |")?;
                writeln!(res, "{pad}+----------------------+-----------------+")?;
                for (name, ty) in symbol_table.symbols.borrow().iter() {
                    let ty_str = if let Some(ty_val) = ty {
                        format!("{ty_val}")
                    } else {
                        "Unknown".to_string()
                    };
                    writeln!(res, "{pad}| {name:20} | {ty_str:15} |")?;
                }
                writeln!(res, "{pad}+----------------------+-----------------+")?;
                writeln!(res, "{pad}")?;
                writeln!(res, "{pad}+----------+-----------------+")?;
                writeln!(res, "{pad}| ID       | Type            |")?;
                writeln!(res, "{pad}+----------+-----------------+")?;
                for (id, ty) in symbol_table.id_type_map.borrow().iter() {
                    let ty_str = if let Some(ty_val) = ty {
                        format!("{ty_val}")
                    } else {
                        "Unknown".to_string()
                    };
                    writeln!(res, "{pad}| {id:8} | {ty_str:15} |")?;
                }
                writeln!(res, "{pad}+----------+-----------------+")?;
                let children: Vec<_> = symbol_table.children.borrow().iter().cloned().collect();
                for child in children {
                    next.push(child);
                }
            }
            if next.is_empty() {
                break;
            }
            symbol_tables = next;
            level += 1;
        }
        write!(f, "{res}")
    }
}

#[allow(clippy::map_entry)]
pub fn build_symbol_table(
    node_kind: Rc<NodeKind>,
    parent: Option<Rc<SymbolTable>>,
) -> anyhow::Result<Rc<SymbolTable>> {
    let symbol_table = Rc::new(SymbolTable::new(parent.clone()));
    let mut nodes: Vec<Rc<NodeKind>> = match node_kind.as_ref() {
        NodeKind::NewScope(node) => node.sorted_children(),
        NodeKind::SameScopeNode(_) => vec![node_kind],
    };
    while let Some(node) = nodes.pop() {
        match node.as_ref() {
            NodeKind::NewScope(node) => {
                let child_scope = build_symbol_table(
                    Rc::new(NodeKind::NewScope(node.clone())),
                    Some(symbol_table.clone()),
                )?;
                if !child_scope.is_empty() {
                    symbol_table.children.borrow_mut().push(child_scope);
                }
            }
            NodeKind::SameScopeNode(same) => match same {
                SameScopeNode::Composite(comp_node) => {
                    for child in comp_node.sorted_children() {
                        nodes.push(child);
                    }
                }
                SameScopeNode::Symbol(symbol_node) => {
                    let symbol_name = symbol_node.name();
                    let symbol_type = if let Some(type_expr) = symbol_node.type_expr() {
                        infer_expr(&type_expr, &symbol_table)
                    } else {
                        symbol_table.lookup(&symbol_name)
                    };

                    if symbol_table.symbols.borrow().contains_key(&symbol_name) {
                        if let Some(symbol_type) = symbol_type {
                            symbol_table.upsert(
                                symbol_node.id(),
                                symbol_name.clone(),
                                Some(symbol_type),
                            );
                        }
                    } else {
                        symbol_table.upsert(symbol_node.id(), symbol_name.clone(), symbol_type);
                    }

                    for child in symbol_node.sorted_children() {
                        nodes.push(child);
                    }
                }
            },
        }
    }
    if parent.as_ref().is_some() && !symbol_table.is_empty() {
        parent
            .unwrap()
            .children
            .borrow_mut()
            .push(symbol_table.clone());
    }
    Ok(symbol_table)
}

fn infer_expr(expr: &Expression, env: &Rc<SymbolTable>) -> Option<Type> {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Nat(n) => Some(Type::Nat(Rc::new(TypeNat::new(n)))),
            Literal::Bool(b) => Some(Type::Boolean(Rc::new(TypeBool::new(b)))),
            Literal::Str(s) => Some(Type::String(Rc::new(TypeString::new(s)))),
            Literal::Version(_) | Literal::Array(_) | Literal::Pad(_) => None,
        },
        Expression::Unary(un_expr) => infer_expr(&un_expr.operand, env),
        Expression::Binary(bin_expr) => {
            let left = infer_expr(&bin_expr.left, env)?;
            let right = infer_expr(&bin_expr.right, env)?;
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
                | BinaryExpressionOperator::Shr
                | BinaryExpressionOperator::Eq
                | BinaryExpressionOperator::Ne
                | BinaryExpressionOperator::Lt
                | BinaryExpressionOperator::Le
                | BinaryExpressionOperator::Gt
                | BinaryExpressionOperator::Ge
                | BinaryExpressionOperator::And
                | BinaryExpressionOperator::Or => {
                    if left.matches(&right) {
                        Some(left)
                    } else {
                        panic!("Error: type mismatch in the binary expression")
                    }
                }
            }
        }
        Expression::Conditional(conditional) => {
            let then_type = infer_expr(&conditional.then_branch, env)?;
            let else_type = infer_expr(&conditional.else_branch, env)?;
            if then_type.matches(&else_type) {
                Some(then_type)
            } else {
                panic!("Error: type mismatch in the conditional expression")
            }
        }
        Expression::Cast(cast) => Some(cast.target_type.clone()),
        Expression::IndexAccess(index_access) => infer_expr(&index_access.base, env),
        Expression::MemberAccess(member_access) => infer_expr(&member_access.base, env),
        Expression::FunctionCall(function_call) => infer_expr(&function_call.function, env),
        Expression::Identifier(identifier) => env.lookup(&identifier.name),
        Expression::TypeExpression(te) => Some(te.clone()),
        Expression::Sequence(expression_sequence) => {
            let mut tv = Vec::new();
            for expr in &expression_sequence.expressions {
                tv.push(infer_expr(expr, env)?);
            }
            Some(Type::Sum(Rc::new(Sum {
                id: 0,
                location: Location {
                    offset_start: tv.first().unwrap().location().offset_start,
                    offset_end: tv.last().unwrap().location().offset_end,
                    start_line: tv.first().unwrap().location().start_line,
                    start_column: tv.first().unwrap().location().start_column,
                    end_line: tv.last().unwrap().location().end_line,
                    end_column: tv.last().unwrap().location().end_column,
                    source: expression_sequence.location.source.clone(),
                },
                types: tv,
            })))
        }
        Expression::Disclose(disclose) => infer_expr(&disclose.expression, env),
        Expression::Map(_) | Expression::Fold(_) | Expression::Function(_) => None,
        Expression::Default(t) => infer_expr(&Expression::TypeExpression(t.clone()), env),
        Expression::Struct(struct_expr) => Some(struct_expr.ty.clone()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{
        definition::Definition,
        directive::VersionExpr,
        expression::{Binary, Conditional, Identifier, Sequence},
        literal::{Bool, Nat, Str, Version},
        node::Location,
        statement::{Block, If, Return, Statement, Var},
    };
    use anyhow::Result;

    // ============================
    // --- Helpers: create a default structs ---
    // ============================
    fn default_location() -> crate::ast::node::Location {
        crate::ast::node::Location {
            offset_start: 0,
            offset_end: 0,
            start_line: 0,
            start_column: 0,
            end_line: 0,
            end_column: 0,
            source: String::default(),
        }
    }

    fn mock_identifier(id: u32, name: &str) -> Rc<Identifier> {
        Rc::new(Identifier {
            id,
            location: default_location(),
            name: name.to_string(),
        })
    }

    #[test]
    fn test_build_symbol_table() -> anyhow::Result<()> {
        let block_stmt = Block {
            statements: vec![
                Statement::Var(Rc::new(Var {
                    id: 2,
                    location: default_location(),
                    ident: mock_identifier(3, "a"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 4,
                        location: default_location(),
                        value: 0,
                    }))),
                    ty_: None,
                })),
                Statement::Var(Rc::new(Var {
                    id: 5,
                    location: default_location(),
                    ident: mock_identifier(6, "b"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 7,
                        location: default_location(),
                        value: 0,
                    }))),
                    ty_: None,
                })),
                Statement::Return(Rc::new(Return {
                    id: 8,
                    location: default_location(),
                    value: Some(Expression::Sequence(Rc::new(Sequence {
                        id: 9,
                        location: default_location(),
                        expressions: vec![Expression::Binary(Rc::new(Binary {
                            id: 10,
                            location: default_location(),
                            left: Expression::Identifier(mock_identifier(11, "a")),
                            right: Expression::Identifier(mock_identifier(12, "b")),
                            operator: BinaryExpressionOperator::Add,
                        }))],
                    }))),
                })),
            ],
            id: 1,
            location: default_location(),
        };
        let symbol_table = build_symbol_table(
            Rc::new(NodeKind::from(&Statement::Block(Rc::new(block_stmt)))),
            None,
        )?;
        println!("{symbol_table}");
        assert!(symbol_table.parent.is_none());
        assert_eq!(symbol_table.symbols.borrow().len(), 2);
        assert_eq!(symbol_table.children.borrow().len(), 0);
        let _ = symbol_table.lookdown("a").unwrap();
        let _ = symbol_table.lookdown("b").unwrap();
        Ok(())
    }

    #[test]
    fn test_infer_expr() -> Result<()> {
        let block_stmt = Block {
            statements: vec![
                Statement::Var(Rc::new(Var {
                    id: 0,
                    location: default_location(),
                    ident: mock_identifier(6, "a"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 1,
                        location: default_location(),
                        value: 0,
                    }))),
                    ty_: Some(Expression::TypeExpression(Type::Nat(Rc::new(
                        TypeNat::default(),
                    )))),
                })),
                Statement::Var(Rc::new(Var {
                    id: 2,
                    location: default_location(),
                    ident: mock_identifier(7, "b"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 3,
                        location: default_location(),
                        value: 0,
                    }))),
                    ty_: Some(Expression::TypeExpression(Type::Nat(Rc::new(
                        TypeNat::default(),
                    )))),
                })),
                Statement::Return(Rc::new(Return {
                    id: 4,
                    location: default_location(),
                    value: Some(Expression::Sequence(Rc::new(Sequence {
                        id: 5,
                        location: default_location(),
                        expressions: vec![Expression::Binary(Rc::new(Binary {
                            id: 5,
                            location: default_location(),
                            left: Expression::Identifier(mock_identifier(6, "a")),
                            right: Expression::Identifier(mock_identifier(7, "b")),
                            operator: BinaryExpressionOperator::Add,
                        }))],
                    }))),
                })),
            ],
            id: 8,
            location: Location::default(),
        };
        let symbol_table = build_symbol_table(
            Rc::new(NodeKind::from(&Statement::Block(Rc::new(block_stmt)))),
            None,
        )?;
        let _ = symbol_table.lookup("a").unwrap();
        let _ = symbol_table.lookup("b").unwrap();
        let expr = Expression::Binary(Rc::new(Binary {
            id: 5,
            location: Location::default(),
            left: Expression::Identifier(mock_identifier(6, "a")),
            right: Expression::Identifier(mock_identifier(7, "b")),
            operator: BinaryExpressionOperator::Add,
        }));
        let ty = infer_expr(&expr, &symbol_table).unwrap();
        assert!(matches!(ty, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_literal_nat() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 1,
            location: default_location(),
            value: 0,
        })));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_literal_bool() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Bool(Rc::new(Bool {
            id: 2,
            location: default_location(),
            value: false,
        })));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::Boolean(_)));
        Ok(())
    }

    #[test]
    fn test_literal_str() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Str(Rc::new(Str {
            id: 3,
            location: default_location(),
            value: String::default(),
        })));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::String(_)));
        Ok(())
    }

    #[test]
    fn test_literal_version() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Version(Rc::new(Version {
            id: 4,
            location: default_location(),
            major: Rc::new(Nat {
                id: 5,
                location: default_location(),
                value: 0,
            }),
            minor: Some(Rc::new(Nat {
                id: 6,
                location: default_location(),
                value: 0,
            })),
            bugfix: None,
            operator: crate::ast::literal::VersionOperator::Gt,
        })));
        let ty = infer_expr(&expr, &env);
        // We treat Version as Unknown
        assert!(ty.is_none());
        Ok(())
    }

    #[test]
    fn test_conditional_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let cond = Expression::Literal(Literal::Bool(Rc::new(Bool {
            id: 5,
            location: default_location(),
            value: false,
        })));
        let then_branch = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 6,
            location: default_location(),
            value: 0,
        })));
        let else_branch = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 7,
            location: default_location(),
            value: 0,
        })));
        let cond_expr = Expression::Conditional(Rc::new(Conditional {
            id: 8,
            location: default_location(),
            condition: cond,
            then_branch,
            else_branch,
        }));
        let ty = infer_expr(&cond_expr, &env).unwrap();
        // Both branches are Nat -> Int.
        assert!(matches!(ty, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_binary_expr_add() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let left = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 9,
            location: default_location(),
            value: 0,
        })));
        let right = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 10,
            location: default_location(),
            value: 0,
        })));
        let binary = crate::ast::expression::Binary {
            id: 11,
            location: default_location(),
            left,
            right,
            operator: BinaryExpressionOperator::Add,
        };
        let expr = Expression::Binary(Rc::new(binary));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_cast_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        // For cast, our simple implementation calls infer_expr on target_type.
        // Here, target_type is a literal string so we expect Type::String.
        let cast = crate::ast::expression::Cast {
            id: 12,
            location: default_location(),
            expression: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 13,
                location: default_location(),
                value: 0,
            }))),
            target_type: Type::String(Rc::new(TypeString::default())),
        };
        let expr = Expression::Cast(Rc::new(cast));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::String(_)));
        Ok(())
    }

    #[test]
    fn test_member_access_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        // Insert a symbol "a" manually into env.
        env.insert(
            "a".to_string(),
            Some(Type::Nat(Rc::new(TypeNat::default()))),
        )
        .unwrap();
        // Create a member access expression whose base is the identifier "a".
        let member_access = crate::ast::expression::MemberAccess {
            id: 15,
            location: default_location(),
            base: Expression::Identifier(mock_identifier(16, "a")),
            member: mock_identifier(37, "name"),
            arguments: None,
        };
        let expr = Expression::MemberAccess(Rc::new(member_access));
        let ty = infer_expr(&expr, &env).unwrap();
        // infer_expr for MemberAccess just passes through the base’s type.
        assert!(matches!(ty, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_function_call_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        // Insert a symbol "f" with type String.
        env.insert(
            "f".to_string(),
            Some(Type::String(Rc::new(TypeString::default()))),
        )
        .unwrap();
        let func_call = crate::ast::expression::FunctionCall {
            id: 17,
            location: default_location(),
            function: Expression::Identifier(mock_identifier(18, "f")),
            arguments: vec![],
            reference: None,
        };
        let expr = Expression::FunctionCall(Rc::new(func_call));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::String(_)));
        Ok(())
    }

    #[test]
    fn test_identifier_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        env.insert(
            "a".to_string(),
            Some(Type::Nat(Rc::new(TypeNat::default()))),
        )
        .unwrap();
        let expr = Expression::Identifier(mock_identifier(19, "a"));
        let ty = infer_expr(&expr, &env).unwrap();
        assert!(matches!(ty, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_var_statement_and_block() -> Result<()> {
        // Create a block that declares two variables "a" and "b".
        let var_a = Statement::Var(Rc::new(Var {
            id: 20,
            location: default_location(),
            ident: mock_identifier(21, "a"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 22,
                location: default_location(),
                value: 0,
            }))),
            ty_: None,
        }));
        let var_b = Statement::Var(Rc::new(Var {
            id: 23,
            location: default_location(),
            ident: mock_identifier(24, "b"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 25,
                location: default_location(),
                value: 0,
            }))),
            ty_: None,
        }));
        let block = Statement::Block(Rc::new(Block {
            id: 26,
            location: default_location(),
            statements: vec![var_a.clone(), var_b.clone()],
        }));
        let symbol_table =
            build_symbol_table(Rc::new(crate::passes::NodeKind::from(&block)), None)?;
        println!("{symbol_table}");
        let sym_a = symbol_table.lookdown("a").unwrap();
        let sym_b = symbol_table.lookdown("b").unwrap();
        assert!(matches!(sym_a, Type::Nat(_)));
        assert!(matches!(sym_b, Type::Nat(_)));
        Ok(())
    }

    #[test]
    fn test_if_statement() -> Result<()> {
        // Build an If statement that uses identifiers "a" and "b".
        let if_stmt = Statement::If(Rc::new(If {
            id: 33,
            location: default_location(),
            condition: Expression::Literal(Literal::Bool(Rc::new(Bool {
                id: 34,
                location: default_location(),
                value: false,
            }))),
            then_branch: Statement::Block(Rc::new(Block {
                id: 27,
                location: default_location(),
                statements: vec![Statement::Var(Rc::new(Var {
                    id: 28,
                    location: default_location(),
                    ident: mock_identifier(28, "a"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 29,
                        location: default_location(),
                        value: 0,
                    }))),
                    ty_: None,
                }))],
            })),
            else_branch: Some(Statement::Block(Rc::new(Block {
                id: 30,
                location: default_location(),
                statements: vec![Statement::Var(Rc::new(Var {
                    id: 31,
                    location: default_location(),
                    ident: mock_identifier(31, "b"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 32,
                        location: default_location(),
                        value: 0,
                    }))),
                    ty_: None,
                }))],
            }))),
        }));
        let block = Statement::Block(Rc::new(Block {
            id: 35,
            location: default_location(),
            statements: vec![if_stmt],
        }));
        let symbol_table =
            build_symbol_table(Rc::new(crate::passes::NodeKind::from(&block)), None)?;
        println!("Symbol table: {symbol_table}\n");
        assert!(symbol_table.lookdown("a").is_some());
        assert!(symbol_table.lookdown("b").is_some());
        Ok(())
    }

    #[test]
    fn test_return_statement() -> Result<()> {
        // Create a Return statement that returns a binary expression "a + b"
        let var_a = Statement::Var(Rc::new(Var {
            id: 36,
            location: default_location(),
            ident: mock_identifier(37, "a"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 38,
                location: default_location(),
                value: 0,
            }))),
            ty_: None,
        }));
        let var_b = Statement::Var(Rc::new(Var {
            id: 39,
            location: default_location(),
            ident: mock_identifier(40, "b"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 41,
                location: default_location(),
                value: 0,
            }))),
            ty_: None,
        }));
        let ret_stmt = Statement::Return(Rc::new(Return {
            id: 42,
            location: default_location(),
            value: Some(Expression::Sequence(Rc::new(Sequence {
                id: 5,
                location: default_location(),
                expressions: vec![Expression::Binary(Rc::new(Binary {
                    id: 5,
                    location: default_location(),
                    left: Expression::Identifier(mock_identifier(6, "a")),
                    right: Expression::Identifier(mock_identifier(7, "b")),
                    operator: BinaryExpressionOperator::Add,
                }))],
            }))),
        }));
        let block = Statement::Block(Rc::new(Block {
            id: 44,
            location: default_location(),
            statements: vec![var_a, var_b, ret_stmt],
        }));
        let symbol_table =
            build_symbol_table(Rc::new(crate::passes::NodeKind::from(&block)), None)?;
        // Ensure "a" and "b" are present.
        assert!(symbol_table.lookup("a").is_some());
        assert!(symbol_table.lookup("b").is_some());
        // And infer_expr on the return expression yields Type::Int.
        if let Statement::Return(ret) = &block {
            if let Some(expr) = &ret.value {
                let ty = infer_expr(&expr.clone(), &symbol_table).unwrap();
                assert!(matches!(ty, Type::Nat(_)));
            }
        }
        Ok(())
    }

    fn test_definition_nodes() {
        let module = crate::ast::definition::Module {
            id: 55,
            location: default_location(),
            is_exported: false,
            name: mock_identifier(56, "module"),
            generic_parameters: None,
            nodes: vec![],
        };
        let circuit = crate::ast::definition::Circuit {
            id: 56,
            location: default_location(),
            is_exported: false,
            is_pure: false,
            name: mock_identifier(57, "circuit"),
            arguments: vec![],
            generic_parameters: None,
            ty: Type::Nat(Rc::new(TypeNat::default())),
            body: Some(Rc::new(crate::ast::statement::Block {
                id: 58,
                location: default_location(),
                statements: vec![],
            })),
        };
        let defs = vec![
            Definition::Module(Rc::new(module)),
            Definition::Circuit(Rc::new(circuit)),
        ];
        for def in defs {
            assert!(!format!("{def:?}").is_empty());
        }
    }

    #[test]
    fn test_directive_nodes() {
        let pragma = crate::ast::directive::Pragma {
            id: 57,
            location: default_location(),
            value: Rc::new(Identifier {
                id: 58,
                location: default_location(),
                name: "pragma".to_string(),
            }),
            version: VersionExpr::Version(Rc::new(Version {
                id: 59,
                location: default_location(),
                major: Rc::new(Nat {
                    id: 60,
                    location: default_location(),
                    value: 0,
                }),
                minor: Some(Rc::new(Nat {
                    id: 61,
                    location: default_location(),
                    value: 0,
                })),
                bugfix: None,
                operator: crate::ast::literal::VersionOperator::Gt,
            })),
        };

        let dirs = vec![crate::ast::directive::Directive::Pragma(Rc::new(pragma))];
        for d in dirs {
            assert!(!format!("{d:?}").is_empty());
        }
    }
}
