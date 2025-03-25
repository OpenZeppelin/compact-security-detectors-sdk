use std::{cell::RefCell, collections::HashMap};

use crate::{
    ast::ty::Type,
    build_codebase,
    codebase::{Codebase, SealedState},
};

#[allow(dead_code)]
fn build_codebase_wrapper(src: &str) -> RefCell<Codebase<SealedState>> {
    let mut files = HashMap::new();
    files.insert("dummy".to_string(), src.to_string());
    build_codebase(files).unwrap()
}

#[allow(dead_code)]
fn check_type_uint_fixed_size(ty: &Type, size: u64) {
    match ty {
        Type::Uint(n) => {
            assert_eq!(n.start.value, size);
            assert!(n.end.is_none());
        }
        _ => panic!("Expected Uint type"),
    }
}

#[cfg(test)]
mod circuit_parsing_tests {
    use core::panic;

    use crate::{
        ast::{
            declaration::Pattern,
            expression::{BinaryExpressionOperator, Expression},
            statement::Statement,
            ty::Type,
        },
        builder_tests::{build_codebase_wrapper, check_type_uint_fixed_size},
    };

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
        assert!(circuit.generic_parameters.is_none());
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
            "export pure circuit add (x: Uint<8>, y: Uint<8>) : Uint<8> { return x * y; }",
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
        assert!(circuit.is_pure);
        assert_eq!(circuit.name(), "add");
        assert!(circuit.generic_parameters.is_none());
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

    #[test]
    fn circuit_with_generic_parameters() {
        let codebase =
            build_codebase_wrapper("circuit process<T> (data: Field) : Field {return data;}");
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
        assert_eq!(circuit.name(), "process");
        assert_eq!(circuit.arguments.len(), 1);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "data");
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "data");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Field(_)) => {}
                    _ => panic!("Expected Field type for identifier with id {}", ident.id),
                }
            }
            _ => panic!("Expected identifier"),
        }
        assert!(matches!(&arg.ty, Type::Field(_)));
        assert!(matches!(&circuit.ty, Type::Field(_)));
        assert!(circuit.generic_parameters.is_some());
        assert_eq!(circuit.generic_parameters.as_ref().unwrap().len(), 1);
        assert_eq!(
            circuit
                .generic_parameters
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .name,
            "T"
        );
        assert!(circuit.body.is_some());
        assert_eq!(circuit.body.as_ref().unwrap().statements.len(), 1);
        match circuit.body.as_ref().unwrap().statements.first().unwrap() {
            Statement::Return(return_stmt) => {
                assert!(return_stmt.value.is_some());
                match return_stmt.value.as_ref().unwrap() {
                    Expression::Sequence(seq) => {
                        assert_eq!(seq.expressions.len(), 1);
                        match seq.expressions.first().unwrap() {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "data");
                                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                    Some(Type::Field(_)) => {}
                                    _ => panic!("Expected Field type of identifier"),
                                }
                            }
                            _ => panic!("Expected identifier"),
                        }
                    }
                    _ => panic!("Expected sequence expression"),
                }
            }
            _ => panic!("Expected return statement"),
        }
    }
}

#[cfg(test)]
mod constructor_parsing_tests {
    use crate::{
        ast::{
            declaration::Pattern,
            expression::Expression,
            literal::Literal,
            statement::{AssignOperator, Statement},
            ty::Type,
        },
        builder_tests::build_codebase_wrapper,
    };

    #[test]
    fn empty_constructor() {
        let codebase = build_codebase_wrapper("constructor() { }");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        assert_eq!(ast.constructors().len(), 1);
        let binding = ast.constructors();
        let constructor = binding.first().unwrap();
        assert_eq!(constructor.arguments.len(), 0);
        assert_eq!(constructor.body.statements.len(), 0);
    }

    #[test]
    fn constructor_wth_one_parameter() {
        let codebase = build_codebase_wrapper("constructor(x: Field) { }");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        assert_eq!(ast.constructors().len(), 1);
        let binding = ast.constructors();
        let constructor = binding.first().unwrap();
        assert_eq!(constructor.arguments.len(), 1);
        let arg = constructor.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        assert!(matches!(arg.ty, Type::Field(_)));
        assert_eq!(constructor.body.statements.len(), 0);
    }

