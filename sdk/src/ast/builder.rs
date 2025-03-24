#![warn(clippy::pedantic)]
use anyhow::{anyhow, bail, Ok, Result};
use std::rc::Rc;
use tree_sitter::Node;

use crate::ast::{directive::VersionExpr, literal::VersionOperator};

use super::{
    declaration::{
        Argument, Constructor, Contract, Declaration, Export, GArgument, Import, Ledger, Pattern,
        PatternArgument, StructPattern, StructPatternField, TuplePattern, Witness,
    },
    definition::{Circuit, Definition, Enum, Module, Structure},
    directive::{Directive, Pragma},
    expression::{
        Binary, BinaryExpressionOperator, Conditional, Disclose, Expression, Fold, FunctionCall,
        Identifier, Map, MemberAccess, Sequence, StructExpr, StructExprArg, StructNamedField,
        Unary, UnaryExpressionOperator,
    },
    function::{AnonymousFunction, Function, FunctionArgument, NamedFunction},
    literal::{Array, Bool, Literal, Nat, Pad, Str, Version},
    node::Location,
    program::{CompactNode, Program},
    statement::{Assert, Assign, AssignOperator, Block, Const, For, If, Return, Statement},
    ty::{self, Bytes, Opaque, Ref, Sum, Type, TypeBool, TypeField, Uint, Vector, VectorSize},
};

/// Builds an AST from the given root node and source code.
///
/// # Errors
/// This function will return an error if the root node kind is not `source_file` or if any child node cannot be processed.
///
/// # Panics
/// This function will panic if `root.named_child(i).unwrap()` fails.
pub fn build_ast(root: &Node, source: &str) -> Result<Rc<Program>> {
    if root.kind() != "source_file" {
        bail!("Invalid root node kind: {}", root.kind());
    }
    let mut directives: Vec<Directive> = Vec::new();
    let mut declarations: Vec<Declaration> = Vec::new();
    let mut definitions: Vec<Definition> = Vec::new();
    let mut modules: Vec<Rc<Module>> = Vec::new();

    for i in 0..root.named_child_count() {
        let child = root.named_child(i).unwrap();
        match build_compact_node(&child, source)? {
            CompactNode::Directive(d) => directives.push(d),
            CompactNode::Declaration(d) => declarations.push(d),
            CompactNode::Definition(d) => definitions.push(d),
            CompactNode::Module(m) => modules.push(m),
            CompactNode::Comment(_) => {}
        }
    }
    Ok(Rc::new(Program {
        id: node_id(),
        location: location(root),
        directives,
        declarations,
        definitions,
        modules,
    }))
}

fn build_compact_node(node: &Node, source: &str) -> Result<CompactNode> {
    match node.kind() {
        "pragma" => {
            let pragma = build_pragma(node, source)?;
            Ok(CompactNode::Directive(Directive::Pragma(Rc::new(pragma))))
        }
        // import declaration
        "idecl" => {
            let import_decl = build_import(node, source)?;
            Ok(CompactNode::Declaration(Declaration::Import(Rc::new(
                import_decl,
            ))))
        }
        // export declaration
        "xdecl" => {
            let export_decl = build_export(node, source)?;
            Ok(CompactNode::Declaration(Declaration::Export(Rc::new(
                export_decl,
            ))))
        }
        // ledger declaration
        "ldecl" => {
            let ledger_decl = build_ledger(node, source)?;
            Ok(CompactNode::Declaration(Declaration::Ledger(Rc::new(
                ledger_decl,
            ))))
        }
        // // witness declaration
        "wdecl" => {
            let witness_decl = build_witness(node, source)?;
            Ok(CompactNode::Declaration(Declaration::Witness(Rc::new(
                witness_decl,
            ))))
        }
        // circuit definition
        "cdefn" => {
            let circuit = build_circuit(node, source)?;
            Ok(CompactNode::Definition(Definition::Circuit(Rc::new(
                circuit,
            ))))
        }
        //
        "edecl" => {
            let circuit = build_external_circuit(node, source)?;
            Ok(CompactNode::Definition(Definition::Circuit(Rc::new(
                circuit,
            ))))
        }
        // struct definition
        "struct" => {
            let structure = build_structure(node, source)?;
            Ok(CompactNode::Definition(Definition::Structure(Rc::new(
                structure,
            ))))
        }
        // enum definition
        "enumdef" => {
            let enum_def = build_enum(node, source)?;
            Ok(CompactNode::Definition(Definition::Enum(Rc::new(enum_def))))
        }
        // external contract
        "ecdecl" => {
            let contract = build_external_contract(node, source)?;
            Ok(CompactNode::Declaration(Declaration::Contract(Rc::new(
                contract,
            ))))
        }
        // constructor definition
        "lconstructor" => {
            let constructor = build_constructor(node, source)?;
            Ok(CompactNode::Declaration(Declaration::Constructor(Rc::new(
                constructor,
            ))))
        }
        // module definition
        "mdefn" => {
            let module = build_module(node, source)?;
            Ok(CompactNode::Module(Rc::new(module)))
        }
        "comment" => {
            let comment = node.utf8_text(source.as_bytes())?;
            Ok(CompactNode::Comment(comment.to_string()))
        }
        other => bail!("Unhandled node kind: {}", other),
    }
}

