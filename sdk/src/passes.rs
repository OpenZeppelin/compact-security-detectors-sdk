#![allow(dead_code)]
use anyhow::{anyhow, Ok, Result};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::{
    expression::{BinaryExpressionOperator, Expression},
    literal::Literal,
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

impl From<Rc<dyn Node>> for NodeKind {
    fn from(node: Rc<dyn Node>) -> Self {
        NodeKind::NewScope(node)
    }
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

pub struct SymbolTable {
    pub symbols: RefCell<HashMap<String, Type>>,
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
    pub fn insert(&self, name: String, ty: Type) -> Result<()> {
        let mut syms = self.symbols.borrow_mut();
        if syms.contains_key(&name) {
            Err(anyhow!("Symbol {name} already exists"))
        } else {
            syms.insert(name, ty);
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<Type> {
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
    let mut nodes: Vec<Rc<NodeKind>> = vec![node_kind];
    while let Some(node) = nodes.pop() {
        match node.as_ref() {
            NodeKind::NewScope(node) => {
                let child_scope = build_symbol_table(
                    Rc::new(NodeKind::NewScope(node.clone())),
                    Some(symbol_table.clone()),
                )?;
                symbol_table.children.borrow_mut().push(child_scope);
            }
            NodeKind::SameScopeNode(same) => match same {
                SameScopeNode::Composite(comp_node) => {
                    for child in comp_node.children() {
                        nodes.push(child);
                    }
                }
                SameScopeNode::Symbol(symbol_node) => {
                    let symbol_name = symbol_node.name();
                    let symbol_type = if let Some(type_expr) = symbol_node.type_expr() {
                        infer_expr(type_expr, &symbol_table)?
                    } else {
                        Type::Unknown
                    };
                    if symbol_table.symbols.borrow().contains_key(&symbol_name) {
                        if let Some(st) = symbol_table.lookup(&symbol_name) {
                            if st == Type::Unknown {
                                //why didn't we see the type of the symbol before? bug?
                                return Err(anyhow!(
                                    "Symbol {symbol_name} without a type in a symbol table"
                                ));
                            } else if symbol_type == Type::Unknown {
                                symbol_table
                                    .symbols
                                    .borrow_mut()
                                    .insert(symbol_name.clone(), st);
                            } else {
                                //shadowing is not allowed
                                return Err(anyhow!("Symbol {symbol_name} already exists"));
                            }
                        }
                    } else {
                        symbol_table
                            .symbols
                            .borrow_mut()
                            .insert(symbol_name.clone(), symbol_type);
                    }
                    for child in symbol_node.children() {
                        nodes.push(child);
                    }
                }
            },
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
            let symbol_type = env
                .lookup(&identifier.name)
                .ok_or_else(|| anyhow!("Undefined identifier"))?;
            Ok(symbol_type)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{
        declaration::Declaration,
        definition::Definition,
        expression::{Binary, Conditional, Identifier},
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
            source_code: String::new(),
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
        }
    }

    fn mock_identifier(id: u128, name: &str) -> Rc<Identifier> {
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
                    id: 0,
                    location: default_location(),
                    name: mock_identifier(1, "a"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 1,
                        location: default_location(),
                    }))),
                    ty_: None,
                })),
                Statement::Var(Rc::new(Var {
                    id: 2,
                    location: default_location(),
                    name: mock_identifier(2, "b"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 3,
                        location: default_location(),
                    }))),
                    ty_: None,
                })),
                Statement::Return(Rc::new(Return {
                    id: 4,
                    location: default_location(),
                    value: Some(Expression::Binary(Rc::new(Binary {
                        id: 5,
                        location: default_location(),
                        left_operand: Expression::Identifier(mock_identifier(1, "a")),
                        right_operand: Expression::Identifier(mock_identifier(2, "b")),
                        operator: BinaryExpressionOperator::Add,
                    }))),
                })),
            ],
            id: 8,
            location: default_location(),
        };
        let symbol_table = build_symbol_table(
            Rc::new(NodeKind::from(&Statement::Block(Rc::new(block_stmt)))),
            None,
        )?;
        assert!(symbol_table.parent.is_none());
        assert_eq!(symbol_table.symbols.borrow().len(), 2);
        assert_eq!(symbol_table.children.borrow().len(), 0);
        let _ = symbol_table.lookup("a").unwrap();
        let _ = symbol_table.lookup("b").unwrap();
        Ok(())
    }

    #[test]
    fn test_infer_expr() -> Result<()> {
        let block_stmt = Block {
            statements: vec![
                Statement::Var(Rc::new(Var {
                    id: 0,
                    location: default_location(),
                    name: mock_identifier(6, "a"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 1,
                        location: default_location(),
                    }))),
                    ty_: None,
                })),
                Statement::Var(Rc::new(Var {
                    id: 2,
                    location: default_location(),
                    name: mock_identifier(7, "b"),
                    value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                        id: 3,
                        location: default_location(),
                    }))),
                    ty_: None,
                })),
                Statement::Return(Rc::new(Return {
                    id: 4,
                    location: default_location(),
                    value: Some(Expression::Binary(Rc::new(Binary {
                        id: 5,
                        location: default_location(),
                        left_operand: Expression::Identifier(mock_identifier(6, "a")),
                        right_operand: Expression::Identifier(mock_identifier(7, "b")),
                        operator: BinaryExpressionOperator::Add,
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
            left_operand: Expression::Identifier(mock_identifier(6, "a")),
            right_operand: Expression::Identifier(mock_identifier(7, "b")),
            operator: BinaryExpressionOperator::Add,
        }));
        let ty = infer_expr(&expr, &symbol_table)?;
        assert_eq!(ty, Type::Int);
        Ok(())
    }

    #[test]
    fn test_literal_nat() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 1,
            location: default_location(),
        })));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::Int);
        Ok(())
    }

    #[test]
    fn test_literal_bool() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Bool(Rc::new(Bool {
            id: 2,
            location: default_location(),
        })));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::Bool);
        Ok(())
    }

    #[test]
    fn test_literal_str() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Str(Rc::new(Str {
            id: 3,
            location: default_location(),
        })));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::String);
        Ok(())
    }

    #[test]
    fn test_literal_version() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let expr = Expression::Literal(Literal::Version(Rc::new(Version {
            id: 4,
            location: default_location(),
        })));
        let ty = infer_expr(&expr, &env)?;
        // We treat Version as Unknown
        assert_eq!(ty, Type::Unknown);
        Ok(())
    }

    #[test]
    fn test_conditional_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let cond = Expression::Literal(Literal::Bool(Rc::new(Bool {
            id: 5,
            location: default_location(),
        })));
        let then_branch = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 6,
            location: default_location(),
        })));
        let else_branch = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 7,
            location: default_location(),
        })));
        let cond_expr = Expression::Conditional(Rc::new(Conditional {
            id: 8,
            location: default_location(),
            condition: Rc::new(cond),
            then_branch: Rc::new(then_branch),
            else_branch: Rc::new(else_branch),
        }));
        let ty = infer_expr(&cond_expr, &env)?;
        // Both branches are Nat -> Int.
        assert_eq!(ty, Type::Int);
        Ok(())
    }

    #[test]
    fn test_binary_expr_add() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        let left = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 9,
            location: default_location(),
        })));
        let right = Expression::Literal(Literal::Nat(Rc::new(Nat {
            id: 10,
            location: default_location(),
        })));
        let binary = crate::ast::expression::Binary {
            id: 11,
            location: default_location(),
            left_operand: left,
            right_operand: right,
            operator: BinaryExpressionOperator::Add,
        };
        let expr = Expression::Binary(Rc::new(binary));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::Int);
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
            expression: Rc::new(Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 13,
                location: default_location(),
            })))),
            target_type: Rc::new(Expression::Literal(Literal::Str(Rc::new(Str {
                id: 14,
                location: default_location(),
            })))),
        };
        let expr = Expression::Cast(Rc::new(cast));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::String);
        Ok(())
    }

    #[test]
    fn test_member_access_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        // Insert a symbol "a" manually into env.
        env.insert("a".to_string(), Type::Int)?;
        // Create a member access expression whose base is the identifier "a".
        let member_access = crate::ast::expression::MemberAccess {
            id: 15,
            location: default_location(),
            base: Rc::new(Expression::Identifier(mock_identifier(16, "a"))),
            member: "dummy".to_string(),
        };
        let expr = Expression::MemberAccess(Rc::new(member_access));
        let ty = infer_expr(&expr, &env)?;
        // infer_expr for MemberAccess just passes through the base’s type.
        assert_eq!(ty, Type::Int);
        Ok(())
    }

    #[test]
    fn test_function_call_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        // Insert a symbol "f" with type String.
        env.insert("f".to_string(), Type::String)?;
        let func_call = crate::ast::expression::FunctionCall {
            id: 17,
            location: default_location(),
            function: Rc::new(Expression::Identifier(mock_identifier(18, "f"))),
            arguments: vec![],
        };
        let expr = Expression::FunctionCall(Rc::new(func_call));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::String);
        Ok(())
    }

    #[test]
    fn test_identifier_expr() -> Result<()> {
        let env = Rc::new(SymbolTable::new(None));
        env.insert("a".to_string(), Type::Int)?;
        let expr = Expression::Identifier(mock_identifier(19, "a"));
        let ty = infer_expr(&expr, &env)?;
        assert_eq!(ty, Type::Int);
        Ok(())
    }

    #[test]
    fn test_var_statement_and_block() -> Result<()> {
        // Create a block that declares two variables "a" and "b".
        let var_a = Statement::Var(Rc::new(Var {
            id: 20,
            location: default_location(),
            name: mock_identifier(21, "a"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 22,
                location: default_location(),
            }))),
            ty_: None,
        }));
        let var_b = Statement::Var(Rc::new(Var {
            id: 23,
            location: default_location(),
            name: mock_identifier(24, "b"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 25,
                location: default_location(),
            }))),
            ty_: None,
        }));
        let block = Statement::Block(Rc::new(Block {
            id: 26,
            location: default_location(),
            statements: vec![var_a.clone(), var_b.clone()],
        }));
        // Build the symbol table for the block.
        let symbol_table =
            build_symbol_table(Rc::new(crate::passes::NodeKind::from(&block)), None)?;
        // Both symbols should be present with type Int.
        let sym_a = symbol_table.lookup("a").unwrap();
        let sym_b = symbol_table.lookup("b").unwrap();
        assert_eq!(sym_a, Type::Int);
        assert_eq!(sym_b, Type::Int);
        Ok(())
    }

    #[test]
    fn test_if_statement() -> Result<()> {
        // Build an If statement that uses identifiers "a" and "b".
        let var_a = Statement::Var(Rc::new(Var {
            id: 27,
            location: default_location(),
            name: mock_identifier(28, "a"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 29,
                location: default_location(),
            }))),
            ty_: None,
        }));
        let var_b = Statement::Var(Rc::new(Var {
            id: 30,
            location: default_location(),
            name: mock_identifier(31, "b"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 32,
                location: default_location(),
            }))),
            ty_: None,
        }));
        let if_stmt = Statement::If(Rc::new(If {
            id: 33,
            location: default_location(),
            condition: Expression::Literal(Literal::Bool(Rc::new(Bool {
                id: 34,
                location: default_location(),
            }))),
            then_branch: var_a.clone(),
            else_branch: Some(var_b.clone()),
        }));
        // Wrap the if statement in a block.
        let block = Statement::Block(Rc::new(Block {
            id: 35,
            location: default_location(),
            statements: vec![if_stmt],
        }));
        let symbol_table =
            build_symbol_table(Rc::new(crate::passes::NodeKind::from(&block)), None)?;
        // The block should include the symbols from any Var declarations inside.
        assert!(symbol_table.lookup("a").is_some());
        assert!(symbol_table.lookup("b").is_some());
        Ok(())
    }

    #[test]
    fn test_return_statement() -> Result<()> {
        // Create a Return statement that returns a binary expression "a + b"
        let var_a = Statement::Var(Rc::new(Var {
            id: 36,
            location: default_location(),
            name: mock_identifier(37, "a"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 38,
                location: default_location(),
            }))),
            ty_: None,
        }));
        let var_b = Statement::Var(Rc::new(Var {
            id: 39,
            location: default_location(),
            name: mock_identifier(40, "b"),
            value: Expression::Literal(Literal::Nat(Rc::new(Nat {
                id: 41,
                location: default_location(),
            }))),
            ty_: None,
        }));
        let ret_stmt = Statement::Return(Rc::new(Return {
            id: 42,
            location: default_location(),
            value: Some(Expression::Binary(Rc::new(
                crate::ast::expression::Binary {
                    id: 43,
                    location: default_location(),
                    left_operand: Expression::Identifier(mock_identifier(37, "a")),
                    right_operand: Expression::Identifier(mock_identifier(40, "b")),
                    operator: BinaryExpressionOperator::Add,
                },
            ))),
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
                let ty = infer_expr(expr, &symbol_table)?;
                assert_eq!(ty, Type::Int);
            }
        }
        Ok(())
    }

    #[test]
    fn test_declaration_nodes() {
        let import = crate::ast::declaration::Import {
            id: 45,
            location: default_location(),
        };
        let export = crate::ast::declaration::Export {
            id: 46,
            location: default_location(),
        };
        let external = crate::ast::declaration::External {
            id: 47,
            location: default_location(),
        };
        let witness = crate::ast::declaration::Witness {
            id: 48,
            location: default_location(),
        };
        let ledger = crate::ast::declaration::Ledger {
            id: 49,
            location: default_location(),
        };
        let ctor = crate::ast::declaration::Ctor {
            id: 50,
            location: default_location(),
        };
        let contract = crate::ast::declaration::Contract {
            id: 51,
            location: default_location(),
        };
        let struc = crate::ast::declaration::Struct {
            id: 52,
            location: default_location(),
        };
        let enm = crate::ast::declaration::Enum {
            id: 53,
            location: default_location(),
        };

        let decls = vec![
            Declaration::Import(Rc::new(import)),
            Declaration::Export(Rc::new(export)),
            Declaration::External(Rc::new(external)),
            Declaration::Witness(Rc::new(witness)),
            Declaration::Ledger(Rc::new(ledger)),
            Declaration::Ctor(Rc::new(ctor)),
            Declaration::Contract(Rc::new(contract)),
            Declaration::Struct(Rc::new(struc)),
            Declaration::Enum(Rc::new(enm)),
            Declaration::Definition(Definition::Module(Rc::new(
                crate::ast::definition::Module {
                    id: 54,
                    location: default_location(),
                },
            ))),
        ];
        for decl in decls {
            assert!(!format!("{decl:?}").is_empty());
        }
    }

    #[test]
    fn test_definition_nodes() {
        let module = crate::ast::definition::Module {
            id: 55,
            location: default_location(),
        };
        let circuit = crate::ast::definition::Circuit {
            id: 56,
            location: default_location(),
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
        };
        let include = crate::ast::directive::Include {
            id: 58,
            location: default_location(),
        };
        let dirs = vec![
            crate::ast::directive::Directive::Pragma(Rc::new(pragma)),
            crate::ast::directive::Directive::Include(Rc::new(include)),
        ];
        for d in dirs {
            assert!(!format!("{d:?}").is_empty());
        }
    }
}