    #[test]
    fn constructor_with_two_parameters_and_a_block_with_a_simple_statement() {
        let codebase = build_codebase_wrapper("constructor(x: Field, y: Uint<32>) { x = 0; }");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        assert_eq!(ast.constructors().len(), 1);
        let binding = ast.constructors();
        let constructor = binding.first().unwrap();
        assert_eq!(constructor.arguments.len(), 2);
        let arg = constructor.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        assert!(matches!(arg.ty, Type::Field(_)));
        let arg = constructor.arguments.last().unwrap();
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "y");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Uint(uint_t)) => {
                        assert_eq!(uint_t.start.value, 32);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type for identifier with id {}", ident.id),
                }
            }
            _ => panic!("Expected identifier"),
        }
        assert!(matches!(arg.ty, Type::Uint(_)));
        assert_eq!(constructor.body.statements.len(), 1);
        let stmt = constructor.body.statements.first().unwrap();
        match stmt {
            Statement::Assign(assign_stmt) => {
                assert!(assign_stmt.operator == AssignOperator::Simple);
                match &assign_stmt.target {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.name, "x");
                        match codebase.get_symbol_type_by_id("dummy", ident.id) {
                            Some(Type::Field(_)) => {}
                            _ => panic!("Expected Field type for identifier with id {}", ident.id),
                        }
                    }
                    _ => panic!("Expected identifier"),
                }
                match &assign_stmt.value {
                    Expression::Literal(Literal::Nat(literal)) => {
                        assert_eq!(literal.value, 0);
                    }
                    _ => panic!("Expected literal expression"),
                }
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn constructor_with_two_parameters_and_a_block_with_if_statement() {
        let codebase = build_codebase_wrapper(
            "constructor (x: Field, y: Uint<32>) { if (x == 0) { return; } else { y += 1; } }",
        );
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        assert_eq!(ast.constructors().len(), 1);
        let binding = ast.constructors();
        let constructor = binding.first().unwrap();
        assert_eq!(constructor.arguments.len(), 2);
        let arg = constructor.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        assert!(matches!(arg.ty, Type::Field(_)));
        let arg = constructor.arguments.last().unwrap();
        match &arg.pattern {
            Pattern::Identifier(ident) => {
                assert_eq!(ident.name, "y");
                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                    Some(Type::Uint(uint_t)) => {
                        assert_eq!(uint_t.start.value, 32);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type for identifier with id {}", ident.id),
                }
            }
            _ => panic!("Expected identifier"),
        }
        assert!(matches!(arg.ty, Type::Uint(_)));
        assert_eq!(constructor.body.statements.len(), 1);
        let stmt = constructor.body.statements.first().unwrap();
        match stmt {
            Statement::If(if_stmt) => {
                match &if_stmt.condition {
                    Expression::Sequence(seq_expr) => {
                        assert_eq!(seq_expr.expressions.len(), 1);
                        let expr = seq_expr.expressions.first().unwrap();
                        assert!(matches!(expr, Expression::Binary(_)));
                        let Expression::Binary(op) = expr else {
                            panic!("Expected binary expression");
                        };
                        assert_eq!(
                            op.operator,
                            crate::ast::expression::BinaryExpressionOperator::Eq
                        );
                        match &op.left {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                                match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                    Some(Type::Field(_)) => {}
                                    _ => panic!(
                                        "Expected Field type for identifier with id {}",
                                        ident.id
                                    ),
                                }
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &op.right {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 0);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression, found {:?}", if_stmt.condition),
                }
                if let Statement::Block(block) = &if_stmt.then_branch {
                    let stmt = block.statements.first().unwrap();
                    match stmt {
                        Statement::Return(_) => {}
                        _ => panic!("Expected return statement"),
                    }
                }
                assert!(if_stmt.else_branch.is_some());
                if let Some(Statement::Block(block)) = &if_stmt.else_branch {
                    let stmt = block.statements.first().unwrap();
                    match stmt {
                        Statement::Assign(assign_stmt) => {
                            assert!(assign_stmt.operator == AssignOperator::Add);
                            match &assign_stmt.target {
                                Expression::Identifier(ident) => {
                                    assert_eq!(ident.name, "y");
                                    match codebase.get_symbol_type_by_id("dummy", ident.id) {
                                        Some(Type::Uint(uint_t)) => {
                                            assert_eq!(uint_t.start.value, 32);
                                            assert!(uint_t.end.is_none());
                                        }
                                        _ => panic!(
                                            "Expected Uint type for identifier with id {}",
                                            ident.id
                                        ),
                                    }
                                }
                                _ => panic!("Expected identifier"),
                            }
                            match &assign_stmt.value {
                                Expression::Literal(Literal::Nat(literal)) => {
                                    assert_eq!(literal.value, 1);
                                }
                                _ => panic!("Expected literal expression"),
                            }
                        }
                        _ => panic!("Expected assignment statement"),
                    }
                }
            }
            _ => panic!("Expected if statement"),
        }
    }
}

#[cfg(test)]
mod enum_parsing_tests {
    use crate::{ast::definition::Definition, builder_tests::build_codebase_wrapper};

    #[test]
    fn simple_enum() {
        let codebase = build_codebase_wrapper("enum Color { red }");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.definitions.len(), 1);
        match &ast.definitions[0] {
            Definition::Enum(enum_def) => {
                assert_eq!(enum_def.name(), "Color");
                assert!(!enum_def.is_exported);
                assert_eq!(enum_def.options.len(), 1);
                assert_eq!(enum_def.options[0].name, "red");
            }
            _ => panic!("Expected enum declaration"),
        }
    }

    #[test]
    fn multiple_enum_options() {
        let codebase = build_codebase_wrapper("export enum Days { Monday, Tuesday, Wednesday }");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.definitions.len(), 1);
        match &ast.definitions[0] {
            Definition::Enum(enum_def) => {
                assert_eq!(enum_def.name(), "Days");
                assert!(enum_def.is_exported);
                assert_eq!(enum_def.options.len(), 3);
                assert_eq!(enum_def.options[0].name, "Monday");
                assert_eq!(enum_def.options[1].name, "Tuesday");
                assert_eq!(enum_def.options[2].name, "Wednesday");
            }
            _ => panic!("Expected enum declaration"),
        }
    }
}

#[cfg(test)]
mod export_parsing_tests {
    use crate::{ast::declaration::Declaration, builder_tests::build_codebase_wrapper};

    #[test]
    fn simple_export() {
        let codebase = build_codebase_wrapper("export { foo }");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Export(export) => {
                assert_eq!(export.values.len(), 1);
                assert_eq!(export.values[0].name, "foo");
            }
            _ => panic!("Expected name export"),
        }
    }

    #[test]
    fn multiple_exports() {
        let codebase = build_codebase_wrapper("export { foo, bar, baz, };");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Export(export) => {
                assert_eq!(export.values.len(), 3);
                assert_eq!(export.values[0].name, "foo");
                assert_eq!(export.values[1].name, "bar");
                assert_eq!(export.values[2].name, "baz");
            }
            _ => panic!("Expected name export"),
        }
    }
}

#[cfg(test)]
mod external_contract_parsing_tests {
    use crate::{
        ast::{declaration::Declaration, ty::Type},
        builder_tests::build_codebase_wrapper,
    };

