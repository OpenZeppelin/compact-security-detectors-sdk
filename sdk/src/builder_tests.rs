#[cfg(test)]
mod circuit_parsing_tests {
    use core::panic;
    use std::{cell::RefCell, collections::HashMap};

    use crate::{
        ast::{
            declaration::Pattern,
            expression::{BinaryExpressionOperator, Expression},
            statement::Statement,
            ty::Type,
        },
        build_codebase,
        codebase::{Codebase, SealedState, SourceCodeFile},
        parse_content,
    };

    fn build_codebase_wrapper(src: &str) -> RefCell<Codebase<SealedState>> {
        let mut files = HashMap::new();
        files.insert("dummy".to_string(), src.to_string());
        build_codebase(files).unwrap()
    }

    fn build_source_code_file(src: &str) -> SourceCodeFile {
        parse_content("dummy", src).unwrap()
    }

    fn check_type_uint_fixed_size(ty: &Type, size: u64) {
        match ty {
            Type::Uint(n) => {
                assert_eq!(n.start.value, size);
                assert!(n.end.is_none());
            }
            _ => panic!("Expected Uint type"),
        }
    }

    #[test]
    fn simple() {
        let codebase = build_codebase_wrapper(
            "circuit add (x: Uint<8>, y: Uint<8>) : Uint<8> { return x + y; }",
        );
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.definitions.len(), 1);
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let circuit = circuits.first().unwrap();
        assert!(!circuit.is_exported);
        assert!(!circuit.is_pure);
        assert_eq!(circuit.name(), "add");
        assert_eq!(circuit.arguments.len(), 2);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "x");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Uint(uint_t)) => {
                        assert_eq!(uint_t.start.value, 8);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type for identifier with id {}", ident.id),
                }
            }
            _ => panic!("Expected identifier"),
        }
        check_type_uint_fixed_size(&arg.ty, 8);
        let arg = circuit.arguments.last().unwrap();
        assert_eq!(arg.name().unwrap(), "y");
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "y");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Uint(uint_t)) => {
                        assert_eq!(uint_t.start.value, 8);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type"),
                }
            }
            _ => panic!("Expected identifier"),
        }
        check_type_uint_fixed_size(&arg.ty, 8);
        assert!(matches!(circuit.ty, Type::Uint(_)));
        check_type_uint_fixed_size(&circuit.ty, 8);
        assert!(circuit.body.is_some());
        assert_eq!(circuit.body.as_ref().unwrap().statements.len(), 1);
        match circuit.body.as_ref().unwrap().statements.first().unwrap() {
            Statement::Return(return_stmt) => {
                assert!(return_stmt.value.is_some());
                match return_stmt.value.as_ref().unwrap() {
                    Expression::Sequence(seq) => {
                        assert_eq!(seq.expressions.len(), 1);
                        match seq.expressions.first().unwrap() {
                            Expression::Binary(op) => {
                                assert_eq!(op.operator, BinaryExpressionOperator::Add);
                                match &op.left {
                                    Expression::Identifier(ident) => {
                                        assert_eq!(ident.name, "x");
                                        match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                            Some(Type::Uint(uint_t)) => {
                                                assert_eq!(uint_t.start.value, 8);
                                                assert!(uint_t.end.is_none());
                                            }
                                            _ => panic!("Expected Uint type of identifier"),
                                        }
                                    }
                                    _ => panic!("Expected identifier"),
                                }
                                match &op.right {
                                    Expression::Identifier(ident) => {
                                        assert_eq!(ident.name, "y");
                                        match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                            Some(Type::Uint(uint_t)) => {
                                                assert_eq!(uint_t.start.value, 8);
                                                assert!(uint_t.end.is_none());
                                            }
                                            _ => panic!("Expected Uint type of identifier"),
                                        }
                                    }
                                    _ => panic!("Expected identifier"),
                                }
                            }
                            _ => panic!("Expected binary operation"),
                        }
                    }
                    _ => panic!("Expected sequence expression"),
                }
            }
            _ => panic!("Expected return statement"),
        }
    }

    #[test]
    fn export_simple_circuit() {
        let codebase = build_codebase_wrapper(
            "export circuit add (x: Uint<8>, y: Uint<8>) : Uint<8> { return x * y; }",
        );
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.definitions.len(), 1);
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let circuit = circuits.first().unwrap();
        assert!(circuit.is_exported);
        assert!(!circuit.is_pure);
        assert_eq!(circuit.name(), "add");
        assert_eq!(circuit.arguments.len(), 2);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "x");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Uint(uint_t)) => {
                        assert_eq!(uint_t.start.value, 8);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type for identifier with id {}", ident.id),
                }
            }
            _ => panic!("Expected identifier"),
        }
        check_type_uint_fixed_size(&arg.ty, 8);
        let arg = circuit.arguments.last().unwrap();
        assert_eq!(arg.name().unwrap(), "y");
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "y");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Uint(uint_t)) => {
                        assert_eq!(uint_t.start.value, 8);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type"),
                }
            }
            _ => panic!("Expected identifier"),
        }
        check_type_uint_fixed_size(&arg.ty, 8);
        assert!(matches!(circuit.ty, Type::Uint(_)));
        check_type_uint_fixed_size(&circuit.ty, 8);
        assert!(circuit.body.is_some());
        assert_eq!(circuit.body.as_ref().unwrap().statements.len(), 1);
        match circuit.body.as_ref().unwrap().statements.first().unwrap() {
            Statement::Return(return_stmt) => {
                assert!(return_stmt.value.is_some());
                match return_stmt.value.as_ref().unwrap() {
                    Expression::Sequence(seq) => {
                        assert_eq!(seq.expressions.len(), 1);
                        match seq.expressions.first().unwrap() {
                            Expression::Binary(op) => {
                                assert_eq!(op.operator, BinaryExpressionOperator::Mul);
                                match &op.left {
                                    Expression::Identifier(ident) => {
                                        assert_eq!(ident.name, "x");
                                        match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                            Some(Type::Uint(uint_t)) => {
                                                assert_eq!(uint_t.start.value, 8);
                                                assert!(uint_t.end.is_none());
                                            }
                                            _ => panic!("Expected Uint type of identifier"),
                                        }
                                    }
                                    _ => panic!("Expected identifier"),
                                }
                                match &op.right {
                                    Expression::Identifier(ident) => {
                                        assert_eq!(ident.name, "y");
                                        match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                            Some(Type::Uint(uint_t)) => {
                                                assert_eq!(uint_t.start.value, 8);
                                                assert!(uint_t.end.is_none());
                                            }
                                            _ => panic!("Expected Uint type of identifier"),
                                        }
                                    }
                                    _ => panic!("Expected identifier"),
                                }
                            }
                            _ => panic!("Expected binary operation"),
                        }
                    }
                    _ => panic!("Expected sequence expression"),
                }
            }
            _ => panic!("Expected return statement"),
        }
    }
}
