#![warn(clippy::pedantic)]
use crate::ast::{declaration::Argument, expression::Identifier};
use crate::ast::{
    declaration::{Declaration, Import, Pattern},
    definition::{Circuit, Definition, Module},
    directive::{Directive, Pragma},
    program::Program,
    statement::{Block, Statement},
    ty::{Ref, Type},
};
use anyhow::{anyhow, bail, Ok, Result};
use ast::declaration::{
    Export, Ledger, PatternArgument, StructPattern, StructPatternField, TuplePattern, Witness,
};
use ast::directive::VersionExpr;
use ast::expression::{
    Binary, BinaryExpressionOperator, Conditional, Disclose, Expression, Fold, Map, Sequence,
    Unary, UnaryExpressionOperator,
};
use ast::function::{AnonymousFunction, Function, FunctionArgument, NamedFunction};
use ast::literal::{Bool, Literal, Nat, Str, Version, VersionOperator};
use ast::node::Location;
use ast::statement::{Assert, Assign, AssignOperator, Const, For, If, Return};
use ast::ty::{Bytes, Opaque, Sum, TypeBool, TypeField, Uint, Vector};
use std::rc::Rc;
use tree_sitter::Node;
pub mod ast;
pub mod codebase;
mod passes;

pub fn parse_content(fname: &str, source_code: &str) -> anyhow::Result<codebase::SourceCodeFile> {
    let compact_language = tree_sitter_compact::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&compact_language)
        .expect("Error loading Inference grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    let ast = build_ast(&root_node, source_code)?;
    let source_code_file = codebase::SourceCodeFile {
        fname: fname.to_string(),
        ast,
    };
    Ok(source_code_file)
}

pub fn build_ast(root: &Node, source: &str) -> Result<Program> {
    if root.kind() != "source_file" {
        bail!("Expected source_file as root, found {}", root.kind());
    }

    let mut directives: Vec<Directive> = Vec::new();
    let mut declarations: Vec<Declaration> = Vec::new();
    let mut definitions: Vec<Definition> = Vec::new();
    let mut statements: Vec<Statement> = Vec::new();
    let mut modules: Vec<Rc<Module>> = Vec::new();

    for i in 0..root.named_child_count() {
        let child = root.named_child(i).unwrap();
        match child.kind() {
            "pragma" => {
                let pragma = build_pragma(&child, source)?;
                directives.push(Directive::Pragma(Rc::new(pragma)));
            }
            // import declaration
            "idecl" => {
                let import_decl = build_import(&child, source)?;
                declarations.push(Declaration::Import(Rc::new(import_decl)));
            }
            // export declaration
            "xdecl" => {
                let export_decl = build_export(&child, source)?;
                declarations.push(Declaration::Export(Rc::new(export_decl)));
            }
            // ledger declaration
            "ldecl" => {
                let ledger_decl = build_ledger(&child, source)?;
                declarations.push(Declaration::Ledger(Rc::new(ledger_decl)));
            }
            // // witness declaration
            "wdecl" => {
                let witness_decl = build_witness(&child, source)?;
                declarations.push(Declaration::Witness(Rc::new(witness_decl)));
            }
            // circuit definition
            "cdefn" => {
                let circuit = build_circuit(&child, source)?;
                definitions.push(Definition::Circuit(Rc::new(circuit)));
            }
            // // struct definition
            // "struct" => {
            //     let structure = build_structure(&child, source)?;
            //     definitions.push(Definition::Structure(Rc::new(structure)));
            // }
            // // constructor definition
            // "lconstructor" => {
            //     let constructor = build_constructor(&child, source)?;
            //     definitions.push(Definition::Constructor(Rc::new(constructor)));
            // }
            other => bail!("Unhandled node kind: {}", other),
        }
    }
    Ok(Program {
        directives,
        declarations,
        definitions,
        statements,
        modules,
    })
}

pub fn build_pragma(node: &Node, source: &str) -> Result<Pragma> {
    let id_node = node
        .child_by_field_name("id")
        .ok_or_else(|| anyhow!("Missing 'id' field in pragma"))?;
    let identifier = build_identifier(&id_node, source)?;

    let mut start = 1;
    let mut version_expressions: Vec<VersionExpr> = Vec::new();

    while start < node.named_child_count() {
        let child = node.named_child(start).unwrap();
        match child.kind() {
            "not"
            | "greater_than"
            | "less_than"
            | "greater_than_or_equal"
            | "less_than_or_equal" => {
                let version = build_version_with_operator(&child, source)?;
                version_expressions.push(VersionExpr::Version(version.clone()));
                start += 1;
            }
            "version" => {
                let version = build_version(&child, VersionOperator::Eq, source)?;
                version_expressions.push(VersionExpr::Version(version.clone()));
            }
            &_ => {}
        }
        start += 1;
    }

    if version_expressions.len() == 1 {
        let pragma = Pragma {
            id: node_id(),
            location: location(node),
            version: version_expressions.first().unwrap().clone(),
            value: identifier,
        };

        return Ok(pragma);
    }
    bail!("Invalid pragma structure");
}