    #[test]
    fn simple_external_contract() {
        let codebase = build_codebase_wrapper(
            "contract MyContract {
                circuit foo (x: Field) : Field;
            }",
        );
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Contract(contract) => {
                assert_eq!(contract.name(), "MyContract");
                assert_eq!(contract.circuits.len(), 1);
                let circuit = &contract.circuits[0];
                assert_eq!(circuit.name(), "foo");
                assert_eq!(circuit.arguments.len(), 1);
                let arg = &circuit.arguments[0];
                assert_eq!(arg.name().unwrap(), "x");
                assert!(matches!(arg.ty, Type::Field(_)));
                assert!(matches!(circuit.ty, Type::Field(_)));
                assert!(circuit.body.is_none());
            }
            _ => panic!("Expected contract declaration"),
        }
    }

    #[test]
    fn simple_export_contract() {
        let codebase = build_codebase_wrapper(
            "export contract MyContract {
                circuit foo (x: Field) : Field;
            }",
        );
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Contract(contract) => {
                assert_eq!(contract.name(), "MyContract");
                assert_eq!(contract.circuits.len(), 1);
                let circuit = &contract.circuits[0];
                assert_eq!(circuit.name(), "foo");
                assert_eq!(circuit.arguments.len(), 1);
                let arg = &circuit.arguments[0];
                assert_eq!(arg.name().unwrap(), "x");
                assert!(matches!(arg.ty, Type::Field(_)));
                assert!(matches!(circuit.ty, Type::Field(_)));
                assert!(circuit.body.is_none());
            }
            _ => panic!("Expected contract declaration"),
        }
    }

    #[test]
    fn contract_with_multiple_circuits() {
        let codebase = build_codebase_wrapper(
            "contract MyContract {
                circuit foo (x: Field) : Field;
                pure circuit bar (a: Field, b: Uint<32>) : Field;
            };",
        );
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Contract(contract) => {
                assert_eq!(contract.name(), "MyContract");
                assert_eq!(contract.circuits.len(), 2);
                let circuit = &contract.circuits[0];
                assert_eq!(circuit.name(), "foo");
                assert_eq!(circuit.arguments.len(), 1);
                let arg = &circuit.arguments[0];
                assert_eq!(arg.name().unwrap(), "x");
                assert!(matches!(arg.ty, Type::Field(_)));
                assert!(matches!(circuit.ty, Type::Field(_)));
                assert!(circuit.body.is_none());
                let circuit = &contract.circuits[1];
                assert_eq!(circuit.name(), "bar");
                assert_eq!(circuit.arguments.len(), 2);
                let arg = &circuit.arguments[0];
                assert_eq!(arg.name().unwrap(), "a");
                assert!(matches!(arg.ty, Type::Field(_)));
                let arg = &circuit.arguments[1];
                assert_eq!(arg.name().unwrap(), "b");
                match &arg.ty {
                    Type::Uint(uint_t) => {
                        assert_eq!(uint_t.start.value, 32);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type"),
                }
                assert!(matches!(circuit.ty, Type::Field(_)));
                assert!(circuit.body.is_none());
            }
            _ => panic!("Expected contract declaration"),
        }
    }
}

#[cfg(test)]
mod external_parsing_tests {
    use crate::ast::ty::Type;
    use crate::builder_tests::build_codebase_wrapper;

    #[test]
    fn simple_circuit() {
        let codebase = build_codebase_wrapper("circuit add (x: Field) : Field;");
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
        assert_eq!(circuit.arguments.len(), 1);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        assert!(matches!(arg.ty, Type::Field(_)));
        assert!(matches!(circuit.ty, Type::Field(_)));
        assert!(circuit.body.is_none());
    }

    #[test]
    fn export_circuit() {
        let codebase =
            build_codebase_wrapper("export circuit multiply (a: Field, b: Field) : Field;");
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
        assert_eq!(circuit.name(), "multiply");
        assert_eq!(circuit.arguments.len(), 2);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "a");
        assert!(matches!(arg.ty, Type::Field(_)));
        let arg = circuit.arguments.last().unwrap();
        assert_eq!(arg.name().unwrap(), "b");
        assert!(matches!(arg.ty, Type::Field(_)));
        assert!(matches!(circuit.ty, Type::Field(_)));
        assert!(circuit.body.is_none());
    }

    #[test]
    fn circuit_with_generic_parameters() {
        let codebase = build_codebase_wrapper("circuit process<T> (data: Field) : Field;");
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
        assert_eq!(circuit.name(), "process");
        assert_eq!(circuit.arguments.len(), 1);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "data");
        assert!(matches!(arg.ty, Type::Field(_)));
        assert!(matches!(circuit.ty, Type::Field(_)));
        assert!(circuit.generic_parameters.is_some());
        assert_eq!(circuit.generic_parameters.as_ref().unwrap().len(), 1);
        assert_eq!(
            circuit
                .generic_parameters
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .name,
            "T"
        );
        assert!(circuit.body.is_none());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn circuit_with_multiple_parameters() {
        let codebase =
            build_codebase_wrapper("circuit compute (x: Field, y: Field, z: Uint<32>) : Uint<32>;");
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
        assert_eq!(circuit.name(), "compute");
        assert_eq!(circuit.arguments.len(), 3);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "x");
        assert!(matches!(arg.ty, Type::Field(_)));
        let arg = circuit.arguments.get(1).unwrap();
        assert_eq!(arg.name().unwrap(), "y");
        assert!(matches!(arg.ty, Type::Field(_)));
        let arg = circuit.arguments.last().unwrap();
        assert_eq!(arg.name().unwrap(), "z");
        match &arg.ty {
            Type::Uint(uint_t) => {
                assert_eq!(uint_t.start.value, 32);
                assert!(uint_t.end.is_none());
            }
            _ => panic!("Expected Uint type"),
        }
        assert!(matches!(circuit.ty, Type::Uint(_)));
        match &circuit.ty {
            Type::Uint(uint_t) => {
                assert_eq!(uint_t.start.value, 32);
                assert!(uint_t.end.is_none());
            }
            _ => panic!("Expected Uint type"),
        }
        assert!(circuit.body.is_none());
    }

    #[test]
    fn circuit_with_vector_return_type() {
        let codebase = build_codebase_wrapper("circuit build (a: Field) : Vector<10, Field>;");
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
        assert_eq!(circuit.name(), "build");
        assert_eq!(circuit.arguments.len(), 1);
        let arg = circuit.arguments.first().unwrap();
        assert_eq!(arg.name().unwrap(), "a");
        assert!(matches!(arg.ty, Type::Field(_)));
        match &circuit.ty {
            Type::Vector(vec_t) => {
                match &vec_t.size {
                    crate::ast::ty::VectorSize::Nat(size) => {
                        assert_eq!(size.value, 10);
                    }
                    crate::ast::ty::VectorSize::Ref(_) => panic!("Expected fixed size vector"),
                }
                assert!(matches!(vec_t.ty, Type::Field(_)));
            }
            _ => panic!("Expected Vector type"),
        }
    }
}

#[cfg(test)]
mod import_parsing_tests {
    use crate::{
        ast::{
            declaration::{Declaration, GArgument},
            ty::Type,
        },
        builder_tests::build_codebase_wrapper,
    };