fn build_module(node: &Node, source: &str) -> Result<Module> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in module definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(&name_node, source)?;
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(generic_node, source));
    let cursor = &mut node.walk();
    let module_element_nodes = node.children_by_field_name("module_element", cursor);
    let mut nodes = Vec::new();
    for child in module_element_nodes {
        let compact_node = build_compact_node(&child, source)?;
        nodes.push(compact_node);
    }
    Ok(Module {
        id: node_id(),
        location: location(node),
        is_exported,
        name,
        generic_parameters,
        nodes,
    })
}

fn build_enum(node: &Node, source: &str) -> Result<Enum> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in enum definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(&name_node, source)?;
    let ids = node
        .children_by_field_name("id", &mut node.walk())
        .map(|id_node| build_identifier(&id_node, source))
        .collect::<Result<Vec<_>>>()?;

    Ok(Enum {
        id: node_id(),
        location: location(node),
        is_exported,
        name,
        options: ids,
    })
}

fn build_pragma(node: &Node, source: &str) -> Result<Pragma> {
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
            "Missing 'id' field in witness declaration: {:?}",
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

fn build_external_circuit(node: &Node, source: &str) -> Result<Circuit> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name = build_identifier(
        &node
            .child_by_field_name("id")
            .ok_or_else(|| anyhow!("Missing 'id' field in external circuit"))?,
        source,
    )?;
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(generic_node, source));
    let cursor = &mut node.walk();

    let arguments = node
        .children_by_field_name("arg", cursor)
        .map(|arg_node| {
            let arg = build_argument(&arg_node, source)?;
            Ok(Rc::new(PatternArgument {
                id: node_id(),
                location: location(&arg_node),
                pattern: Pattern::Identifier(arg.name.clone()),
                ty: arg.ty.clone(),
            }))
        })
        .collect::<Result<Vec<_>>>()?;

    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in external circuit: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(&type_node, source)?;

    Ok(Circuit {
        id: node_id(),
        location: location(node),
        is_exported,
        is_pure: false,
        arguments,
        name,
        generic_parameters,
        body: None,
        ty,
    })
}