fn build_version_with_operator(node: &Node, source: &str) -> Result<Rc<Version>> {
    let version_operator = match node.kind() {
        "greater_than" => VersionOperator::Gt,
        "greater_than_or_equal" => VersionOperator::Ge,
        "less_than" => VersionOperator::Lt,
        "less_than_or_equal" => VersionOperator::Le,
        "not" => VersionOperator::Neq,
        _ => bail!("Invalid version operator"),
    };
    let version_node = &node.next_named_sibling().ok_or_else(|| {
        anyhow!(
            "Missing version node in version expression: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    build_version(version_node, version_operator, source)
}

fn build_version(
    version_node: &Node,
    version_operator: VersionOperator,
    source: &str,
) -> Result<Rc<Version>> {
    let version_text = version_node.utf8_text(source.as_bytes())?;
    let parts: Vec<&str> = version_text.split('.').collect();
    let major = parts
        .first()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or_else(|| anyhow!("Invalid version format: {}", version_text))?;
    let minor = parts
        .get(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let bugfix = parts.get(2).and_then(|s| s.parse::<u64>().ok());
    Ok(Rc::new(Version {
        id: node_id(),
        location: location(version_node),
        major: Rc::new(Nat {
            id: node_id(),
            location: location(version_node),
            value: major,
        }),
        minor: Some(Rc::new(Nat {
            id: node_id(),
            location: location(version_node),
            value: minor,
        })),
        bugfix: bugfix.map(|b| {
            Rc::new(Nat {
                id: node_id(),
                location: location(version_node),
                value: b,
            })
        }),
        operator: version_operator,
    }))
}

fn build_import(node: &Node, source: &str) -> Result<Import> {
    let import_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in import declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let identifier = build_identifier(&import_name_node, source)?;
    Ok(Import {
        id: node_id(),
        location: location(node),
        value: identifier,
    })
}

fn build_export(node: &Node, source: &str) -> Result<Export> {
    let mut cursor = node.walk();
    let export_names: Result<Vec<_>> = node
        .children_by_field_name("id", &mut cursor)
        .map(|id_node| build_identifier(&id_node, source))
        .collect();
    let export_names = export_names?;
    Ok(Export {
        id: node_id(),
        location: location(node),
        values: export_names,
    })
}

fn build_ledger(node: &Node, source: &str) -> Result<Ledger> {
    let ledger_name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in ledger declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let is_exported = node.child_by_field_name("export").is_some();
    let is_sealed = node.child_by_field_name("sealed").is_some();
    let name = build_identifier(&ledger_name_node, source)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in ledger declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(&type_node, source)?;
    Ok(Ledger {
        id: node_id(),
        location: location(node),
        name,
        is_exported,
        is_sealed,
        ty,
    })
}

fn build_witness(node: &Node, source: &str) -> Result<Witness> {
    let witness_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in witness declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let is_exported = node.child_by_field_name("export").is_some();
    let name = build_identifier(&witness_name_node, source)?;
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(generic_node, source));

    let cursor = &mut node.walk();
    let arguments = node
        .children_by_field_name("arg", cursor)
        .map(|arg_node| build_argument(&arg_node, source))
        .collect::<Result<Vec<_>>>()?;

    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in witness declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(&type_node, source)?;
    Ok(Witness {
        id: node_id(),
        location: location(node),
        name,
        generic_parameters,
        arguments,
        ty,
        is_exported,
    })
}

fn build_circuit(node: &Node, source: &str) -> Result<Circuit> {
    let circuit_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in circuit definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(&circuit_name_node, source)?;
    let is_exported = node.child_by_field_name("export").is_some();
    let is_pure = node.child_by_field_name("pure").is_some();

    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(generic_node, source));

    let arguments = node
        .children_by_field_name("parg", &mut node.walk())
        .map(|arg_node| build_pargument(&arg_node, source))
        .collect::<Result<Vec<_>>>()?;

    let body_node = node.child_by_field_name("body").ok_or_else(|| {
        anyhow!(
            "Missing 'body' field in circuit definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;

    let body = build_block(&body_node, source)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in circuit definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(&type_node, source)?;

    Ok(Circuit {
        id: node_id(),
        location: location(node),
        name,
        is_exported,
        is_pure,
        generic_parameters,
        arguments,
        body: Some(body),
        ty,
    })
}

fn build_statement(node: &Node, source: &str) -> Result<Statement> {
    let node = if node.kind() == "stmt" {
        &node.child(0).unwrap()
    } else {
        node
    };
    let kind = node.kind();
    let statement = match kind {
        "assign_stmt" => Statement::Assign(build_assign_statement(node, source)?),
        "block" => Statement::Block(build_block(node, source)?),
        "if" => Statement::If(build_if_statement(node, source)?),
        "for" => Statement::For(build_for_statement(node, source)?),
        "return" => Statement::Return(build_return_statement(node, source)?),
        "assert" => Statement::Assert(build_assert_statement(&node.parent().unwrap(), source)?),
        "const_stmt" => Statement::Const(build_const_statement(node, source)?),
        "expression_sequence_stmt" => {
            Statement::ExpressionSequence(build_expression_sequence(node, source)?)
        }
        _ => bail!("Unhandled statement kind: {} {:}", kind, node),
    };
    Ok(statement)
}

fn build_assign_statement(node: &Node, source: &str) -> Result<Rc<Assign>> {
    let target_node = node.child_by_field_name("target").ok_or_else(|| {
        anyhow!(
            "Missing 'target' field in assign statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let target = build_expression(&target_node, source)?;
    let value_node = node.child_by_field_name("value").ok_or_else(|| {
        anyhow!(
            "Missing 'value' field in assign statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let value = build_expression(&value_node, source)?;
    let operator_node = node.child_by_field_name("operator").ok_or_else(|| {
        anyhow!(
            "Missing 'operator' field in assign statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let operator = match operator_node.utf8_text(source.as_bytes())? {
        "=" => AssignOperator::Simple,
        "+=" => AssignOperator::Add,
        "-=" => AssignOperator::Sub,
        _ => bail!(
            "Invalid assign operator: {:?}",
            operator_node.utf8_text(source.as_bytes())?
        ),
    };
    Ok(Rc::new(Assign {
        id: node_id(),
        location: location(node),
        target,
        value,
        operator,
    }))
}

fn build_const_statement(node: &Node, source: &str) -> Result<Rc<Const>> {
    let pattern_node = node.child_by_field_name("pattern").ok_or_else(|| {
        anyhow!(
            "Missing 'pattern' field in const statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let pattern = build_pattern(&pattern_node, source)?;
    let ty_node = node.child_by_field_name("type");
    let ty = if let Some(ty_n) = ty_node {
        Some(build_type(&ty_n, source)?)
    } else {
        None
    };
    let value_node = node.child_by_field_name("value").ok_or_else(|| {
        anyhow!(
            "Missing 'value' field in const statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let value = build_expression(&value_node, source)?;
    Ok(Rc::new(Const {
        id: node_id(),
        location: location(node),
        pattern,
        value,
        ty,
    }))
}

fn build_if_statement(node: &Node, source: &str) -> Result<Rc<If>> {
    let condition_node = node.child_by_field_name("condition").ok_or_else(|| {
        anyhow!(
            "Missing 'condition' field in if statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let condition = build_expression_sequence(&condition_node, source)?;
    let then_branch_node = node.child_by_field_name("then_branch").ok_or_else(|| {
        anyhow!(
            "Missing 'then' field in if statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let then_branch = build_block(&then_branch_node, source)?;
    let else_branch_node = node.child_by_field_name("else_branch");
    let else_branch = else_branch_node
        .map(|node| build_block(&node, source))
        .transpose()?;
    Ok(Rc::new(If {
        id: node_id(),
        location: location(node),
        condition: Expression::Sequence(condition),
        then_branch,
        else_branch,
    }))
}

fn build_for_statement(node: &Node, source: &str) -> Result<Rc<For>> {
    let counter_node = node.child_by_field_name("counter").ok_or_else(|| {
        anyhow!(
            "Missing 'counter' field in for statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let counter = build_identifier(&counter_node, source)?;
    let range_start_node = node.child_by_field_name("range_start");
    let range_end_node = node.child_by_field_name("range_end");
    let limit_node = node.child_by_field_name("limit");

    let limit = if let Some(limit) = limit_node {
        Some(build_expression(&limit, source)?)
    } else {
        None
    };

    let range = if let (Some(start), Some(end)) = (range_start_node, range_end_node) {
        let start = build_nat(&start, source)?;
        let end = build_nat(&end, source)?;
        Some((start, end))
    } else {
        None
    };

    let body_node = node.child_by_field_name("body").ok_or_else(|| {
        anyhow!(
            "Missing 'body' field in for statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let body = build_block(&body_node, source)?;
    Ok(Rc::new(For {
        id: node_id(),
        location: location(node),
        counter,
        limit,
        range,
        body,
    }))
}

fn build_return_statement(node: &Node, source: &str) -> Result<Rc<Return>> {
    let value_node = node.child_by_field_name("value");
    let value = if let Some(value_node) = value_node {
        Some(build_expression(&value_node, source)?)
    } else {
        None
    };
    Ok(Rc::new(Return {
        id: node_id(),
        location: location(node),
        value,
    }))
}

fn build_assert_statement(node: &Node, source: &str) -> Result<Rc<Assert>> {
    let condition_node = node.child_by_field_name("condition").ok_or_else(|| {
        anyhow!(
            "Missing 'condition' field in assert statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let condition = build_expression(&condition_node, source)?;
    let message_node = node.child_by_field_name("message");
    let message = if message_node.is_some() {
        Some(build_str(&message_node.unwrap(), source)?)
    } else {
        None
    };
    Ok(Rc::new(Assert {
        id: node_id(),
        location: location(node),
        condition,
        msg: message,
    }))
}

fn build_block(node: &Node, source: &str) -> Result<Rc<Block>> {
    let mut cursor = node.walk();
    let statements: Result<Vec<_>> = node
        .children_by_field_name("stmt", &mut cursor)
        .map(|stmt_node| build_statement(&stmt_node, source))
        .collect();
    Ok(Rc::new(Block {
        id: node_id(),
        location: location(node),
        statements: statements?,
    }))
}

#[allow(clippy::too_many_lines)]
fn build_expression(node: &Node, source: &str) -> Result<Expression> {
    let expression = match node.kind() {
        "conditional_expr" => {
            let condition = build_expression(
                &node
                    .child_by_field_name("condition")
                    .ok_or_else(|| anyhow!("Missing 'condition' field in conditional_expr"))?,
                source,
            )?;
            let then_branch = build_expression(
                &node
                    .child_by_field_name("then_branch")
                    .ok_or_else(|| anyhow!("Missing 'then_branch' field in conditional_expr"))?,
                source,
            )?;
            let else_branch = build_expression(
                &node
                    .child_by_field_name("else_branch")
                    .ok_or_else(|| anyhow!("Missing 'else_branch' field in conditional_expr"))?,
                source,
            )?;
            Expression::Conditional(Rc::new(Conditional {
                id: node_id(),
                location: location(node),
                condition,
                then_branch,
                else_branch,
            }))
        }
        "expr" => {
            // Otherwise, delegate to the next level.
            build_expression(&node.named_child(0).unwrap(), source)?
        }
        // Binary operators: we assume the node has two named children.
        "or" => {
            let left = build_expression(&node.named_child(0).unwrap(), source)?;
            let right = build_expression(&node.named_child(1).unwrap(), source)?;
            Expression::Binary(Rc::new(Binary {
                id: node_id(),
                location: location(node),
                left,
                right,
                operator: BinaryExpressionOperator::Or,
            }))
        }
        "and" => {
            let left = build_expression(&node.named_child(0).unwrap(), source)?;
            let right = build_expression(&node.named_child(1).unwrap(), source)?;
            Expression::Binary(Rc::new(Binary {
                id: node_id(),
                location: location(node),
                left,
                right,
                operator: BinaryExpressionOperator::And,
            }))
        }
        "comparison_expr" => {
            let left = build_expression(&node.child_by_field_name("left").unwrap(), source)?;
            let right = build_expression(&node.child_by_field_name("right").unwrap(), source)?;
            let operator_node = node.child_by_field_name("operator").unwrap();
            let operator = match operator_node.utf8_text(source.as_bytes())? {
                "==" => BinaryExpressionOperator::Eq,
                "!=" => BinaryExpressionOperator::Ne,
                _ => bail!(
                    "Invalid comparison operator: {:?}",
                    operator_node.utf8_text(source.as_bytes())?
                ),
            };
            Expression::Binary(Rc::new(Binary {
                id: node_id(),
                location: location(node),
                left,
                right,
                operator,
            }))
        }
        "less_than" | "less_than_or_equal" | "greater_than" | "greater_than_or_equal" => {
            let left = build_expression(&node.named_child(0).unwrap(), source)?;
            let right = build_expression(&node.named_child(1).unwrap(), source)?;
            let operator = match node.kind() {
                "less_than" => BinaryExpressionOperator::Lt,
                "less_than_or_equal" => BinaryExpressionOperator::Le,
                "greater_than" => BinaryExpressionOperator::Gt,
                "greater_than_or_equal" => BinaryExpressionOperator::Ge,
                _ => unreachable!(),
            };
            Expression::Binary(Rc::new(Binary {
                id: node_id(),
                location: location(node),
                left,
                right,
                operator,
            }))
        }
        "add" | "subtract" | "multiply" => {
            let left = build_expression(&node.named_child(0).unwrap(), source)?;
            let right = build_expression(&node.named_child(1).unwrap(), source)?;
            let operator = match node.kind() {
                "add" => BinaryExpressionOperator::Add,
                "subtract" => BinaryExpressionOperator::Sub,
                "multiply" => BinaryExpressionOperator::Mul,
                _ => unreachable!(),
            };
            Expression::Binary(Rc::new(Binary {
                id: node_id(),
                location: location(node),
                left,
                right,
                operator,
            }))
        }
        "not_expr" => {
            let expr = build_expression(&node.child_by_field_name("expr").unwrap(), source)?;
            Expression::Unary(Rc::new(Unary {
                id: node_id(),
                location: location(node),
                operator: UnaryExpressionOperator::Not,
                operand: expr,
            }))
        }
        "term" => build_term(node, source)?,
        _ => bail!("Unhandled expression kind: {}", node.kind()),
    };
    Ok(expression)
}

#[allow(clippy::too_many_lines)]
fn build_term(node: &Node, source: &str) -> Result<Expression> {
    // If the node is a wrapper (kind "term"), descend to its first child.
    let term_node = if node.kind() == "term" {
        &node.child(0).ok_or_else(|| anyhow!("Empty term node"))?
    } else {
        node
    };

    let term = match term_node.kind() {
        "lit" => build_literal(term_node, source)?,
        "default" => {
            // Grammar: seq("default", "<", $.type, ">")
            // The type is at child index 2.
            let type_node = term_node.child(2).ok_or_else(|| {
                anyhow!(
                    "Missing type in default term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let ty = build_type(&type_node, source)?;
            Expression::Default(ty)
        }
        "map" => {
            // Grammar: seq("map", "(", $.fun, ",", commaSep1($.expr), ")")
            // We expect the function at index 2 and the list of expressions starting at index 4.
            let fun_node = term_node.child(2).ok_or_else(|| {
                anyhow!(
                    "Missing function in map term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(&fun_node, source)?;
            let exprs: Vec<_> = term_node
                .children(&mut term_node.walk())
                .filter(tree_sitter::Node::is_named)
                .skip(4) // assuming index 4 onward holds expressions
                .take_while(|child| child.kind() != ")")
                .collect();
            let expressions: Result<Vec<_>> = exprs
                .into_iter()
                .map(|n| build_expression(&n, source))
                .collect();
            Expression::Map(Rc::new(Map {
                id: node_id(),
                location: location(term_node),
                function: fun,
                expressions: expressions?,
            }))
        }
        "fold" => {
            // Grammar: seq("fold", "(", $.fun, ",", field("init_value", $.expr), ",", commaSep1($.expr), ")")
            let fun_node = term_node.child(2).ok_or_else(|| {
                anyhow!(
                    "Missing function in fold term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(&fun_node, source)?;
            // Use the labeled field "init_value" if available; otherwise assume it is at index 4.
            let init_value_node = term_node
                .child_by_field_name("init_value")
                .or_else(|| term_node.child(4))
                .ok_or_else(|| {
                    anyhow!(
                        "Missing init_value in fold term: {:?}",
                        term_node.utf8_text(source.as_bytes())
                    )
                })?;
            let initial_value = build_expression(&init_value_node, source)?;
            let exprs: Vec<_> = term_node
                .children(&mut term_node.walk())
                .filter(tree_sitter::Node::is_named)
                .skip(6) // assuming expressions start at index 6
                .take_while(|child| child.kind() != ")")
                .collect();
            let expressions: Result<Vec<_>> = exprs
                .into_iter()
                .map(|n| build_expression(&n, source))
                .collect();
            Expression::Fold(Rc::new(Fold {
                id: node_id(),
                location: location(term_node),
                function: fun,
                initial_value,
                expressions: expressions?,
            }))
        }
        "disclose_term" => {
            // Grammar: seq("disclose", "(", $.expr, ")")
            let expr_node = term_node.child_by_field_name("expr").ok_or_else(|| {
                anyhow!(
                    "Missing expression in disclose term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let expr = build_expression(&expr_node, source)?;
            Expression::Disclose(Rc::new(Disclose {
                id: node_id(),
                location: location(term_node),
                expression: expr,
            }))
        }
        "id" => {
            let id = build_identifier(term_node, source)?;
            Expression::Identifier(id)
        }
        "expr_seq" => {
            let seq = build_expression_sequence(term_node, source)?;
            Expression::Sequence(seq)
        }
        "fun" => {
            let fun = build_function(term_node, source)?;
            Expression::Function(fun)
        }
        _ => bail!("Unhandled term kind: {}", term_node.kind()),
    };
    Ok(term)
}

fn build_function(node: &Node, source: &str) -> Result<Function> {
    let name_node = node.child_by_field_name("id");

    if let Some(name_n) = name_node {
        let name = build_identifier(&name_n, source)?;
        let generic_parameters_node = node.child_by_field_name("gargs");
        let mut generic_parameters = None;
        if let Some(generics_node) = generic_parameters_node {
            let cursor = &mut generics_node.walk();
            let generic_nodes: Result<Vec<_>> = generics_node
                .children_by_field_name("garg", cursor)
                .map(|type_node| build_type(&type_node.child(0).unwrap(), source))
                .collect();
            generic_parameters = Some(generic_nodes?);
        }
        Ok(Function::Named(Rc::new(NamedFunction {
            id: node_id(),
            location: location(node),
            name,
            generic_parameters,
        })))
    } else {
        let cursor = &mut node.walk();
        let pattern_nodes = node
            .children_by_field_name("pattern", cursor)
            .map(|pattern_node| build_pattern(&pattern_node, source))
            .collect::<Result<Vec<_>>>()?;
        let parg_nodes = node
            .children_by_field_name("parg", cursor)
            .map(|parg_node| build_pargument(&parg_node, source))
            .collect::<Result<Vec<_>>>()?;
        let mut arguments = Vec::new();
        for pn in pattern_nodes {
            arguments.push(FunctionArgument::Pattern(pn));
        }
        for pn in parg_nodes {
            arguments.push(FunctionArgument::PatternArgument(pn));
        }
        let return_node = node.child_by_field_name("type");
        let return_type = if let Some(return_node) = return_node {
            Some(build_type(&return_node, source)?)
        } else {
            None
        };
        let block_node = node.child_by_field_name("block");
        let block = if let Some(block_node) = block_node {
            Some(build_block(&block_node, source)?)
        } else {
            None
        };
        if block.is_some() {
            Ok(Function::Anonymous(Rc::new(AnonymousFunction {
                id: node_id(),
                location: location(node),
                arguments,
                return_type,
                body: block,
                expr_body: None,
            })))
        } else {
            let expr_node = node.child_by_field_name("expr").ok_or_else(|| {
                anyhow!(
                    "Missing 'expr' field in anonymous function: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let expr = build_expression(&expr_node, source)?;
            Ok(Function::Anonymous(Rc::new(AnonymousFunction {
                id: node_id(),
                location: location(node),
                arguments,
                return_type,
                body: None,
                expr_body: Some(expr),
            })))
        }
    }
}

fn build_literal(node: &Node, source: &str) -> Result<Expression> {
    let kind = node.kind();
    let literal = match kind {
        "true" => Expression::Literal(Literal::Bool(Rc::new(Bool {
            id: node_id(),
            location: location(node),
            value: true,
        }))),
        "false" => Expression::Literal(Literal::Bool(Rc::new(Bool {
            id: node_id(),
            location: location(node),
            value: false,
        }))),
        "nat" => {
            let nat = build_nat(node, source)?;
            Expression::Literal(Literal::Nat(nat))
        }
        "str" => {
            let str = build_str(node, source)?;
            Expression::Literal(Literal::Str(str))
        }
        _ => bail!("Unhandled literal kind: {}", kind),
    };
    Ok(literal)
}

fn build_expression_sequence(node: &Node, source: &str) -> Result<Rc<Sequence>> {
    let mut cursor = node.walk();
    let expressions: Result<Vec<_>> = node
        .children_by_field_name("expr", &mut cursor)
        .map(|expr_node| build_expression(&expr_node, source))
        .collect();
    Ok(Rc::new(Sequence {
        id: node_id(),
        location: location(node),
        expressions: expressions?,
    }))
}

#[allow(clippy::too_many_lines)]
fn build_type(node: &Node, source: &str) -> Result<Type> {
    let node = if node.kind() == "type" {
        &node.child(0).unwrap()
    } else {
        node
    };
    let kind = node.kind();
    match kind {
        "tref" => {
            let ref_node = node.child_by_field_name("id").ok_or_else(|| {
                anyhow!(
                    "Missing 'id' field in type reference: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let ref_name = build_identifier(&ref_node, source)?;
            let generic_parameters_node = node.child_by_field_name("gargs");
            let mut generic_parameters: Option<Vec<Type>> = None;
            if let Some(generics_node) = generic_parameters_node {
                let cursor = &mut generics_node.walk();
                let generic_nodes: Result<Vec<_>> = generics_node
                    .children_by_field_name("garg", cursor)
                    .map(|type_node| build_type(&type_node.child(0).unwrap(), source))
                    .collect();
                generic_parameters = Some(generic_nodes?);
            }

            Ok(Type::Ref(Rc::new(Ref {
                id: node_id(),
                location: location(node),
                name: ref_name,
                generic_parameters,
            })))
        }
        "Boolean" => Ok(Type::Bool(Rc::new(TypeBool {
            id: node_id(),
            location: location(node),
        }))),
        "Field" => Ok(Type::Field(Rc::new(TypeField {
            id: node_id(),
            location: location(node),
        }))),
        "uint_type" => {
            let cursor = &mut node.walk();
            let size_nodes = node
                .children_by_field_name("tsize", cursor)
                .map(|size_node| build_nat(&size_node, source).unwrap())
                .collect::<Vec<_>>();
            let start = size_nodes.first().cloned().ok_or_else(|| {
                anyhow!(
                    "Missing 'tsize' field in Uint type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let end = if size_nodes.len() > 1 {
                size_nodes.last().cloned()
            } else {
                None
            };
            Ok(Type::Uint(Rc::new(Uint {
                id: node_id(),
                location: location(node),
                start,
                end,
            })))
        }
        "bytes_type" => {
            let size_node = node.child_by_field_name("tsize").ok_or_else(|| {
                anyhow!(
                    "Missing 'tsize' field in Bytes type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let nat = build_nat(&size_node, source)?;
            Ok(Type::Bytes(Rc::new(Bytes {
                id: node_id(),
                location: location(node),
                size: nat,
            })))
        }
        "opaque_type" => {
            let size_node = node.child_by_field_name("str").ok_or_else(|| {
                anyhow!(
                    "Missing 'str' field in Opaque type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let str = build_str(&size_node, source)?;
            Ok(Type::Opaque(Rc::new(Opaque {
                id: node_id(),
                location: location(node),
                value: str,
            })))
        }
        "vector_type" => {
            let size_node = node.child_by_field_name("tsize").ok_or_else(|| {
                anyhow!(
                    "Missing 'tsize' field in Vector type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let size = build_nat(&size_node, source)?;
            let element_node = node.child_by_field_name("type").ok_or_else(|| {
                anyhow!(
                    "Missing 'type' field in Vector type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let element_type = build_type(&element_node, source)?;
            Ok(Type::Vector(Rc::new(Vector {
                id: node_id(),
                location: location(node),
                size,
                ty: element_type,
            })))
        }
        "[" => {
            let mut cursor = node.walk();
            let type_nodes: Result<Vec<_>> = node
                .children_by_field_name("type", &mut cursor)
                .map(|size_node| build_type(&size_node, source))
                .collect();
            let sizes = type_nodes?;
            Ok(Type::Sum(Rc::new(Sum {
                id: node_id(),
                location: location(node),
                types: sizes,
            })))
        }
        _ => bail!("Unhandled type kind: {}", kind),
    }
}

fn build_argument(node: &Node, source: &str) -> Result<Rc<Argument>> {
    let name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(&name_node, source)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(&type_node, source)?;
    Ok(Rc::new(Argument {
        id: node_id(),
        location: location(node),
        name,
        ty,
    }))
}

fn build_pargument(node: &Node, source: &str) -> Result<Rc<PatternArgument>> {
    let pattern_node = node.child_by_field_name("pattern").ok_or_else(|| {
        anyhow!(
            "Missing 'pattern' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let pattern = build_pattern(&pattern_node, source)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(&type_node, source)?;
    Ok(Rc::new(PatternArgument {
        id: node_id(),
        location: location(node),
        pattern,
        ty,
    }))
}

fn build_pattern(node: &Node, source: &str) -> Result<Pattern> {
    let node = if node.kind() == "pattern" {
        &node.child(0).unwrap()
    } else {
        node
    };
    let kind = node.kind();
    match kind {
        "id" => {
            let name = build_identifier(node, source)?;
            Ok(Pattern::Identifier(name))
        }
        "pattern_tuple" => {
            let cursor = &mut node.walk();
            let patterns: Result<Vec<_>> = node
                .children_by_field_name("pattern_tuple_elt", cursor)
                .map(|pattern_node| build_pattern(&pattern_node, source))
                .collect();
            let patterns = patterns?;
            Ok(Pattern::Tuple(Rc::new(TuplePattern {
                id: node_id(),
                location: location(node),
                patterns,
            })))
        }
        "pattern_struct" => {
            let mut cursor = node.walk();
            let field_nodes = node
                .children_by_field_name("pattern_struct_elt", &mut cursor)
                .collect::<Vec<_>>();
            let mut fields = Vec::new();
            for field_node in field_nodes {
                let name_node = field_node.child_by_field_name("id").ok_or_else(|| {
                    anyhow!(
                        "Missing 'id' field in struct pattern field: {:?}",
                        field_node.utf8_text(source.as_bytes())
                    )
                })?;
                let name = build_identifier(&name_node, source)?;
                let pattern_node = field_node.child_by_field_name("pattern").ok_or_else(|| {
                    anyhow!(
                        "Missing 'pattern' field in struct pattern field: {:?}",
                        field_node.utf8_text(source.as_bytes())
                    )
                })?;
                let pattern = build_pattern(&pattern_node, source)?;
                fields.push(Rc::new(StructPatternField {
                    id: node_id(),
                    location: location(&field_node),
                    name,
                    pattern,
                }));
            }
            Ok(Pattern::Struct(Rc::new(StructPattern {
                id: node_id(),
                location: location(node),
                fields,
            })))
        }
        _ => bail!("Unhandled pattern kind: {}", kind),
    }
}

fn build_generic_parameters(node: &Node, source: &str) -> Vec<Rc<Identifier>> {
    let mut cursor = node.walk();
    let generic_nodes: Result<Vec<_>> = node
        .children_by_field_name("gparam", &mut cursor)
        .map(|ident_node| build_identifier(&ident_node, source))
        .collect();
    generic_nodes.unwrap()
}

fn build_identifier(node: &Node, source: &str) -> Result<Rc<Identifier>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    Ok(Rc::new(Identifier {
        id: node_id(),
        location: location(node),
        name: text,
    }))
}

fn build_nat(node: &Node, source: &str) -> Result<Rc<Nat>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    let value = text
        .parse::<u64>()
        .map_err(|_| anyhow!("Invalid Nat value: {}", text))?;
    Ok(Rc::new(Nat {
        id: node_id(),
        location: location(node),
        value,
    }))
}

fn build_str(node: &Node, source: &str) -> Result<Rc<Str>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    Ok(Rc::new(Str {
        id: node_id(),
        location: location(node),
        value: text,
    }))
}

fn location(node: &Node) -> Location {
    let start = node.start_byte();
    let end = node.end_byte();
    Location { start, end }
}

fn node_id() -> u128 {
    uuid::Uuid::new_v4().as_u128()
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_pragma_directive() {
        let source = "pragma language_version 0.13.0;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.directives.len(), 1);
        let Directive::Pragma(pragma) = source_file.ast.directives.first().unwrap();
        match &pragma.version {
            VersionExpr::Version(version) => {
                assert_eq!(version.major.value, 0);
                assert_eq!(version.minor.as_ref().unwrap().value, 13);
                assert!(version.bugfix.is_some());
                assert_eq!(version.bugfix.as_ref().unwrap().value, 0);
                assert!(matches!(version.operator, VersionOperator::Eq));
            }
            VersionExpr::Or(_, _) | VersionExpr::And(_, _) => {
                panic!("Expected a simple version expression");
            }
        }

        let source = "pragma language_version >= 0.13.0;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.directives.len(), 1);
        let Directive::Pragma(pragma) = source_file.ast.directives.first().unwrap();
        match &pragma.version {
            VersionExpr::Version(version) => {
                assert_eq!(version.major.value, 0);
                assert_eq!(version.minor.as_ref().unwrap().value, 13);
                assert!(version.bugfix.is_some());
                assert_eq!(version.bugfix.as_ref().unwrap().value, 0);
                assert!(matches!(version.operator, VersionOperator::Ge));
            }
            VersionExpr::Or(_, _) | VersionExpr::And(_, _) => {
                panic!("Expected a simple version expression");
            }
        }

        let source = "pragma language_version > 0.13.0;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.directives.len(), 1);
        let Directive::Pragma(pragma) = source_file.ast.directives.first().unwrap();
        match &pragma.version {
            VersionExpr::Version(version) => {
                assert_eq!(version.major.value, 0);
                assert_eq!(version.minor.as_ref().unwrap().value, 13);
                assert!(version.bugfix.is_some());
                assert_eq!(version.bugfix.as_ref().unwrap().value, 0);
                assert!(matches!(version.operator, VersionOperator::Gt));
            }
            VersionExpr::Or(_, _) | VersionExpr::And(_, _) => {
                panic!("Expected a simple version expression");
            }
        }

        let source = "pragma language_version <= 0.13.0;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.directives.len(), 1);
        let Directive::Pragma(pragma) = source_file.ast.directives.first().unwrap();
        match &pragma.version {
            VersionExpr::Version(version) => {
                assert_eq!(version.major.value, 0);
                assert_eq!(version.minor.as_ref().unwrap().value, 13);
                assert!(version.bugfix.is_some());
                assert_eq!(version.bugfix.as_ref().unwrap().value, 0);
                assert!(matches!(version.operator, VersionOperator::Le));
            }
            VersionExpr::Or(_, _) | VersionExpr::And(_, _) => {
                panic!("Expected a simple version expression");
            }
        }

        let source = "pragma language_version < 0.13.0;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.directives.len(), 1);
        let Directive::Pragma(pragma) = source_file.ast.directives.first().unwrap();
        match &pragma.version {
            VersionExpr::Version(version) => {
                assert_eq!(version.major.value, 0);
                assert_eq!(version.minor.as_ref().unwrap().value, 13);
                assert!(version.bugfix.is_some());
                assert_eq!(version.bugfix.as_ref().unwrap().value, 0);
                assert!(matches!(version.operator, VersionOperator::Lt));
            }
            VersionExpr::Or(_, _) | VersionExpr::And(_, _) => {
                panic!("Expected a simple version expression");
            }
        }
    }

    #[test]
    fn test_import() {
        let source = "import CompactStandardLibrary;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Import(import) => {
                assert_eq!(import.name(), "CompactStandardLibrary");
            }
            _ => panic!("Expected an import declaration"),
        }
    }
    
    #[test]
    fn test_export() {
        let source = "export { CompactStandardLibrary };";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Export(export) => {
                assert_eq!(export.values.len(), 1);
                assert_eq!(export.values.first().unwrap().name, "CompactStandardLibrary");
            }
            _ => panic!("Expected an export declaration"),
        }

        let source = "export { CompactStandardLibrary, CompactStandardLibrary2 };";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Export(export) => {
                assert_eq!(export.values.len(), 2);
                assert_eq!(export.values.first().unwrap().name, "CompactStandardLibrary");
                assert_eq!(export.values.last().unwrap().name, "CompactStandardLibrary2");
            }
            _ => panic!("Expected an export declaration"),
        }
    }

    #[test]
    fn test_ledger() {
        let source = "export ledger game_state: GAME_STATE;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "game_state");
                assert!(ledger.is_exported);
                assert!(!ledger.is_sealed);
                assert!(matches!(ledger.ty, Type::Ref(_)));
                match &ledger.ty {
                    Type::Ref(rt) => {
                        assert!(rt.generic_parameters.is_none());
                        assert_eq!(rt.name.name, "GAME_STATE");
                    },
                    _ => panic!("Expected a reference type"),
                }
            }
            _ => panic!("Expected a ledger declaration"),
        }

        let source = "export ledger last_shot_result: Maybe<ShotResult>;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "last_shot_result");
                assert!(ledger.is_exported);
                assert!(!ledger.is_sealed);
                assert!(matches!(ledger.ty, Type::Ref(_)));
                match &ledger.ty {
                    Type::Ref(rt) => {
                        assert_eq!(rt.name.name, "Maybe");
                        assert!(rt.generic_parameters.is_some());
                        assert_eq!(rt.generic_parameters.as_ref().unwrap().len(), 1);
                        match rt.generic_parameters.as_ref().unwrap().first().unwrap() {
                            Type::Ref(rt) => {
                                assert!(rt.generic_parameters.is_none());
                                assert_eq!(rt.name.name, "ShotResult");
                            },
                            _ => panic!("Expected a reference type"),
                        }
                    },
                    _ => panic!("Expected a reference type"),
                }
            }
            _ => panic!("Expected a ledger declaration"),
        }

        let source = "export sealed ledger reward_coin_color: Bytes<32>;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "reward_coin_color");
                assert!(ledger.is_exported);
                assert!(ledger.is_sealed);
                assert!(matches!(ledger.ty, Type::Bytes(_)));
                match &ledger.ty {
                    Type::Bytes(bt) => {
                        assert!(matches!(bt.size.value, 32));
                    },
                    _ => panic!("Expected a bytes type"),
                }
            }
            _ => panic!("Expected a ledger declaration"),
        }

        let source = "export ledger p1_public_key: Maybe<ZswapCoinPublicKey>;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Ledger(ledger) => {
                assert_eq!(ledger.name(), "p1_public_key");
                assert!(ledger.is_exported);
                assert!(!ledger.is_sealed);
                assert!(matches!(ledger.ty, Type::Ref(_)));
                match &ledger.ty {
                    Type::Ref(rt) => {
                        assert_eq!(rt.name.name, "Maybe");
                        assert!(rt.generic_parameters.is_some());
                        assert_eq!(rt.generic_parameters.as_ref().unwrap().len(), 1);
                        match rt.generic_parameters.as_ref().unwrap().first().unwrap() {
                            Type::Ref(rt) => {
                                assert!(rt.generic_parameters.is_none());
                                assert_eq!(rt.name.name, "ZswapCoinPublicKey");
                            },
                            _ => panic!("Expected a reference type"),
                        }
                    },
                    _ => panic!("Expected a reference type"),
                }
            }
            _ => panic!("Expected a ledger declaration"),
        }
    }

    #[test]
    fn test_witness() {
        let source = "witness local_secret_key(): Bytes<32>;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Witness(witness) => {
                assert_eq!(witness.name.name, "local_secret_key");
                assert!(!witness.is_exported);
                assert!(witness.generic_parameters.is_none());
                assert_eq!(witness.arguments.len(), 0);
                assert!(matches!(witness.ty, Type::Bytes(_)));
                match &witness.ty {
                    Type::Bytes(bt) => {
                        assert!(matches!(bt.size.value, 32));
                    },
                    _ => panic!("Expected a bytes type"),
                }
            }
            _ => panic!("Expected a witness declaration"),
        }

        let source = "export witness player_ship_positions(): Ships;";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Witness(witness) => {
                assert_eq!(witness.name.name, "player_ship_positions");
                assert!(witness.is_exported);
                assert!(witness.generic_parameters.is_none());
                assert_eq!(witness.arguments.len(), 0);
                assert!(matches!(witness.ty, Type::Ref(_)));
                match &witness.ty {
                    Type::Ref(rt) => {
                        assert!(rt.generic_parameters.is_none());
                        assert_eq!(rt.name.name, "Ships");
                    },
                    _ => panic!("Expected a reference type"),
                }
            }
            _ => panic!("Expected a witness declaration"),
        }

        let source = "witness set_player_ship_state(ship_state: ShipState): [];";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Witness(witness) => {
                assert_eq!(witness.name.name, "set_player_ship_state");
                assert!(!witness.is_exported);
                assert!(witness.generic_parameters.is_none());
                assert_eq!(witness.arguments.len(), 1);
                let arg = witness.arguments.first().unwrap();
                assert_eq!(arg.name.name, "ship_state");
                assert!(matches!(arg.ty, Type::Ref(_)));
                match &arg.ty {
                    Type::Ref(rt) => {
                        assert!(rt.generic_parameters.is_none());
                        assert_eq!(rt.name.name, "ShipState");
                    },
                    _ => panic!("Expected a reference type"),
                }
                assert!(matches!(witness.ty, Type::Sum(_)));
                match &witness.ty {
                    Type::Sum(_) => {},
                    _ => panic!("Expected a sum type"),
                }
            }
            _ => panic!("Expected a witness declaration"),
        }
    }

    #[test]
    fn test_circuit() {
        let source = r#"export circuit join_p1(): [] {
  assert game_state == GAME_STATE.waiting_p1 "Attempted to join a game that is not waiting for player 1";
  assert !p1.is_some "Already in the game";
  const sk = local_secret_key();
  // we hash the secret key and the contract address to get a unique hash for the state for each game
  const secret_key = persistent_hash<Vector<2, Bytes<32>>>([sk, kernel.self().bytes]);
  const me = public_key(sk);
  p1 = disclose(some<Bytes<32>>(me));

  const ship_positions = player_ship_positions();
  const cells = occupied_cells(ship_positions);
  assert_valid_ship_position(ship_positions, cells);

  assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s11), cells);
  assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s12), cells);
  assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s13), cells);
  assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s14), cells);
  assert_no_adjacent_neighbour_for_2ship(neighbour2_cells(ship_positions.s21, ship_positions.v21), cells);
  assert_no_adjacent_neighbour_for_2ship(neighbour2_cells(ship_positions.s22, ship_positions.v22), cells);
  assert_no_adjacent_neighbour_for_2ship(neighbour2_cells(ship_positions.s23, ship_positions.v23), cells);
  assert_no_adjacent_neighbour_for_3ship(neighbour3_cells(ship_positions.s31, ship_positions.v31), cells);
  assert_no_adjacent_neighbour_for_3ship(neighbour3_cells(ship_positions.s32, ship_positions.v32), cells);
  assert_no_adjacent_neighbour_for_4ship(neighbour4_cells(ship_positions.s41, ship_positions.v41), cells);

  const ship_state = create_ship_state(ship_positions);
  p1_ship_positions_hash = persistent_commit<Ships>(ship_positions, secret_key);
  p1_ship_state_hash = update_ship_state(ship_state, secret_key);

  game_state = GAME_STATE.waiting_p2;
}"#;
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.definitions.len(), 1);
        match &source_file.ast.definitions.first().unwrap() {
            Definition::Circuit(circuit) => {
                assert_eq!(circuit.name.name, "join_p1");
                assert!(circuit.is_exported);
                assert_eq!(circuit.arguments.len(), 0);
                assert!(circuit.generic_parameters.is_none());
                assert!(matches!(circuit.ty, Type::Sum(_)));
                match &circuit.ty {
                    Type::Sum(_) => {},
                    _ => panic!("Expected a sum type"),
                }
                assert!(circuit.body.is_some());
                let body = circuit.body.as_ref().unwrap();
                assert_eq!(body.statements.len(), 23);
            }
            _ => panic!("Expected a circuit declaration"),
        }
    }

    #[test]
    fn test_statements_1() {
        // assert game_state == GAME_STATE.waiting_p1 "Attempted to join a game that is not waiting for player 1";
        // assert !p1.is_some "Already in the game";
        // const sk = local_secret_key();
        // // we hash the secret key and the contract address to get a unique hash for the state for each game
        // const secret_key = persistent_hash<Vector<2, Bytes<32>>>([sk, kernel.self().bytes]);
        // const me = public_key(sk);
        // p1 = disclose(some<Bytes<32>>(me));
      
        // const ship_positions = player_ship_positions();
        // const cells = occupied_cells(ship_positions);
        // assert_valid_ship_position(ship_positions, cells);
      
        // assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s11), cells);
        // assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s12), cells);
        // assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s13), cells);
        // assert_neighbour_is_not_1ship(neighbour1_cells(ship_positions.s14), cells);
        // assert_no_adjacent_neighbour_for_2ship(neighbour2_cells(ship_positions.s21, ship_positions.v21), cells);
        // assert_no_adjacent_neighbour_for_2ship(neighbour2_cells(ship_positions.s22, ship_positions.v22), cells);
        // assert_no_adjacent_neighbour_for_2ship(neighbour2_cells(ship_positions.s23, ship_positions.v23), cells);
        // assert_no_adjacent_neighbour_for_3ship(neighbour3_cells(ship_positions.s31, ship_positions.v31), cells);
        // assert_no_adjacent_neighbour_for_3ship(neighbour3_cells(ship_positions.s32, ship_positions.v32), cells);
        // assert_no_adjacent_neighbour_for_4ship(neighbour4_cells(ship_positions.s41, ship_positions.v41), cells);
      
        // const ship_state = create_ship_state(ship_positions);
        // p1_ship_positions_hash = persistent_commit<Ships>(ship_positions, secret_key);
        // p1_ship_state_hash = update_ship_state(ship_state, secret_key);
      
        // game_state = GAME_STATE.waiting_p2;
        let source = "circuit join_p1(): [] { assert game_state == GAME_STATE.waiting_p1 \"Attempted to join a game that is not waiting for player 1\";}";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.definitions.len(), 1);
        match &source_file.ast.definitions.first().unwrap() {
            Definition::Circuit(circuit) => {
                assert_eq!(circuit.name.name, "join_p1");
                assert!(!circuit.is_exported);
                assert_eq!(circuit.arguments.len(), 0);
                assert!(circuit.generic_parameters.is_none());
                assert!(matches!(circuit.ty, Type::Sum(_)));
                match &circuit.ty {
                    Type::Sum(_) => {},
                    _ => panic!("Expected a sum type"),
                }
                assert!(circuit.body.is_some());
                let body = circuit.body.as_ref().unwrap();
                assert_eq!(body.statements.len(), 1);
                let statement = body.statements.first().unwrap();
                assert!(matches!(statement, Statement::Assert(_)));
                let Statement::Assert(assert_statement) = &statement else {
                  panic!("Expected an assert statement");  
                };
                assert!(matches!(assert_statement.condition, Expression::Binary(_)));
                let Expression::Binary(binary_expr) = &assert_statement.condition else {
                    panic!("Expected a binary expression");
                };
                assert!(matches!(binary_expr.left, Expression::Identifier(_)));
                assert!(matches!(binary_expr.right, Expression::Identifier(_)));
                assert!(matches!(binary_expr.operator, BinaryExpressionOperator::Eq));
                assert!(assert_statement.msg.is_some());
                assert_eq!(assert_statement.msg.as_ref().unwrap().value, "\"Attempted to join a game that is not waiting for player 1\"");
            }
            _ => panic!("Expected a circuit declaration"),
        }

    }

}