    #[test]
    fn import_with_string_literal() {
        let codebase = build_codebase_wrapper(r#"import "test/corpus/import.txt";"#);
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "\"test/corpus/import.txt\"");
                assert!(import.prefix.is_none());
                assert!(import.generic_parameters.is_none());
            }
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn import_with_identifier() {
        let codebase = build_codebase_wrapper("import id;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "id");
                assert!(import.prefix.is_none());
                assert!(import.generic_parameters.is_none());
            }
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn import_with_identifier_and_parameter() {
        let codebase = build_codebase_wrapper("import id<1>;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "id");
                assert!(import.prefix.is_none());
                assert!(import.generic_parameters.is_some());
                assert_eq!(import.generic_parameters.as_ref().unwrap().len(), 1);
                match &import.generic_parameters.as_ref().unwrap()[0] {
                    GArgument::Nat(nat) => {
                        assert_eq!(nat.value, 1);
                    }
                    other @ GArgument::Type(_) => {
                        panic!("Expected natural number, got {other:?}")
                    }
                }
            }
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn import_with_module_and_parameters() {
        let codebase = build_codebase_wrapper("import myModule<42, Boolean>;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "myModule");
                assert!(import.prefix.is_none());
                assert!(import.generic_parameters.is_some());
                assert_eq!(import.generic_parameters.as_ref().unwrap().len(), 2);
                let params = import.generic_parameters.as_ref().unwrap();
                match &params[0] {
                    GArgument::Nat(nat) => {
                        assert_eq!(nat.value, 42);
                    }
                    GArgument::Type(_) => panic!("Expected natural number"),
                }
                assert!(matches!(params[1], GArgument::Type(Type::Boolean(_))));
            }
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn import_with_module_and_prefix() {
        let codebase = build_codebase_wrapper("import myModule prefix helper;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "myModule");
                assert!(import.prefix.is_some());
                assert_eq!(import.prefix.as_ref().unwrap().name, "helper");
                assert!(import.generic_parameters.is_none());
            }
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn import_with_module_parameters_and_prefix() {
        let codebase = build_codebase_wrapper("import myModule<42, Field> prefix helper;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "myModule");
                assert!(import.prefix.is_some());
                assert_eq!(import.prefix.as_ref().unwrap().name, "helper");
                assert!(import.generic_parameters.is_some());
                assert_eq!(import.generic_parameters.as_ref().unwrap().len(), 2);
            }
            _ => panic!("Expected import declaration"),
        }
    }

    #[test]
    fn import_with_string_literal_and_prefix() {
        let codebase = build_codebase_wrapper(r#"import "module/file" prefix myHelper;"#);
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "\"module/file\"");
                assert!(import.prefix.is_some());
                assert_eq!(import.prefix.as_ref().unwrap().name, "myHelper");
                assert!(import.generic_parameters.is_none());
            }
            _ => panic!("Expected import declaration"),
        }
    }
}

#[cfg(test)]
mod include_parsing_tests {
    use crate::{ast::declaration::Declaration, builder_tests::build_codebase_wrapper};

    #[test]
    fn include_with_string_literal() {
        let codebase = build_codebase_wrapper(r#"include "test/corpus/include.txt";"#);
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Include(include) => {
                assert_eq!(include.path, "\"test/corpus/include.txt\"");
            }
            _ => panic!("Expected include declaration"),
        }
    }
}

#[cfg(test)]
mod ledger_parsing_tests {
    use crate::{
        ast::{declaration::Declaration, ty::Type},
        builder_tests::build_codebase_wrapper,
    };

    #[test]
    fn simple_ledger() {
        let codebase = build_codebase_wrapper("ledger myLedger : Field;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "myLedger");
                assert!(matches!(ledger.ty, Type::Field(_)));
                assert!(!ledger.is_exported);
                assert!(!ledger.is_sealed);
            }
            _ => panic!("Expected ledger declaration"),
        }
    }

    #[test]
    fn export_ledger() {
        let codebase = build_codebase_wrapper("export ledger myLedger : Boolean;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "myLedger");
                assert!(matches!(ledger.ty, Type::Boolean(_)));
                assert!(ledger.is_exported);
                assert!(!ledger.is_sealed);
            }
            _ => panic!("Expected ledger declaration"),
        }
    }

    #[test]
    fn sealed_ledger() {
        let codebase = build_codebase_wrapper("sealed ledger myLedger : Uint<32>;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "myLedger");
                match &ledger.ty {
                    Type::Uint(uint_t) => {
                        assert_eq!(uint_t.start.value, 32);
                        assert!(uint_t.end.is_none());
                    }
                    _ => panic!("Expected Uint type"),
                }
                assert!(!ledger.is_exported);
                assert!(ledger.is_sealed);
            }
            _ => panic!("Expected ledger declaration"),
        }
    }

    #[test]
    fn export_sealed_ledger() {
        let codebase = build_codebase_wrapper("export sealed ledger myLedger : Field;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "myLedger");
                assert!(matches!(ledger.ty, Type::Field(_)));
                assert!(ledger.is_exported);
                assert!(ledger.is_sealed);
            }
            _ => panic!("Expected ledger declaration"),
        }
    }

    #[test]
    fn ledger_with_vector_type() {
        let codebase = build_codebase_wrapper("ledger myLedger : Vector<10, Boolean>;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "myLedger");
                match &ledger.ty {
                    Type::Vector(vec_t) => {
                        match &vec_t.size {
                            crate::ast::ty::VectorSize::Nat(size) => {
                                assert_eq!(size.value, 10);
                            }
                            crate::ast::ty::VectorSize::Ref(_) => {
                                panic!("Expected fixed size vector")
                            }
                        }
                        assert!(matches!(vec_t.ty, Type::Boolean(_)));
                    }
                    _ => panic!("Expected Vector type"),
                }
                assert!(!ledger.is_exported);
                assert!(!ledger.is_sealed);
            }
            _ => panic!("Expected ledger declaration"),
        }
    }

    #[test]
    fn ledger_with_generic_type() {
        let codebase = build_codebase_wrapper("ledger myLedger : MyType<42, Field>;");
        let codebase = codebase.borrow();
        assert_eq!(codebase.fname_ast_map.len(), 1);
        assert_eq!(codebase.symbol_tables.len(), 1);
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.declarations.len(), 1);
        match &ast.declarations[0] {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "myLedger");
                match &ledger.ty {
                    Type::Ref(gen_t) => {
                        assert_eq!(gen_t.name(), "MyType");
                        assert_eq!(gen_t.generic_parameters.as_ref().unwrap().len(), 2);
                        match &gen_t.generic_parameters.as_ref().unwrap()[0] {
                            crate::ast::declaration::GArgument::Nat(nat) => {
                                assert_eq!(nat.value, 42);
                            }
                            crate::ast::declaration::GArgument::Type(_) => {
                                panic!("Expected natural number")
                            }
                        }
                        assert!(matches!(
                            gen_t.generic_parameters.as_ref().unwrap()[1],
                            crate::ast::declaration::GArgument::Type(Type::Field(_))
                        ));
                    }
                    _ => panic!("Expected Generic type"),
                }
                assert!(!ledger.is_exported);
                assert!(!ledger.is_sealed);
            }
            _ => panic!("Expected ledger declaration"),
        }
    }
}