fn build_constructor(node: &Node, source: &str) -> Result<Constructor> {
    let arguments = node
        .children_by_field_name("parg", &mut node.walk())
        .map(|arg_node| build_pargument(&arg_node, source))
        .collect::<Result<Vec<_>>>()?;

    let body_node = node.child_by_field_name("body").ok_or_else(|| {
        anyhow!(
            "Missing 'body' field in constructor declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;

    let body = build_block(&body_node, source)?;

    Ok(Constructor {
        id: node_id(),
        location: location(node),
        arguments,
        body,
    })
}

fn build_external_contract(node: &Node, source: &str) -> Result<Contract> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name = build_identifier(
        &node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("Missing 'id' field in external contract"))?,
        source,
    )?;
    let cursor = &mut node.walk();
    let contract_circuit_nodes = node.children_by_field_name("contract_circuit", cursor);
    let mut circuits = Vec::new();
    for circuit_node in contract_circuit_nodes {
        let is_pure = circuit_node.child_by_field_name("pure").is_some();
        let id_node = circuit_node
            .child_by_field_name("id")
            .ok_or_else(|| anyhow!("Missing 'id' field in contract circuit"))?;
        let name = build_identifier(&id_node, source)?;
        let cursor = &mut circuit_node.walk();
        let arguments = circuit_node
            .children_by_field_name("arg", cursor)
            .map(|arg_node| {
                let arg = build_argument(&arg_node, source)?;
                Ok(Rc::new(PatternArgument {
                    id: node_id(),
                    location: location(&arg_node),
                    pattern: Pattern::Identifier(arg.name.clone()),
                    ty: arg.ty.clone(),
                }))
            })
            .collect::<Result<Vec<_>>>()?;
        let ty_node = circuit_node.child_by_field_name("type").ok_or_else(|| {
            anyhow!(
                "Missing 'type' field in contract circuit: {:?}",
                circuit_node.utf8_text(source.as_bytes())
            )
        })?;
        let ty = build_type(&ty_node, source)?;
        circuits.push(Rc::new(Circuit {
            id: node_id(),
            location: location(&circuit_node),
            name,
            arguments,
            generic_parameters: None,
            is_exported: false,
            is_pure,
            ty,
            body: None,
        }));
    }
    Ok(Contract {
        id: node_id(),
        location: location(node),
        is_exported,
        name,
        circuits,
    })
}

fn build_structure(node: &Node, source: &str) -> Result<Structure> {
    let structure_name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in structure definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(&structure_name_node, source)?;
    let is_exported = node.child_by_field_name("export").is_some();
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(generic_node, source));

    let fields = node
        .children_by_field_name("arg", &mut node.walk())
        .map(|field_node| build_argument(&field_node, source))
        .collect::<Result<Vec<_>>>()?;

    Ok(Structure {
        id: node_id(),
        location: location(node),
        name,
        is_exported,
        generic_parameters,
        fields,
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
        "if_stmt" => Statement::If(build_if_statement(node, source)?),
        "for_stmt" => Statement::For(build_for_statement(node, source)?),
        "return_stmt" => Statement::Return(build_return_statement(node, source)?),
        "assert_stmt" => Statement::Assert(build_assert_statement(node, source)?),
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
    let pattern = build_pattern(&pattern_node.child(0).unwrap(), source)?;
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
    let then_branch = build_statement(&then_branch_node, source)?;
    let else_branch_node = node.child_by_field_name("else_branch");
    let else_branch = else_branch_node
        .map(|node| build_statement(&node, source))
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
    let condition_node = node
        .child_by_field_name("condition")
        .ok_or_else(|| anyhow!("Missing 'condition' field in assert statement: {:?}", node))?;
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
        "bin_sum_expr" | "bin_mul_expr" => {
            let left = build_expression(&node.child_by_field_name("left").unwrap(), source)?;
            let right = build_expression(&node.child_by_field_name("right").unwrap(), source)?;
            let operator = match node
                .child_by_field_name("operator")
                .unwrap()
                .utf8_text(source.as_bytes())?
            {
                "+" => BinaryExpressionOperator::Add,
                "-" => BinaryExpressionOperator::Sub,
                "*" => BinaryExpressionOperator::Mul,
                _ => bail!("Invalid binary operator"),
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
        "member_access_expr" => {
            let base = build_expression(&node.child_by_field_name("base").unwrap(), source)?;
            let member = build_identifier(&node.child_by_field_name("member").unwrap(), source)?;
            let arguments_node = node.child_by_field_name("arguments");
            let arguments = if let Some(arguments_node) = arguments_node {
                let arguments: Result<Vec<_>> = arguments_node
                    .children_by_field_name("expr", &mut arguments_node.walk())
                    .map(|arg_node| build_expression(&arg_node, source))
                    .collect();
                Some(arguments?)
            } else {
                None
            };
            Expression::MemberAccess(Rc::new(MemberAccess {
                id: node_id(),
                location: location(node),
                base,
                member,
                arguments,
            }))
        }
        "expr_seq" => {
            let seq = build_expression_sequence(node, source)?;
            Expression::Sequence(seq)
        }
        "term" => build_term(node, source)?,
        _ => bail!("Unhandled expression kind: {}", node.kind()),
    };
    Ok(expression)
}

#[allow(clippy::too_many_lines)]
fn build_term(node: &Node, source: &str) -> Result<Expression> {
    let term_node = if node.kind() == "term" {
        &node.child(0).ok_or_else(|| anyhow!("Empty term node"))?
    } else {
        node
    };

    let term = match term_node.kind() {
        "lit" => build_literal(&term_node.child(0).unwrap(), source)?,
        "default_term" => {
            let type_node = term_node.child_by_field_name("type").ok_or_else(|| {
                anyhow!(
                    "Missing 'type' field in default term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let ty = build_type(&type_node, source)?;
            Expression::Default(ty)
        }
        "map_term" => {
            let fun_node = term_node.child_by_field_name("fun").ok_or_else(|| {
                anyhow!(
                    "Missing function in map term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(&fun_node, source)?;
            let expr_nodes = term_node
                .children_by_field_name("expr", &mut term_node.walk())
                .collect::<Vec<_>>();
            let expressions: Result<Vec<_>> = expr_nodes
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
        "fold_term" => {
            let fun_node = term_node.child_by_field_name("fun").ok_or_else(|| {
                anyhow!(
                    "Missing function in fold term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(&fun_node, source)?;
            let init_value_node = term_node
                .child_by_field_name("init_value")
                .unwrap_or_else(|| term_node.child(4).unwrap());
            let initial_value = build_expression(&init_value_node, source)?;
            let exprs: Vec<_> = term_node
                .children_by_field_name("expr", &mut term_node.walk())
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
        "expr_seq_term" => {
            let seq = build_expression_sequence(term_node, source)?;
            Expression::Sequence(seq)
        }
        "function_call_term" => {
            let fun_node = term_node.child_by_field_name("fun").ok_or_else(|| {
                anyhow!(
                    "Missing 'fun' field in function_call_term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(&fun_node, source)?;
            let expr_nodes = term_node
                .children_by_field_name("expr", &mut term_node.walk())
                .collect::<Vec<_>>();
            let arguments = expr_nodes
                .into_iter()
                .map(|expr_node| build_expression(&expr_node, source))
                .collect::<Result<Vec<_>>>()?;
            Expression::FunctionCall(Rc::new(FunctionCall {
                id: node_id(),
                location: location(term_node),
                function: Expression::Function(fun),
                arguments,
            }))
        }
        "struct_term" => {
            let struct_expr = build_struct_expression(term_node, source)?;
            Expression::Struct(struct_expr)
        }
        "[" => {
            let expr_nodes = term_node
                .parent()
                .unwrap()
                .children_by_field_name("expr", &mut term_node.walk())
                .collect::<Vec<_>>();
            let elements = expr_nodes
                .into_iter()
                .map(|expr_node| build_expression(&expr_node, source))
                .collect::<Result<Vec<_>>>()?;
            Expression::Literal(Literal::Array(Rc::new(Array {
                id: node_id(),
                location: location(term_node),
                elements,
            })))
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
                .map(|type_node| build_gargument(&type_node.child(0).unwrap(), source))
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
        "pad" => {
            let nat_node = node.child_by_field_name("nat").ok_or_else(|| {
                anyhow!(
                    "Missing 'nat' field in pad literal: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let nat = build_nat(&nat_node, source)?;
            let str_node = node.child_by_field_name("str").ok_or_else(|| {
                anyhow!(
                    "Missing 'str' field in pad literal: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let str = build_str(&str_node, source)?;
            Expression::Literal(Literal::Pad(Rc::new(Pad {
                id: node_id(),
                location: location(node),
                number: nat,
                name: str,
            })))
        }
        _ => bail!("Unhandled literal kind: {:?}", node),
    };
    Ok(literal)
}

fn build_struct_expression(node: &Node, source: &str) -> Result<Rc<StructExpr>> {
    let tref_node = node
        .child_by_field_name("tref")
        .ok_or_else(|| anyhow!("Missing 'tref' field in struct expression: {:?}", node))?;
    let tref = build_type(&tref_node, source)?;
    let cursor = &mut node.walk();
    let mut struct_args = Vec::new();
    for child in node.children(cursor) {
        if child.kind() == "struct_arg" {
            let struct_arg_node = child.named_child(0).unwrap();
            match struct_arg_node.kind() {
                "expr" => {
                    let expr = build_expression(&struct_arg_node, source)?;
                    struct_args.push(StructExprArg::Expression(expr));
                }
                "struct_named_filed_initializer" => {
                    let id_node = struct_arg_node.child_by_field_name("id").ok_or_else(|| {
                        anyhow!(
                            "Missing 'id' field in struct_named_filed_initializer {:?}",
                            struct_arg_node
                        )
                    })?;
                    let expr_node =
                        struct_arg_node.child_by_field_name("expr").ok_or_else(|| {
                            anyhow!("Missing 'expr' field in struct_named_filed_initializer")
                        })?;
                    let name = build_identifier(&id_node, source)?;
                    let expr = build_expression(&expr_node, source)?;
                    struct_args.push(StructExprArg::NamedField(Rc::new(StructNamedField {
                        id: node_id(),
                        location: location(&struct_arg_node),
                        name,
                        value: expr,
                    })));
                }
                "struct_update_field" => {
                    let expr_node = struct_arg_node
                        .child_by_field_name("expr")
                        .ok_or_else(|| anyhow!("Missing 'expr' field in struct_update_field"))?;
                    let expr = build_expression(&expr_node, source)?;
                    struct_args.push(StructExprArg::Update(expr));
                }
                _ => bail!("Unhandled struct_arg node: {}", struct_arg_node.kind()),
            }
        }
    }

    Ok(Rc::new(StructExpr {
        id: node_id(),
        location: location(node),
        ty: tref,
        args: struct_args,
    }))
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
            let mut generic_parameters: Option<Vec<GArgument>> = None;
            if let Some(generics_node) = generic_parameters_node {
                let cursor = &mut generics_node.walk();
                let generic_nodes: Result<Vec<_>> = generics_node
                    .children_by_field_name("garg", cursor)
                    .map(|type_node| build_gargument(&type_node.child(0).unwrap(), source))
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
        "Boolean" => Ok(Type::Boolean(Rc::new(TypeBool {
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
            let size = match size_node.child(0).unwrap().kind() {
                "nat" => VectorSize::Nat(build_nat(&size_node, source)?),
                "id" => VectorSize::Ref(build_identifier(&size_node, source)?),
                _ => bail!("Invalid size kind: {:?}", size_node.kind()),
            };
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

fn build_gargument(node: &Node, source: &str) -> Result<GArgument> {
    match node.kind() {
        "nat" => {
            let nat = build_nat(node, source)?;
            Ok(GArgument::Nat(nat))
        }
        "type" => {
            let ty = build_type(node, source)?;
            Ok(GArgument::Type(ty))
        }
        _ => bail!(
            "Unhandled generic argument kind: {:?}",
            node.child(0).unwrap().kind()
        ),
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
    let pattern = build_pattern(&pattern_node.child(0).unwrap(), source)?;
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
        "[" => {
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
        "{" => {
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
        _ => bail!("Unhandled pattern kind: {:?}", kind),
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
    static mut CURR_ID: u128 = 0;
    unsafe {
        CURR_ID += 1;
        CURR_ID
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::parse_content;

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

    #[test]#[allow(clippy::too_many_lines)]
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
                            GArgument::Type(ty) => {
                                assert!(matches!(ty, Type::Ref(_)));
                                match ty {
                                    Type::Ref(rt) => {
                                        assert!(rt.generic_parameters.is_none());
                                        assert_eq!(rt.name.name, "ShotResult");
                                    },
                                    _ => panic!("Expected a reference type"),
                                }
                            },
                            GArgument::Nat(_) => panic!("Expected a type argument"),
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
                            GArgument::Type(ty) => {
                                assert!(matches!(ty, Type::Ref(_)));
                                match ty {
                                    Type::Ref(rt) => {
                                        assert!(rt.generic_parameters.is_none());
                                        assert_eq!(rt.name.name, "ZswapCoinPublicKey");
                                    },
                                    _ => panic!("Expected a reference type"),
                                }
                            },
                            GArgument::Nat(_) => panic!("Expected a type argument"),
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
                assert!(matches!(binary_expr.right, Expression::MemberAccess(_)));
                assert!(matches!(binary_expr.operator, BinaryExpressionOperator::Eq));
                let Expression::Identifier(identifier) = &binary_expr.left else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(identifier.name, "game_state");
                let Expression::MemberAccess(member_access_expr) = &binary_expr.right else {
                    panic!("Expected a member access expression");
                };
                assert_eq!(member_access_expr.member.name, "waiting_p1");
                assert!(matches!(member_access_expr.base, Expression::Identifier(_)));
                let Expression::Identifier(identifier) = &member_access_expr.base else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(identifier.name, "GAME_STATE");
                assert!(assert_statement.msg.is_some());
                assert_eq!(assert_statement.msg.as_ref().unwrap().value, "\"Attempted to join a game that is not waiting for player 1\"");
            }
            _ => panic!("Expected a circuit declaration"),
        }
        
    }

    #[test]
    fn test_statements_2() {
        let source = "circuit join_p1(): [] { assert !p1.is_some \"Already in the game\";}";
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
                assert!(matches!(assert_statement.condition, Expression::Unary(_)));
                let Expression::Unary(unary_expr) = &assert_statement.condition else {
                    panic!("Expected a unary expression");
                };
                assert!(matches!(unary_expr.operator, UnaryExpressionOperator::Not));
                assert!(matches!(unary_expr.operand, Expression::MemberAccess(_)));
                let Expression::MemberAccess(member_access_expr) = &unary_expr.operand else {
                    panic!("Expected a member access expression");
                };
                assert_eq!(member_access_expr.member.name, "is_some");
                assert!(matches!(member_access_expr.base, Expression::Identifier(_)));
                let Expression::Identifier(identifier) = &member_access_expr.base else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(identifier.name, "p1");
                assert!(assert_statement.msg.is_some());
                assert_eq!(assert_statement.msg.as_ref().unwrap().value, "\"Already in the game\"");
            }
            _ => panic!("Expected a circuit declaration"),
        }
    }

    #[test]#[allow(clippy::too_many_lines)]
    fn test_const_statements() {
        let source = r"circuit join_p1(): [] {
            const secret_key = persistent_hash<Vector<2, Bytes<32>>>([sk, kernel.self().bytes]);
            const ship_positions = player_ship_positions();
            const cells = occupied_cells(ship_positions);
        }";
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
                assert_eq!(body.statements.len(), 3);
                let statement = body.statements.first().unwrap();
                assert!(matches!(statement, Statement::Const(_)));
                let Statement::Const(const_statement) = &statement else {
                    panic!("Expected a const statement");  
                };
                assert!(matches!(const_statement.pattern, Pattern::Identifier(_)));
                let Pattern::Identifier(identifier) = &const_statement.pattern else {
                    panic!("Expected an identifier pattern");
                };
                assert_eq!(identifier.name, "secret_key");
                assert!(const_statement.ty.is_none());

                assert!(matches!(const_statement.value, Expression::FunctionCall(_)));
                let Expression::FunctionCall(call_expr) = &const_statement.value else {
                    panic!("Expected a function call expression");
                };
                assert!(matches!(call_expr.function, Expression::Function(_)));
                let Expression::Function(function_expr) = &call_expr.function else {
                    panic!("Expected a function expression");
                };
                assert!(matches!(function_expr, Function::Named(_)));
                let Function::Named(named_function) = function_expr else {
                    panic!("Expected a named function");
                };
                assert_eq!(named_function.name.name, "persistent_hash");
                assert!(named_function.generic_parameters.is_some());
                assert_eq!(named_function.generic_parameters.as_ref().unwrap().len(), 1);
                match named_function.generic_parameters.as_ref().unwrap().first().unwrap() {
                    GArgument::Type(vector) => {
                        assert!(matches!(vector, Type::Vector(_)));
                        match vector {
                            Type::Vector(vector) => {
                                assert!(matches!(vector.size, VectorSize::Nat(_)));
                                match &vector.size {
                                    VectorSize::Nat(nat) => {
                                        assert_eq!(nat.value, 2);
                                    },
                                    VectorSize::Ref(_) => panic!("Expected a nat argument"),
                                }
                                assert!(matches!(vector.ty, Type::Bytes(_)));
                                match &vector.ty {
                                    Type::Bytes(bt) => {
                                        assert!(matches!(bt.size.value, 32));
                                    },
                                    _ => panic!("Expected a bytes type"),
                                }
                            },
                            _ => panic!("Expected a vector type"),
                        }
                    },
                    GArgument::Nat(_) => panic!("Expected a nat argument"),
                }
                assert_eq!(call_expr.arguments.len(), 1);
                let arg = call_expr.arguments.first().unwrap();
                assert!(matches!(arg, Expression::Literal(Literal::Array(_))));
                let Expression::Literal(Literal::Array(array_expr)) = arg else {
                    panic!("Expected an array expression");
                };
                assert_eq!(array_expr.elements.len(), 2);
                let element = array_expr.elements.last().unwrap();
                assert!(matches!(element, Expression::MemberAccess(_)));
                let Expression::MemberAccess(member_access) = element else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(member_access.member.name, "bytes");
                assert!(matches!(member_access.base, Expression::MemberAccess(_)));
                let Expression::MemberAccess(member_access) = &member_access.base else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(member_access.member.name, "self");
                assert!(matches!(member_access.base, Expression::Identifier(_)));
                let Expression::Identifier(identifier) = &member_access.base else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(identifier.name, "kernel");
                println!("{:?}", member_access.arguments);
                assert!(member_access.arguments.is_some());
                assert_eq!(member_access.arguments.as_ref().unwrap().len(), 0);
            }
            _ => panic!("Expected a circuit declaration"),
        }
    }

    #[test]
    fn test_structure() {
        let source = "export struct ShotResult {
            cell: Coord;
            result: SHOT_RESULT;
            player: Bytes<32>;
            ship_def: ShipDef;
        }";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.definitions.len(), 1);
        match &source_file.ast.definitions.first().unwrap() {
            Definition::Structure(struct_decl) => {
                assert_eq!(struct_decl.name.name, "ShotResult");
                assert!(struct_decl.is_exported);
                assert_eq!(struct_decl.fields.len(), 4);
                let field = struct_decl.fields.first().unwrap();
                assert_eq!(field.name.name, "cell");
                assert!(matches!(field.ty, Type::Ref(_)));
                match &field.ty {
                    Type::Ref(rt) => {
                        assert!(rt.generic_parameters.is_none());
                        assert_eq!(rt.name.name, "Coord");
                    },
                    _ => panic!("Expected a reference type"),
                }
                let field = struct_decl.fields.get(1).unwrap();
                assert_eq!(field.name.name, "result");
                assert!(matches!(field.ty, Type::Ref(_)));
                match &field.ty {
                    Type::Ref(rt) => {
                        assert!(rt.generic_parameters.is_none());
                        assert_eq!(rt.name.name, "SHOT_RESULT");
                    },
                    _ => panic!("Expected a reference type"),
                }
                let field = struct_decl.fields.get(2).unwrap();
                assert_eq!(field.name.name, "player");
                assert!(matches!(field.ty, Type::Bytes(_)));
                match &field.ty {
                    Type::Bytes(bt) => {
                        assert!(matches!(bt.size.value, 32));
                    },
                    _ => panic!("Expected a bytes type"),
                }
                let field = struct_decl.fields.get(3).unwrap();
                assert_eq!(field.name.name, "ship_def");
                assert!(matches!(field.ty, Type::Ref(_)));
                match &field.ty {
                    Type::Ref(rt) => {
                        assert!(rt.generic_parameters.is_none());
                        assert_eq!(rt.name.name, "ShipDef");
                    },
                    _ => panic!("Expected a reference type"),
                }
            }
            _ => panic!("Expected a struct declaration"),
        }
    }

    #[test]
    fn test_constructor() {
        let source = "constructor(initNonce: Bytes<32>) { nonce = initNonce; }";
        let source_file = parse_content("dummy", source).unwrap();
        assert_eq!(source_file.ast.declarations.len(), 1);
        match &source_file.ast.declarations.first().unwrap() {
            Declaration::Constructor(constructor) => {
                assert_eq!(constructor.arguments.len(), 1);
                let arg = constructor.arguments.first().unwrap();
                assert!(matches!(arg.pattern, Pattern::Identifier(_)));
                let Pattern::Identifier(identifier) = &arg.pattern else {
                    panic!("Expected an identifier pattern");
                };
                assert_eq!(identifier.name, "initNonce");
                assert!(matches!(arg.ty, Type::Bytes(_)));
                match &arg.ty {
                    Type::Bytes(bt) => {
                        assert!(matches!(bt.size.value, 32));
                    },
                    _ => panic!("Expected a bytes type"),
                }
                let body = &constructor.body;
                assert_eq!(body.statements.len(), 1);
                let statement = body.statements.first().unwrap();
                assert!(matches!(statement, Statement::Assign(_)));
                let Statement::Assign(assignment) = &statement else {
                    panic!("Expected an assignment statement");  
                };
                assert!(matches!(assignment.target, Expression::Identifier(_)));
                let Expression::Identifier(identifier) = &assignment.target else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(identifier.name, "nonce");
                assert!(matches!(assignment.value, Expression::Identifier(_)));
                let Expression::Identifier(identifier) = &assignment.value else {
                    panic!("Expected an identifier expression");
                };
                assert_eq!(identifier.name, "initNonce");
            }
            _ => panic!("Expected a constructor declaration"),
        }
    }

}
