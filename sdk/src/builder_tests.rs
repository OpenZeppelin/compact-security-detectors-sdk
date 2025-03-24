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
mod external_contract_parsing_test {
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
mod external_parsing_test {
    use crate::{ast::ty::Type, builder_tests::build_codebase_wrapper};

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