#[cfg(test)]
mod pragma_parsing_tests {

    use crate::ast::directive::{Directive, VersionExpr};
    use crate::ast::literal::VersionOperator;
    use crate::builder_tests::build_codebase_wrapper;

    #[test]
    fn pragma_language_version_eq() {
        let codebase = build_codebase_wrapper("pragma language_version 0.14.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        // Expect one pragma directive.
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        // Default operator is equality.
                        assert_eq!(v.operator, VersionOperator::Eq);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_neq() {
        let codebase = build_codebase_wrapper("pragma language_version !0.14.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        // "!" should yield a not-equal operator.
                        assert_eq!(v.operator, VersionOperator::Neq);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_lt() {
        let codebase = build_codebase_wrapper("pragma language_version <0.14.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        assert_eq!(v.operator, VersionOperator::Lt);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_lte() {
        let codebase = build_codebase_wrapper("pragma language_version <=0.14.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        assert_eq!(v.operator, VersionOperator::Le);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_gt() {
        let codebase = build_codebase_wrapper("pragma language_version >0.14.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        assert_eq!(v.operator, VersionOperator::Gt);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_gte() {
        let codebase = build_codebase_wrapper("pragma language_version >=0.14.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        assert_eq!(v.operator, VersionOperator::Ge);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_parenthesized() {
        let codebase = build_codebase_wrapper("pragma language_version (0.14.0);");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                // Parentheses should not affect the simple version expression.
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        assert_eq!(v.operator, VersionOperator::Eq);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_parenthesized_not() {
        let codebase = build_codebase_wrapper("pragma language_version (!0.14.0);");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Version(v) => {
                        assert_eq!(v.operator, VersionOperator::Neq);
                        assert_eq!(v.major.value, 0);
                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                    }
                    _ => panic!("Expected simple version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_and() {
        let codebase = build_codebase_wrapper("pragma language_version 0.14.0 && 0.15.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::And(left, right) => {
                        // Left expression: 0.14.0 with default equality.
                        match &**left {
                            VersionExpr::Version(v_left) => {
                                assert_eq!(v_left.operator, VersionOperator::Eq);
                                assert_eq!(v_left.major.value, 0);
                                assert_eq!(v_left.minor.as_ref().unwrap().value, 14);
                                assert_eq!(v_left.bugfix.as_ref().unwrap().value, 0);
                            }
                            _ => panic!("Expected version expression on left side"),
                        }
                        // Right expression: 0.15.0 with default equality.
                        match &**right {
                            VersionExpr::Version(v_right) => {
                                assert_eq!(v_right.operator, VersionOperator::Eq);
                                assert_eq!(v_right.major.value, 0);
                                assert_eq!(v_right.minor.as_ref().unwrap().value, 15);
                                assert_eq!(v_right.bugfix.as_ref().unwrap().value, 0);
                            }
                            _ => panic!("Expected version expression on right side"),
                        }
                    }
                    _ => panic!("Expected binary version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_or() {
        let codebase = build_codebase_wrapper("pragma language_version 0.14.0 || 0.15.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Or(left, right) => {
                        match &**left {
                            VersionExpr::Version(v_left) => {
                                assert_eq!(v_left.operator, VersionOperator::Eq);
                                assert_eq!(v_left.major.value, 0);
                                assert_eq!(v_left.minor.as_ref().unwrap().value, 14);
                                assert_eq!(v_left.bugfix.as_ref().unwrap().value, 0);
                            }
                            _ => panic!("Expected version expression on left side"),
                        }
                        match &**right {
                            VersionExpr::Version(v_right) => {
                                assert_eq!(v_right.operator, VersionOperator::Eq);
                                assert_eq!(v_right.major.value, 0);
                                assert_eq!(v_right.minor.as_ref().unwrap().value, 15);
                                assert_eq!(v_right.bugfix.as_ref().unwrap().value, 0);
                            }
                            _ => panic!("Expected version expression on right side"),
                        }
                    }
                    _ => panic!("Expected an Or version expression"),
                }
            }
        }
    }

    #[test]
    fn pragma_language_version_and_or() {
        let codebase =
            build_codebase_wrapper("pragma language_version 0.14.0 && 0.15.0 || 0.16.0;");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        assert_eq!(ast.directives.len(), 1);
        match &ast.directives[0] {
            Directive::Pragma(pragma) => {
                assert_eq!(pragma.name(), "language_version");
                match &pragma.version {
                    VersionExpr::Or(outer_left, outer_right) => {
                        // Left side should be an And expression: 0.14.0 && 0.15.0
                        match &**outer_left {
                            VersionExpr::And(inner_left, inner_right) => {
                                match &**inner_left {
                                    VersionExpr::Version(v) => {
                                        assert_eq!(v.operator, VersionOperator::Eq);
                                        assert_eq!(v.major.value, 0);
                                        assert_eq!(v.minor.as_ref().unwrap().value, 14);
                                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                                    }
                                    _ => {
                                        panic!("Expected version expression on inner left side")
                                    }
                                }
                                match &**inner_right {
                                    VersionExpr::Version(v) => {
                                        assert_eq!(v.operator, VersionOperator::Eq);
                                        assert_eq!(v.major.value, 0);
                                        assert_eq!(v.minor.as_ref().unwrap().value, 15);
                                        assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                                    }
                                    _ => panic!("Expected version expression on inner right side"),
                                }
                            }
                            _ => panic!("Expected And version expression for the left side of Or"),
                        }
                        // Outer right should be the version 0.16.0.
                        match &**outer_right {
                            VersionExpr::Version(v) => {
                                assert_eq!(v.operator, VersionOperator::Eq);
                                assert_eq!(v.major.value, 0);
                                assert_eq!(v.minor.as_ref().unwrap().value, 16);
                                assert_eq!(v.bugfix.as_ref().unwrap().value, 0);
                            }
                            _ => panic!("Expected version expression on outer right side"),
                        }
                    }
                    _ => panic!("Expected an Or version expression"),
                }
            }
        }
    }
}

#[cfg(test)]
mod statements_parsing_tests {

    use crate::ast::{
        declaration::Pattern,
        expression::{
            BinaryExpressionOperator, Expression, StructExprArg, UnaryExpressionOperator,
        },
        function::Function,
        literal::Literal,
        statement::{AssignOperator, Statement},
        ty::Type,
    };

    use super::build_codebase_wrapper;

    #[test]
    fn assign_statement_simple() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x = 42; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Assign(assign) => {
                match &assign.target {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.name, "x");
                    }
                    _ => panic!("Expected identifier expression"),
                }
                match &assign.value {
                    Expression::Literal(Literal::Nat(lit)) => {
                        assert_eq!(lit.value, 42);
                    }
                    _ => panic!("Expected literal expression"),
                }
                assert_eq!(assign.operator, AssignOperator::Simple);
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn assign_statement_add() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x += y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Assign(assign) => {
                match &assign.target {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.name, "x");
                    }
                    _ => panic!("Expected identifier expression"),
                }
                match &assign.value {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.name, "y");
                    }
                    _ => panic!("Expected identifier expression"),
                }
                assert_eq!(assign.operator, AssignOperator::Add);
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn expression_statement_add() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x + y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin) => {
                    match &bin.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    assert_eq!(bin.operator, BinaryExpressionOperator::Add);
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn return_statement() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { return; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Return(return_stmt) => {
                assert!(return_stmt.value.is_none());
            }
            _ => panic!("Expected return statement"),
        }
    }

    #[test]
    fn return_statement_with_expression() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { return x * y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
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
                                    }
                                    _ => panic!("Expected identifier"),
                                }
                                match &op.right {
                                    Expression::Identifier(ident) => {
                                        assert_eq!(ident.name, "y");
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
    fn if_statement() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { if (x > 0) x = 1; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::If(if_stmt) => {
                match &if_stmt.condition {
                    Expression::Sequence(seq_expr) => {
                        assert_eq!(seq_expr.expressions.len(), 1);
                        let expr = seq_expr.expressions.first().unwrap();
                        let Expression::Binary(op) = expr else {
                            panic!("Expected binary expression, found {expr:?}");
                        };
                        assert_eq!(
                            op.operator,
                            crate::ast::expression::BinaryExpressionOperator::Gt
                        );
                        match &op.left {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &op.right {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 0);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression, found {:?}", if_stmt.condition),
                }
                if let Statement::Assign(assign_stmt) = &if_stmt.then_branch {
                    assert!(assign_stmt.operator == AssignOperator::Simple);
                    match &assign_stmt.target {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier"),
                    }
                    match &assign_stmt.value {
                        Expression::Literal(Literal::Nat(literal)) => {
                            assert_eq!(literal.value, 1);
                        }
                        _ => panic!("Expected literal expression"),
                    }
                } else {
                    panic!("Expected assignment statement");
                }
                assert!(if_stmt.else_branch.is_none());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn if_else_statement() {
        let codebase = build_codebase_wrapper(
            "circuit foo(): Bool { if (x > 0) { return x + 1; } else { x = 21; } }",
        );
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::If(if_stmt) => {
                match &if_stmt.condition {
                    Expression::Sequence(seq_expr) => {
                        assert_eq!(seq_expr.expressions.len(), 1);
                        let expr = seq_expr.expressions.first().unwrap();
                        let Expression::Binary(op) = expr else {
                            panic!("Expected binary expression, found {expr:?}");
                        };
                        assert_eq!(
                            op.operator,
                            crate::ast::expression::BinaryExpressionOperator::Gt
                        );
                        match &op.left {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &op.right {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 0);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression, found {:?}", if_stmt.condition),
                }
                if let Statement::Block(block) = &if_stmt.then_branch {
                    let stmt = block.statements.first().unwrap();
                    match stmt {
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
                                                }
                                                _ => panic!("Expected identifier"),
                                            }
                                            match &op.right {
                                                Expression::Literal(Literal::Nat(literal)) => {
                                                    assert_eq!(literal.value, 1);
                                                }
                                                _ => panic!("Expected literal expression"),
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
                assert!(if_stmt.else_branch.is_some());
                if let Some(Statement::Block(block)) = &if_stmt.else_branch {
                    let stmt = block.statements.first().unwrap();
                    match stmt {
                        Statement::Assign(assign_stmt) => {
                            assert!(assign_stmt.operator == AssignOperator::Simple);
                            match &assign_stmt.target {
                                Expression::Identifier(ident) => {
                                    assert_eq!(ident.name, "x");
                                }
                                _ => panic!("Expected identifier"),
                            }
                            match &assign_stmt.value {
                                Expression::Literal(Literal::Nat(literal)) => {
                                    assert_eq!(literal.value, 21);
                                }
                                _ => panic!("Expected literal expression"),
                            }
                        }
                        _ => panic!("Expected assignment statement"),
                    }
                }
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn for_loop_statement() {
        let codebase =
            build_codebase_wrapper("circuit foo(): Bool { for (const i of 0 .. 10) { x = i; } }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::For(for_stmt) => {
                assert_eq!(for_stmt.counter.name, "i");
                assert!(for_stmt.range.is_some());
                let (start, end) = for_stmt.range.as_ref().unwrap();
                assert_eq!(start.value, 0);
                assert_eq!(end.value, 10);
                assert_eq!(for_stmt.body.statements.len(), 1);
                let stmt = for_stmt.body.statements.first().unwrap();
                match stmt {
                    Statement::Assign(assign_stmt) => {
                        assert!(assign_stmt.operator == AssignOperator::Simple);
                        match &assign_stmt.target {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &assign_stmt.value {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "i");
                            }
                            _ => panic!("Expected identifier"),
                        }
                    }
                    _ => panic!("Expected assignment statement"),
                }
            }
            _ => panic!("Expected for loop statement"),
        }
    }

    #[test]
    fn assert_statement() {
        let codebase = build_codebase_wrapper(
            r#"circuit foo(): Bool { assert x != 0 "Division by zero error"; }"#,
        );
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Assert(assert_stmt) => {
                match &assert_stmt.condition {
                    Expression::Binary(op) => {
                        assert_eq!(op.operator, BinaryExpressionOperator::Ne);
                        match &op.left {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &op.right {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 0);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
                assert_eq!(assert_stmt.message().unwrap(), "\"Division by zero error\"");
            }
            _ => panic!("Expected assert statement"),
        }
    }

    #[test]
    fn assert_statement_with_message() {
        let codebase = build_codebase_wrapper(r#"circuit foo(): Bool { assert x > 0 "fail!"; }"#);
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Assert(assert_stmt) => {
                match &assert_stmt.condition {
                    Expression::Binary(op) => {
                        assert_eq!(op.operator, BinaryExpressionOperator::Gt);
                        match &op.left {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &op.right {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 0);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
                assert_eq!(assert_stmt.message().unwrap(), "\"fail!\"");
            }
            _ => panic!("Expected assert statement"),
        }
    }

    #[test]
    fn const_declaration_statement() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { const y: Field = x + 1; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Const(decl) => {
                match &decl.pattern {
                    Pattern::Identifier(ident) => {
                        assert_eq!(ident.name, "y");
                    }
                    _ => panic!("Expected identifier pattern"),
                }
                assert!(decl.ty.is_some());
                assert!(matches!(decl.ty.as_ref().unwrap(), Type::Field(_)));
                match &decl.value {
                    Expression::Binary(op) => {
                        assert_eq!(op.operator, BinaryExpressionOperator::Add);
                        match &op.left {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &op.right {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 1);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected const declaration statement"),
        }
    }

    #[test]
    fn const_declaration_with_destructuring() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { const [a, b] = getPair(); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Const(decl) => {
                match &decl.pattern {
                    Pattern::Tuple(t_pattern) => {
                        assert_eq!(t_pattern.patterns.len(), 2);
                        match &t_pattern.patterns[0] {
                            Pattern::Identifier(ident) => {
                                assert_eq!(ident.name, "a");
                            }
                            _ => panic!("Expected identifier pattern"),
                        }
                        match &t_pattern.patterns[1] {
                            Pattern::Identifier(ident) => {
                                assert_eq!(ident.name, "b");
                            }
                            _ => panic!("Expected identifier pattern"),
                        }
                    }
                    _ => panic!("Expected tuple pattern"),
                }
                assert!(decl.ty.is_none());
                match &decl.value {
                    Expression::FunctionCall(func_call) => match &func_call.function {
                        Expression::Function(func) => match func {
                            Function::Named(named_func) => {
                                assert_eq!(named_func.name.name, "getPair");
                            }
                            Function::Anonymous(_) => panic!("Expected named function"),
                        },
                        _ => panic!("Expected identifier expression"),
                    },
                    _ => panic!("Expected function call expression, found {:?}", decl.value),
                }
            }
            _ => panic!("Expected const declaration statement"),
        }
    }

    #[test]
    fn block_statement() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { { x = 1; return; } }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::Block(block) => {
                assert_eq!(block.statements.len(), 2);
                match &block.statements[0] {
                    Statement::Assign(assign_stmt) => {
                        assert!(assign_stmt.operator == AssignOperator::Simple);
                        match &assign_stmt.target {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "x");
                            }
                            _ => panic!("Expected identifier"),
                        }
                        match &assign_stmt.value {
                            Expression::Literal(Literal::Nat(literal)) => {
                                assert_eq!(literal.value, 1);
                            }
                            _ => panic!("Expected literal expression"),
                        }
                    }
                    _ => panic!("Expected assignment statement"),
                }
                match &block.statements[1] {
                    Statement::Return(return_stmt) => {
                        assert!(return_stmt.value.is_none());
                    }
                    _ => panic!("Expected return statement"),
                }
            }
            _ => panic!("Expected block statement"),
        }
    }

    #[test]
    fn expression_statement_true() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { true; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Literal(Literal::Bool(lit)) => {
                    assert!(lit.value);
                }
                _ => panic!("Expected boolean literal expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_literal() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { 42; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Literal(Literal::Nat(lit)) => {
                    assert_eq!(lit.value, 42);
                }
                _ => panic!("Expected natural number literal expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_string_literal() {
        let codebase = build_codebase_wrapper(r#"circuit foo(): Bool { "hello"; }"#);
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Literal(Literal::Str(lit)) => {
                    assert_eq!(lit.value, "\"hello\"");
                }
                _ => panic!("Expected string literal expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_function_call() {
        let codebase = build_codebase_wrapper(r#"circuit foo(): Bool { pad(5, "hi"); }"#);
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Literal(Literal::Pad(pad)) => {
                    assert_eq!(pad.number.value, 5);
                    assert_eq!(pad.name.value, "\"hi\"");
                }
                _ => panic!("Expected pad literal expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_default() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { default<Field>; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Default(default_expr) => {
                    assert!(matches!(default_expr, Type::Field(_)));
                }
                _ => panic!("Expected default expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_function_call_with_multiple_arguments() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { map(f, x, y); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Map(map_expr) => {
                    match &map_expr.function {
                        Function::Named(named_func) => {
                            assert_eq!(named_func.name(), "f");
                        }
                        Function::Anonymous(_) => panic!("Expected named function"),
                    }
                    assert_eq!(map_expr.expressions.len(), 2);
                    match &map_expr.expressions[0] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &map_expr.expressions[1] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected map expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_fold() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { fold(g, 0, x, y, z); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Fold(fold_expr) => {
                    match &fold_expr.function {
                        Function::Named(named_func) => {
                            assert_eq!(named_func.name(), "g");
                        }
                        Function::Anonymous(_) => panic!("Expected named function"),
                    }
                    match &fold_expr.initial_value {
                        Expression::Literal(Literal::Nat(lit)) => {
                            assert_eq!(lit.value, 0);
                        }
                        _ => panic!("Expected literal expression"),
                    }
                    assert_eq!(fold_expr.expressions.len(), 3);
                    match &fold_expr.expressions[0] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &fold_expr.expressions[1] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &fold_expr.expressions[2] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "z");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected fold expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_function_call_with_two_arguments() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { myFunction(a, b); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::FunctionCall(func_call) => {
                    match &func_call.function {
                        Expression::Function(Function::Named(named_func)) => {
                            assert_eq!(named_func.name.name, "myFunction");
                        }
                        _ => panic!("Expected named function"),
                    }
                    assert_eq!(func_call.arguments.len(), 2);
                    match &func_call.arguments[0] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "a");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &func_call.arguments[1] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "b");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected function call expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_function_call_no_arguments() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { myFunction(); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::FunctionCall(func_call) => {
                    match &func_call.function {
                        Expression::Function(Function::Named(named_func)) => {
                            assert_eq!(named_func.name.name, "myFunction");
                        }
                        _ => panic!("Expected named function"),
                    }
                    assert_eq!(func_call.arguments.len(), 0);
                }
                _ => panic!("Expected function call expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_disclose() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { disclose(x); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Disclose(disclose_expr) => match &disclose_expr.expression {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.name, "x");
                    }
                    _ => panic!("Expected identifier expression"),
                },
                _ => panic!("Expected disclose expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn struct_initialization_statement() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { MyStruct { a: x, b: y }; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Struct(struct_init) => {
                    match &struct_init.ty {
                        Type::Ref(r) => {
                            assert_eq!(r.name(), "MyStruct");
                        }
                        _ => panic!("Expected reference type"),
                    }
                    assert_eq!(struct_init.args.len(), 2);
                    let field_a = &struct_init.args[0];
                    match field_a {
                        StructExprArg::NamedField(field) => {
                            assert_eq!(field.name.name, "a");
                            match &field.value {
                                Expression::Identifier(ident) => {
                                    assert_eq!(ident.name, "x");
                                }
                                _ => panic!("Expected identifier expression for field 'a'"),
                            }
                        }
                        _ => panic!("Expected named field"),
                    }
                    let field_b = &struct_init.args[1];
                    match field_b {
                        StructExprArg::NamedField(field) => {
                            assert_eq!(field.name.name, "b");
                            match &field.value {
                                Expression::Identifier(ident) => {
                                    assert_eq!(ident.name, "y");
                                }
                                _ => panic!("Expected identifier expression for field 'b'"),
                            }
                        }
                        _ => panic!("Expected named field"),
                    }
                }
                _ => panic!("Expected struct initialization expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_array() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { [x, y, z]; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Literal(Literal::Array(array_lit)) => {
                    assert_eq!(array_lit.elements.len(), 3);
                    match &array_lit.elements[0] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &array_lit.elements[1] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &array_lit.elements[2] {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "z");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected array expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_tuple() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { ((x + 1), y, z); }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => {
                assert_eq!(expr.expressions.len(), 1);
                match &expr.expressions[0] {
                    Expression::Sequence(expt_seq) => {
                        assert_eq!(expt_seq.expressions.len(), 3);
                        match &expt_seq.expressions[0] {
                            Expression::Sequence(inner_seq) => {
                                assert_eq!(inner_seq.expressions.len(), 1);
                                match &inner_seq.expressions[0] {
                                    Expression::Binary(bin_expr) => {
                                        assert_eq!(
                                            bin_expr.operator,
                                            BinaryExpressionOperator::Add
                                        );
                                    }
                                    _ => panic!("Expected binary expression"),
                                }
                            }
                            _ => panic!("Expected sequence expression"),
                        }
                        match &expt_seq.expressions[1] {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "y");
                            }
                            _ => panic!("Expected identifier expression"),
                        }
                        match &expt_seq.expressions[2] {
                            Expression::Identifier(ident) => {
                                assert_eq!(ident.name, "z");
                            }
                            _ => panic!("Expected identifier expression"),
                        }
                    }
                    _ => panic!("Expected sequence expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn conditional_expression() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x > 0 ? x : 10 * x; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Conditional(conditional_expr) => {
                    match &conditional_expr.condition {
                        Expression::Binary(op) => {
                            assert_eq!(op.operator, BinaryExpressionOperator::Gt);
                            match &op.left {
                                Expression::Identifier(ident) => {
                                    assert_eq!(ident.name, "x");
                                }
                                _ => panic!("Expected identifier"),
                            }
                            match &op.right {
                                Expression::Literal(Literal::Nat(literal)) => {
                                    assert_eq!(literal.value, 0);
                                }
                                _ => panic!("Expected literal expression"),
                            }
                        }
                        _ => panic!("Expected binary expression"),
                    }
                    match &conditional_expr.then_branch {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier"),
                    }
                    match &conditional_expr.else_branch {
                        Expression::Binary(op) => {
                            assert_eq!(op.operator, BinaryExpressionOperator::Mul);
                            match &op.left {
                                Expression::Literal(Literal::Nat(literal)) => {
                                    assert_eq!(literal.value, 10);
                                }
                                _ => panic!("Expected literal expression"),
                            }
                            match &op.right {
                                Expression::Identifier(ident) => {
                                    assert_eq!(ident.name, "x");
                                }
                                _ => panic!("Expected identifier"),
                            }
                        }
                        _ => panic!("Expected binary expression"),
                    }
                }
                _ => panic!("Expected ternary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_logical_or() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x || y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin_expr) => {
                    assert_eq!(bin_expr.operator, BinaryExpressionOperator::Or);
                    match &bin_expr.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin_expr.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_logical_and() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x && y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin_expr) => {
                    assert_eq!(bin_expr.operator, BinaryExpressionOperator::And);
                    match &bin_expr.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin_expr.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_equality() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x == y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin_expr) => {
                    assert_eq!(bin_expr.operator, BinaryExpressionOperator::Eq);
                    match &bin_expr.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin_expr.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_add_1() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x + y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin_expr) => {
                    assert_eq!(bin_expr.operator, BinaryExpressionOperator::Add);
                    match &bin_expr.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin_expr.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_subtract() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x - y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin_expr) => {
                    assert_eq!(bin_expr.operator, BinaryExpressionOperator::Sub);
                    match &bin_expr.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin_expr.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_multiply() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x * y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Binary(bin_expr) => {
                    assert_eq!(bin_expr.operator, BinaryExpressionOperator::Mul);
                    match &bin_expr.left {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    match &bin_expr.right {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "y");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected binary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_not() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { !x; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Unary(unary_expr) => {
                    assert_eq!(unary_expr.operator, UnaryExpressionOperator::Not);
                    match &unary_expr.operand {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                }
                _ => panic!("Expected unary expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_member_access() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x.y; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::MemberAccess(member_access) => {
                    match &member_access.base {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    assert_eq!(member_access.member.name, "y");
                }
                _ => panic!("Expected member access expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_array_access() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x[0]; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::IndexAccess(array_access) => {
                    match &array_access.base {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    assert_eq!(array_access.index.value, 0);
                }
                _ => panic!("Expected array access expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn expression_statement_cast() {
        let codebase = build_codebase_wrapper("circuit foo(): Bool { x as Field; }");
        let codebase = codebase.borrow();
        let source_file = codebase.fname_ast_map.get("dummy").unwrap();
        let ast = &source_file.ast;
        let circuits = ast.circuits();
        assert_eq!(circuits.len(), 1);
        let statement = circuits
            .first()
            .unwrap()
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap();
        match statement {
            Statement::ExpressionSequence(expr) => match &expr.expressions[0] {
                Expression::Cast(cast_expr) => {
                    match &cast_expr.expression {
                        Expression::Identifier(ident) => {
                            assert_eq!(ident.name, "x");
                        }
                        _ => panic!("Expected identifier expression"),
                    }
                    assert!(matches!(cast_expr.target_type, Type::Field(_)));
                }
                _ => panic!("Expected cast expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }
}
