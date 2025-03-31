#![warn(clippy::pedantic)]
use anyhow::{anyhow, bail, Ok, Result};
use std::rc::Rc;
use tree_sitter::Node;

use crate::{
    ast::{directive::VersionExpr, literal::VersionOperator},
    codebase::{Codebase, OpenState},
};

use super::{
    declaration::{
        Argument, Constructor, Contract, Declaration, Export, GArgument, Import, Include, Ledger,
        Pattern, PatternArgument, StructPattern, StructPatternField, TuplePattern, Witness,
    },
    definition::{Circuit, Definition, Enum, Module, Structure},
    directive::{Directive, Pragma},
    expression::{
        Binary, BinaryExpressionOperator, Cast, Conditional, Disclose, Expression, Fold,
        FunctionCall, Identifier, IndexAccess, Map, MemberAccess, Sequence, StructExpr,
        StructExprArg, StructNamedField, Unary, UnaryExpressionOperator,
    },
    function::{AnonymousFunction, Function, FunctionArgument, NamedFunction},
    literal::{Array, Bool, Literal, Nat, Pad, Str, Version},
    node::Location,
    node_type::NodeType,
    program::{CompactNode, Program},
    statement::{Assert, Assign, AssignOperator, Block, Const, For, If, Return, Statement},
    ty::{Bytes, Opaque, Ref, Sum, Type, TypeBool, TypeField, Uint, Vector, VectorSize},
};

/// Builds an AST from the given root node and source code.
///
/// # Errors
/// This function will return an error if the root node kind is not `source_file` or if any child node cannot be processed.
///
/// # Panics
/// This function will panic if `root.named_child(i).unwrap()` fails.
pub fn build_ast(
    codebase: &mut Codebase<OpenState>,
    root: &Node,
    source: &str,
) -> Result<Rc<Program>> {
    if root.kind() != "source_file" {
        bail!("Invalid root node kind: {}", root.kind());
    }
    let mut directives: Vec<Directive> = Vec::new();
    let mut declarations: Vec<Declaration> = Vec::new();
    let mut definitions: Vec<Definition> = Vec::new();
    let mut modules: Vec<Rc<Module>> = Vec::new();

    let node_id = node_id();
    for i in 0..root.named_child_count() {
        let child = root.named_child(i).unwrap();
        match build_compact_node(codebase, &child, source, node_id)? {
            CompactNode::Directive(d) => directives.push(d),
            CompactNode::Declaration(d) => declarations.push(d),
            CompactNode::Definition(d) => definitions.push(d),
            CompactNode::Module(m) => modules.push(m),
            CompactNode::Comment(_) => {}
        }
    }
    let p = Rc::new(Program {
        id: node_id,
        location: location(root, source),
        directives,
        declarations,
        definitions,
        modules,
    });
    codebase.add_node(NodeType::Program(p.clone()), 0);
    Ok(p)
}

fn build_compact_node(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<CompactNode> {
    match node.kind() {
        "pragma" => {
            let pragma = build_pragma(codebase, node, source, parent_id)?;
            Ok(CompactNode::Directive(Directive::Pragma(Rc::new(pragma))))
        }
        "incld" => {
            let include = build_include(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Include(Rc::new(
                include,
            ))))
        }
        "idecl" => {
            let import_decl = build_import(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Import(Rc::new(
                import_decl,
            ))))
        }
        "xdecl" => {
            let export_decl = build_export(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Export(Rc::new(
                export_decl,
            ))))
        }
        "ldecl" => {
            let ledger_decl = build_ledger(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Ledger(Rc::new(
                ledger_decl,
            ))))
        }
        "wdecl" => {
            let witness_decl = build_witness(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Witness(Rc::new(
                witness_decl,
            ))))
        }
        "cdefn" => {
            let circuit = build_circuit(codebase, node, source, parent_id)?;
            Ok(CompactNode::Definition(Definition::Circuit(Rc::new(
                circuit,
            ))))
        }
        "edecl" => {
            let circuit = build_external_circuit(codebase, node, source, parent_id)?;
            Ok(CompactNode::Definition(Definition::Circuit(Rc::new(
                circuit,
            ))))
        }
        "struct" => {
            let structure = build_structure(codebase, node, source, parent_id)?;
            Ok(CompactNode::Definition(Definition::Structure(Rc::new(
                structure,
            ))))
        }
        "enumdef" => {
            let enum_def = build_enum(codebase, node, source, parent_id)?;
            Ok(CompactNode::Definition(Definition::Enum(Rc::new(enum_def))))
        }
        "ecdecl" => {
            let contract = build_external_contract(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Contract(Rc::new(
                contract,
            ))))
        }
        "lconstructor" => {
            let constructor = build_constructor(codebase, node, source, parent_id)?;
            Ok(CompactNode::Declaration(Declaration::Constructor(Rc::new(
                constructor,
            ))))
        }
        "mdefn" => {
            let module = build_module(codebase, node, source, parent_id)?;
            Ok(CompactNode::Module(Rc::new(module)))
        }
        "comment" => {
            let comment = build_str(codebase, node, source, parent_id)?;
            Ok(CompactNode::Comment(comment))
        }
        other => bail!("Unhandled node kind: {}", other),
    }
}

fn build_module(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Module> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in module definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(codebase, &name_node, source, parent_id)?;
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(codebase, generic_node, source, parent_id));
    let cursor = &mut node.walk();
    let module_element_nodes = node.children_by_field_name("module_element", cursor);
    let mut nodes = Vec::new();
    let module_id = node_id();
    for child in module_element_nodes {
        let compact_node = build_compact_node(codebase, &child, source, module_id)?;
        nodes.push(compact_node);
    }
    let module = Module {
        id: module_id,
        location: location(node, source),
        is_exported,
        name,
        generic_parameters,
        nodes,
    };
    codebase.add_node(
        NodeType::Definition(Definition::Module(Rc::new(module.clone()))),
        parent_id,
    );
    Ok(module)
}

fn build_enum(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Enum> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in enum definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(codebase, &name_node, source, parent_id)?;
    let enum_id = node_id();
    let ids = node
        .children_by_field_name("id", &mut node.walk())
        .map(|id_node| build_identifier(codebase, &id_node, source, enum_id))
        .collect::<Result<Vec<_>>>()?;

    let enum_def = Enum {
        id: enum_id,
        location: location(node, source),
        is_exported,
        name,
        options: ids,
    };

    codebase.add_node(
        NodeType::Definition(Definition::Enum(Rc::new(enum_def.clone()))),
        parent_id,
    );

    Ok(enum_def)
}

#[allow(clippy::too_many_lines)]
fn build_pragma(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Pragma> {
    // Retrieve the 'id' field and build the pragma identifier.
    let id_node = node
        .child_by_field_name("id")
        .ok_or_else(|| anyhow!("Missing 'id' field in pragma"))?;
    let identifier = build_identifier(codebase, &id_node, source, parent_id)?;

    // Ensure the pragma has at least one version expression (after the id).
    let child_count = node.named_child_count();
    if child_count < 2 {
        bail!("Missing version expression in pragma");
    }

    // Collect all tokens that make up the version expression.
    let tokens: Vec<_> = (1..child_count)
        .map(|i| node.named_child(i).unwrap())
        .collect();
    let mut pos = 0;

    // Iteratively parse the version expression using a shunting-yard style approach.
    let mut output_stack: Vec<VersionExpr> = Vec::new();
    let mut op_stack: Vec<(String, i32)> = Vec::new();

    while pos < tokens.len() {
        let token = &tokens[pos];
        match token.kind() {
            // Handle unary operator followed by a literal.
            "not"
            | "greater_than"
            | "greater_than_or_equal"
            | "less_than"
            | "less_than_or_equal" => {
                let operator = match token.kind() {
                    "not" => VersionOperator::Neq,
                    "greater_than" => VersionOperator::Gt,
                    "greater_than_or_equal" => VersionOperator::Ge,
                    "less_than" => VersionOperator::Lt,
                    "less_than_or_equal" => VersionOperator::Le,
                    _ => unreachable!(),
                };
                pos += 1; // Consume the operator token.
                if pos >= tokens.len() {
                    bail!("Expected version literal after unary operator");
                }
                let literal = &tokens[pos];
                if literal.kind() != "version" && literal.kind() != "nat" {
                    bail!(
                        "Expected version literal after unary operator, found {}",
                        literal.kind()
                    );
                }
                let version = build_version(codebase, literal, operator, source, parent_id)?;
                output_stack.push(VersionExpr::Version(version));
                pos += 1;
            }
            // Handle opening parenthesis.
            "(" => {
                op_stack.push(("(".to_string(), 0));
                pos += 1;
            }
            // Handle closing parenthesis: unwind the operator stack.
            ")" => {
                while let Some((op, _)) = op_stack.pop() {
                    if op == "(" {
                        break;
                    }
                    let right = output_stack
                        .pop()
                        .ok_or_else(|| anyhow!("Missing operand for operator"))?;
                    let left = output_stack
                        .pop()
                        .ok_or_else(|| anyhow!("Missing left operand for operator"))?;
                    let expr = match op.as_str() {
                        "and" => VersionExpr::And(Box::new(left), Box::new(right)),
                        "or" => VersionExpr::Or(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                    output_stack.push(expr);
                }
                pos += 1;
            }
            // Handle a bare version literal.
            "version" | "nat" => {
                let version =
                    build_version(codebase, token, VersionOperator::Eq, source, parent_id)?;
                output_stack.push(VersionExpr::Version(version));
                pos += 1;
            }
            // Handle binary operators.
            "and" | "or" => {
                // Define precedence: "and" (2) binds tighter than "or" (1).
                let curr_prec = if token.kind() == "and" { 2 } else { 1 };
                while let Some((op, prec)) = op_stack.last() {
                    if *op != "(" && *prec >= curr_prec {
                        let (op, _) = op_stack.pop().unwrap();
                        let right = output_stack
                            .pop()
                            .ok_or_else(|| anyhow!("Missing operand for operator"))?;
                        let left = output_stack
                            .pop()
                            .ok_or_else(|| anyhow!("Missing left operand for operator"))?;
                        let expr = match op.as_str() {
                            "and" => VersionExpr::And(Box::new(left), Box::new(right)),
                            "or" => VersionExpr::Or(Box::new(left), Box::new(right)),
                            _ => unreachable!(),
                        };
                        output_stack.push(expr);
                    } else {
                        break;
                    }
                }
                op_stack.push((token.kind().to_string(), curr_prec));
                pos += 1;
            }
            // Any unexpected token results in an error.
            other => bail!("Unexpected token in version expression: {}", other),
        }
    }

    // Finish by unwinding any remaining operators.
    while let Some((op, _)) = op_stack.pop() {
        if op == "(" {
            bail!("Mismatched parenthesis in version expression");
        }
        let right = output_stack
            .pop()
            .ok_or_else(|| anyhow!("Missing operand for operator"))?;
        let left = output_stack
            .pop()
            .ok_or_else(|| anyhow!("Missing left operand for operator"))?;
        let expr = match op.as_str() {
            "and" => VersionExpr::And(Box::new(left), Box::new(right)),
            "or" => VersionExpr::Or(Box::new(left), Box::new(right)),
            _ => unreachable!(),
        };
        output_stack.push(expr);
    }

    if output_stack.len() != 1 {
        bail!("Invalid pragma structure: unused tokens in version expression");
    }
    let version_expr = output_stack.pop().unwrap();

    let pragma = Pragma {
        id: node_id(),
        location: location(node, source),
        version: version_expr,
        value: identifier,
    };

    codebase.add_node(
        NodeType::Directive(Directive::Pragma(Rc::new(pragma.clone()))),
        parent_id,
    );

    Ok(pragma)
}

fn build_version(
    codebase: &mut Codebase<OpenState>,
    version_node: &Node,
    version_operator: VersionOperator,
    source: &str,
    parent_id: u128,
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

    let major_nat = Rc::new(Nat {
        id: node_id(),
        location: location(version_node, source),
        value: major,
    });
    codebase.add_node(
        NodeType::Literal(Literal::Nat(major_nat.clone())),
        parent_id,
    );

    let minor_nat = Rc::new(Nat {
        id: node_id(),
        location: location(version_node, source),
        value: minor,
    });
    codebase.add_node(
        NodeType::Literal(Literal::Nat(minor_nat.clone())),
        parent_id,
    );

    let bugfix_nat = bugfix.map(|b| {
        let nat = Rc::new(Nat {
            id: node_id(),
            location: location(version_node, source),
            value: b,
        });
        codebase.add_node(NodeType::Literal(Literal::Nat(nat.clone())), parent_id);
        nat
    });

    let version = Rc::new(Version {
        id: node_id(),
        location: location(version_node, source),
        major: major_nat,
        minor: Some(minor_nat),
        bugfix: bugfix_nat,
        operator: version_operator,
    });

    codebase.add_node(
        NodeType::Literal(Literal::Version(version.clone())),
        parent_id,
    );
    Ok(version)
}

fn build_include(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Include> {
    let include_id = node_id();
    let path_node = node.child_by_field_name("file").ok_or_else(|| {
        anyhow!(
            "Missing 'file' field in include declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let path = path_node.utf8_text(source.as_bytes())?;
    let include = Include {
        id: include_id,
        location: location(node, source),
        path: path.to_string(),
    };
    codebase.add_node(
        NodeType::Declaration(Declaration::Include(Rc::new(include.clone()))),
        parent_id,
    );
    Ok(include)
}

fn build_import(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Import> {
    let import_id = node_id();
    let import_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in import declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let identifier = build_identifier(codebase, &import_name_node, source, import_id)?;
    let generic_parameters_node = node.child_by_field_name("gargs");
    let mut generic_parameters: Option<Vec<GArgument>> = None;
    if let Some(generics_node) = generic_parameters_node {
        let cursor = &mut generics_node.walk();
        let generic_nodes: Result<Vec<_>> = generics_node
            .children_by_field_name("garg", cursor)
            .map(|type_node| {
                build_gargument(codebase, &type_node.child(0).unwrap(), source, import_id)
            })
            .collect();
        generic_parameters = Some(generic_nodes?);
    }
    let prefix = node
        .child_by_field_name("prefix")
        .map(|prefix_node| {
            build_identifier(
                codebase,
                &prefix_node.child_by_field_name("id").unwrap(),
                source,
                import_id,
            )
        })
        .transpose()?;
    let import = Import {
        id: import_id,
        location: location(node, source),
        value: identifier,
        prefix,
        generic_parameters,
    };
    codebase.add_node(
        NodeType::Declaration(Declaration::Import(Rc::new(import.clone()))),
        parent_id,
    );
    Ok(import)
}

fn build_export(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Export> {
    let export_id = node_id();
    let mut cursor = node.walk();
    let export_names: Result<Vec<_>> = node
        .children_by_field_name("id", &mut cursor)
        .map(|id_node| build_identifier(codebase, &id_node, source, export_id))
        .collect();
    let export_names = export_names?;
    let export = Export {
        id: export_id,
        location: location(node, source),
        values: export_names,
    };
    codebase.add_node(
        NodeType::Declaration(Declaration::Export(Rc::new(export.clone()))),
        parent_id,
    );
    Ok(export)
}

fn build_ledger(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Ledger> {
    let ledger_id = node_id();
    let ledger_name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in ledger declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let is_exported = node.child_by_field_name("export").is_some();
    let is_sealed = node.child_by_field_name("sealed").is_some();
    let name = build_identifier(codebase, &ledger_name_node, source, ledger_id)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in ledger declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(codebase, &type_node, source, ledger_id)?;
    let ledger = Ledger {
        id: ledger_id,
        location: location(node, source),
        name,
        is_exported,
        is_sealed,
        ty,
    };
    codebase.add_node(
        NodeType::Declaration(Declaration::Ledger(Rc::new(ledger.clone()))),
        parent_id,
    );
    Ok(ledger)
}

fn build_witness(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Witness> {
    let witness_id = node_id();
    let witness_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in witness declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let is_exported = node.child_by_field_name("export").is_some();
    let name = build_identifier(codebase, &witness_name_node, source, witness_id)?;
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(codebase, generic_node, source, witness_id));

    let cursor = &mut node.walk();
    let arguments = node
        .children_by_field_name("arg", cursor)
        .map(|arg_node| build_argument(codebase, &arg_node, source, witness_id))
        .collect::<Result<Vec<_>>>()?;

    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in witness declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(codebase, &type_node, source, witness_id)?;

    let witness = Witness {
        id: witness_id,
        location: location(node, source),
        name,
        generic_parameters,
        arguments,
        ty,
        is_exported,
    };

    codebase.add_node(
        NodeType::Declaration(Declaration::Witness(Rc::new(witness.clone()))),
        parent_id,
    );

    Ok(witness)
}

fn build_circuit(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Circuit> {
    let circuit_id = node_id();
    let circuit_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in circuit definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(codebase, &circuit_name_node, source, circuit_id)?;
    let is_exported = node.child_by_field_name("export").is_some();
    let is_pure = node.child_by_field_name("pure").is_some();

    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(codebase, generic_node, source, circuit_id));

    let arguments = node
        .children_by_field_name("parg", &mut node.walk())
        .map(|arg_node| build_pargument(codebase, &arg_node, source, circuit_id))
        .collect::<Result<Vec<_>>>()?;

    let body_node = node.child_by_field_name("body").ok_or_else(|| {
        anyhow!(
            "Missing 'body' field in circuit definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;

    let body = build_block(codebase, &body_node, source, circuit_id)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in circuit definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(codebase, &type_node, source, circuit_id)?;

    let circuit = Circuit {
        id: circuit_id,
        location: location(node, source),
        name,
        is_exported,
        is_pure,
        generic_parameters,
        arguments,
        body: Some(body),
        ty,
    };

    codebase.add_node(
        NodeType::Definition(Definition::Circuit(Rc::new(circuit.clone()))),
        parent_id,
    );

    Ok(circuit)
}

fn build_external_circuit(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Circuit> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in external circuit: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let circuit_id = node_id();
    let name = build_identifier(codebase, &name_node, source, circuit_id)?;

    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(codebase, generic_node, source, circuit_id));

    let cursor = &mut node.walk();
    let arguments = node
        .children_by_field_name("arg", cursor)
        .map(|arg_node| {
            let arg = build_argument(codebase, &arg_node, source, circuit_id)?;
            Ok(Rc::new(PatternArgument {
                id: node_id(),
                location: location(&arg_node, source),
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
    let ty = build_type(codebase, &type_node, source, circuit_id)?;

    let circuit = Circuit {
        id: circuit_id,
        location: location(node, source),
        is_exported,
        is_pure: false,
        arguments,
        name,
        generic_parameters,
        body: None,
        ty,
    };

    codebase.add_node(
        NodeType::Definition(Definition::Circuit(Rc::new(circuit.clone()))),
        parent_id,
    );

    Ok(circuit)
}

fn build_constructor(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Constructor> {
    let constructor_id = node_id();
    let arguments = node
        .children_by_field_name("parg", &mut node.walk())
        .map(|arg_node| build_pargument(codebase, &arg_node, source, constructor_id))
        .collect::<Result<Vec<_>>>()?;

    let body_node = node.child_by_field_name("body").ok_or_else(|| {
        anyhow!(
            "Missing 'body' field in constructor declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;

    let body = build_block(codebase, &body_node, source, constructor_id)?;

    let constructor = Constructor {
        id: constructor_id,
        location: location(node, source),
        arguments,
        body,
    };

    codebase.add_node(
        NodeType::Declaration(Declaration::Constructor(Rc::new(constructor.clone()))),
        parent_id,
    );

    Ok(constructor)
}

fn build_external_contract(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Contract> {
    let is_exported = node.child_by_field_name("export").is_some();
    let name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in external contract: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let contract_id = node_id();
    let name = build_identifier(codebase, &name_node, source, contract_id)?;

    let cursor = &mut node.walk();
    let contract_circuit_nodes = node.children_by_field_name("contract_circuit", cursor);
    let mut circuits = Vec::new();

    for circuit_node in contract_circuit_nodes {
        let is_pure = circuit_node.child_by_field_name("pure").is_some();
        let id_node = circuit_node.child_by_field_name("id").ok_or_else(|| {
            anyhow!(
                "Missing 'id' field in contract circuit: {:?}",
                circuit_node.utf8_text(source.as_bytes())
            )
        })?;
        let circuit_name = build_identifier(codebase, &id_node, source, contract_id)?;

        let circuit_id = node_id();
        let arguments = circuit_node
            .children_by_field_name("arg", &mut node.walk())
            .map(|arg_node| {
                let arg = build_argument(codebase, &arg_node, source, circuit_id)?;
                codebase.add_node(
                    NodeType::Declaration(Declaration::Argument(arg.clone())),
                    circuit_id,
                );
                let pa = Rc::new(PatternArgument {
                    id: node_id(),
                    location: location(&arg_node, source),
                    pattern: Pattern::Identifier(arg.name.clone()),
                    ty: arg.ty.clone(),
                });
                codebase.add_node(
                    NodeType::Declaration(Declaration::PatternArgument(pa.clone())),
                    circuit_id,
                );
                Ok(pa)
            })
            .collect::<Result<Vec<_>>>()?;

        let ty_node = circuit_node.child_by_field_name("type").ok_or_else(|| {
            anyhow!(
                "Missing 'type' field in contract circuit: {:?}",
                circuit_node.utf8_text(source.as_bytes())
            )
        })?;
        let ty = build_type(codebase, &ty_node, source, circuit_id)?;

        let circuit = Rc::new(Circuit {
            id: circuit_id,
            location: location(&circuit_node, source),
            name: circuit_name,
            arguments,
            generic_parameters: None,
            is_exported: false,
            is_pure,
            ty,
            body: None,
        });

        codebase.add_node(
            NodeType::Definition(Definition::Circuit(circuit.clone())),
            contract_id,
        );
        circuits.push(circuit);
    }

    let contract = Contract {
        id: contract_id,
        location: location(node, source),
        is_exported,
        name,
        circuits,
    };

    codebase.add_node(
        NodeType::Declaration(Declaration::Contract(Rc::new(contract.clone()))),
        parent_id,
    );

    Ok(contract)
}

fn build_structure(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Structure> {
    let structure_name_node = node.child_by_field_name("name").ok_or_else(|| {
        anyhow!(
            "Missing 'name' field in structure definition: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(codebase, &structure_name_node, source, parent_id)?;
    let is_exported = node.child_by_field_name("export").is_some();
    let generic_parameters_node = node.child_by_field_name("gparams");
    let generic_parameters = generic_parameters_node
        .as_ref()
        .map(|generic_node| build_generic_parameters(codebase, generic_node, source, parent_id));

    let structure_id = node_id();
    let fields = node
        .children_by_field_name("arg", &mut node.walk())
        .map(|field_node| build_argument(codebase, &field_node, source, structure_id))
        .collect::<Result<Vec<_>>>()?;

    let structure = Structure {
        id: structure_id,
        location: location(node, source),
        name,
        is_exported,
        generic_parameters,
        fields,
    };

    codebase.add_node(
        NodeType::Definition(Definition::Structure(Rc::new(structure.clone()))),
        parent_id,
    );

    Ok(structure)
}

fn build_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Statement> {
    let node = if node.kind() == "stmt" {
        &node.child(0).unwrap()
    } else {
        node
    };
    let kind = node.kind();
    let statement = match kind {
        "assign_stmt" => {
            Statement::Assign(build_assign_statement(codebase, node, source, parent_id)?)
        }
        "block" => Statement::Block(build_block(codebase, node, source, parent_id)?),
        "if_stmt" => Statement::If(build_if_statement(codebase, node, source, parent_id)?),
        "for_stmt" => Statement::For(build_for_statement(codebase, node, source, parent_id)?),
        "return_stmt" => {
            Statement::Return(build_return_statement(codebase, node, source, parent_id)?)
        }
        "assert_stmt" => {
            Statement::Assert(build_assert_statement(codebase, node, source, parent_id)?)
        }
        "const_stmt" => Statement::Const(build_const_statement(codebase, node, source, parent_id)?),
        "expression_sequence_stmt" => Statement::ExpressionSequence(build_expression_sequence(
            codebase,
            &node.named_child(0).unwrap(),
            source,
            parent_id,
        )?),
        _ => bail!("Unhandled statement kind: {} {:?}", kind, node),
    };
    let statement_rc = statement;
    codebase.add_node(NodeType::Statement(statement_rc.clone()), parent_id);
    Ok(statement_rc)
}

fn build_assign_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Assign>> {
    let assign_id = node_id();
    let target_node = node.child_by_field_name("target").ok_or_else(|| {
        anyhow!(
            "Missing 'target' field in assign statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let target = build_expression(codebase, &target_node, source, assign_id)?;
    let value_node = node.child_by_field_name("value").ok_or_else(|| {
        anyhow!(
            "Missing 'value' field in assign statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let value = build_expression(codebase, &value_node, source, assign_id)?;
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
    let assign_stmt = Rc::new(Assign {
        id: assign_id,
        location: location(node, source),
        target,
        value,
        operator,
    });
    codebase.add_node(
        NodeType::Statement(Statement::Assign(assign_stmt.clone())),
        parent_id,
    );
    Ok(assign_stmt)
}

fn build_const_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Const>> {
    let const_id = node_id();
    let pattern_node = node.child_by_field_name("pattern").ok_or_else(|| {
        anyhow!(
            "Missing 'pattern' field in const statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let pattern = build_pattern(codebase, &pattern_node, source, const_id)?;
    let ty_node = node.child_by_field_name("type");
    let ty = if let Some(ty_n) = ty_node {
        Some(build_type(codebase, &ty_n, source, const_id)?)
    } else {
        None
    };
    let value_node = node.child_by_field_name("value").ok_or_else(|| {
        anyhow!(
            "Missing 'value' field in const statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let value = build_expression(codebase, &value_node, source, const_id)?;
    let const_stmt = Rc::new(Const {
        id: const_id,
        location: location(node, source),
        pattern,
        value,
        ty,
    });
    codebase.add_node(
        NodeType::Statement(Statement::Const(const_stmt.clone())),
        parent_id,
    );
    Ok(const_stmt)
}

fn build_if_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<If>> {
    let if_id = node_id();
    let condition_node = node.child_by_field_name("condition").ok_or_else(|| {
        anyhow!(
            "Missing 'condition' field in if statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let condition = build_expression_sequence(codebase, &condition_node, source, if_id)?;
    let then_branch_node = node.child_by_field_name("then_branch").ok_or_else(|| {
        anyhow!(
            "Missing 'then' field in if statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let then_branch = build_statement(codebase, &then_branch_node, source, if_id)?;
    let else_branch_node = node.child_by_field_name("else_branch");
    let else_branch = else_branch_node
        .map(|node| build_statement(codebase, &node, source, if_id))
        .transpose()?;
    let if_stmt = Rc::new(If {
        id: if_id,
        location: location(node, source),
        condition: Expression::Sequence(condition),
        then_branch,
        else_branch,
    });
    codebase.add_node(
        NodeType::Statement(Statement::If(if_stmt.clone())),
        parent_id,
    );
    Ok(if_stmt)
}

fn build_for_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<For>> {
    let for_id = node_id();
    let counter_node = node.child_by_field_name("counter").ok_or_else(|| {
        anyhow!(
            "Missing 'counter' field in for statement: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let counter = build_identifier(codebase, &counter_node, source, for_id)?;
    let range_start_node = node.child_by_field_name("range_start");
    let range_end_node = node.child_by_field_name("range_end");
    let limit_node = node.child_by_field_name("limit");

    let limit = if let Some(limit) = limit_node {
        Some(build_expression(codebase, &limit, source, for_id)?)
    } else {
        None
    };

    let range = if let (Some(start), Some(end)) = (range_start_node, range_end_node) {
        let start = build_nat(codebase, &start, source, for_id)?;
        let end = build_nat(codebase, &end, source, for_id)?;
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
    let body = build_block(codebase, &body_node.child(0).unwrap(), source, for_id)?;
    let for_stmt = Rc::new(For {
        id: for_id,
        location: location(node, source),
        counter,
        limit,
        range,
        body,
    });
    codebase.add_node(
        NodeType::Statement(Statement::For(for_stmt.clone())),
        parent_id,
    );
    Ok(for_stmt)
}

fn build_return_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Return>> {
    let return_id = node_id();
    let value_node = node.child_by_field_name("value");
    let value = if let Some(value_node) = value_node {
        Some(build_expression(codebase, &value_node, source, return_id)?)
    } else {
        None
    };
    let return_stmt = Rc::new(Return {
        id: return_id,
        location: location(node, source),
        value,
    });
    codebase.add_node(
        NodeType::Statement(Statement::Return(return_stmt.clone())),
        parent_id,
    );
    Ok(return_stmt)
}

fn build_assert_statement(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Assert>> {
    let assert_id = node_id();
    let condition_node = node
        .child_by_field_name("condition")
        .ok_or_else(|| anyhow!("Missing 'condition' field in assert statement: {:?}", node))?;
    let condition = build_expression(codebase, &condition_node, source, assert_id)?;
    let message_node = node.child_by_field_name("message");
    let message = if let Some(message_node) = message_node {
        Some(build_str(codebase, &message_node, source, assert_id)?)
    } else {
        None
    };
    let assert = Rc::new(Assert {
        id: assert_id,
        location: location(node, source),
        condition,
        msg: message,
    });
    codebase.add_node(
        NodeType::Statement(Statement::Assert(assert.clone())),
        parent_id,
    );
    Ok(assert)
}

fn build_block(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Block>> {
    let block_id = node_id();
    let mut cursor = node.walk();
    let statements: Result<Vec<_>> = node
        .children_by_field_name("stmt", &mut cursor)
        .map(|stmt_node| build_statement(codebase, &stmt_node, source, block_id))
        .collect();
    let block = Rc::new(Block {
        id: block_id,
        location: location(node, source),
        statements: statements?,
    });
    codebase.add_node(
        NodeType::Statement(Statement::Block(block.clone())),
        parent_id,
    );
    Ok(block)
}

#[allow(clippy::too_many_lines)]
fn build_expression(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Expression> {
    let expression = match node.kind() {
        "conditional_expr" => {
            let condition = build_expression(
                codebase,
                &node
                    .child_by_field_name("condition")
                    .ok_or_else(|| anyhow!("Missing 'condition' field in conditional_expr"))?,
                source,
                parent_id,
            )?;
            let then_branch = build_expression(
                codebase,
                &node
                    .child_by_field_name("then_branch")
                    .ok_or_else(|| anyhow!("Missing 'then_branch' field in conditional_expr"))?,
                source,
                parent_id,
            )?;
            let else_branch = build_expression(
                codebase,
                &node
                    .child_by_field_name("else_branch")
                    .ok_or_else(|| anyhow!("Missing 'else_branch' field in conditional_expr"))?,
                source,
                parent_id,
            )?;
            let conditional = Rc::new(Conditional {
                id: node_id(),
                location: location(node, source),
                condition,
                then_branch,
                else_branch,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Conditional(conditional.clone())),
                parent_id,
            );
            Expression::Conditional(conditional)
        }
        "cast_expr" => {
            let expression = build_expression(
                codebase,
                &node
                    .child_by_field_name("expr")
                    .ok_or_else(|| anyhow!("Missing 'expr' field in cast_expr"))?,
                source,
                parent_id,
            )?;
            let ty = build_type(
                codebase,
                &node
                    .child_by_field_name("type")
                    .ok_or_else(|| anyhow!("Missing 'type' field in cast_expr"))?,
                source,
                parent_id,
            )?;
            let cast = Rc::new(Cast {
                id: node_id(),
                location: location(node, source),
                expression,
                target_type: ty,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Cast(cast.clone())),
                parent_id,
            );
            Expression::Cast(cast)
        }
        "expr" => build_expression(codebase, &node.named_child(0).unwrap(), source, parent_id)?,
        "or_expr" => {
            let left = build_expression(
                codebase,
                &node.child_by_field_name("left").unwrap(),
                source,
                parent_id,
            )?;
            let right = build_expression(
                codebase,
                &node.child_by_field_name("right").unwrap(),
                source,
                parent_id,
            )?;
            let binary = Rc::new(Binary {
                id: node_id(),
                location: location(node, source),
                left,
                right,
                operator: BinaryExpressionOperator::Or,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Binary(binary.clone())),
                parent_id,
            );
            Expression::Binary(binary)
        }
        "and_expr" => {
            let left = build_expression(
                codebase,
                &node.child_by_field_name("left").unwrap(),
                source,
                parent_id,
            )?;
            let right = build_expression(
                codebase,
                &node.child_by_field_name("right").unwrap(),
                source,
                parent_id,
            )?;
            let binary = Rc::new(Binary {
                id: node_id(),
                location: location(node, source),
                left,
                right,
                operator: BinaryExpressionOperator::And,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Binary(binary.clone())),
                parent_id,
            );
            Expression::Binary(binary)
        }
        "comparison_expr" => {
            let left = build_expression(
                codebase,
                &node.child_by_field_name("left").unwrap(),
                source,
                parent_id,
            )?;
            let right = build_expression(
                codebase,
                &node.child_by_field_name("right").unwrap(),
                source,
                parent_id,
            )?;
            let operator_node = node.child_by_field_name("operator").unwrap();
            let operator = match operator_node.utf8_text(source.as_bytes())? {
                "==" => BinaryExpressionOperator::Eq,
                "!=" => BinaryExpressionOperator::Ne,
                _ => bail!(
                    "Invalid comparison operator: {:?}",
                    operator_node.utf8_text(source.as_bytes())?
                ),
            };
            let binary = Rc::new(Binary {
                id: node_id(),
                location: location(node, source),
                left,
                right,
                operator,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Binary(binary.clone())),
                parent_id,
            );
            Expression::Binary(binary)
        }
        "rel_comparison_expr" => {
            let left = build_expression(
                codebase,
                &node.child_by_field_name("left").unwrap(),
                source,
                parent_id,
            )?;
            let right = build_expression(
                codebase,
                &node.child_by_field_name("right").unwrap(),
                source,
                parent_id,
            )?;
            let operator_node = node.child_by_field_name("operator").unwrap();
            let operator = match operator_node.kind() {
                "less_than" => BinaryExpressionOperator::Lt,
                "less_than_or_equal" => BinaryExpressionOperator::Le,
                "greater_than" => BinaryExpressionOperator::Gt,
                "greater_than_or_equal" => BinaryExpressionOperator::Ge,
                _ => bail!("Invalid comparison operator {operator_node:?}"),
            };
            let binary = Rc::new(Binary {
                id: node_id(),
                location: location(node, source),
                left,
                right,
                operator,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Binary(binary.clone())),
                parent_id,
            );
            Expression::Binary(binary)
        }
        "bin_sum_expr" | "bin_mul_expr" => {
            let left = build_expression(
                codebase,
                &node.child_by_field_name("left").unwrap(),
                source,
                parent_id,
            )?;
            let right = build_expression(
                codebase,
                &node.child_by_field_name("right").unwrap(),
                source,
                parent_id,
            )?;
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
            let binary = Rc::new(Binary {
                id: node_id(),
                location: location(node, source),
                left,
                right,
                operator,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Binary(binary.clone())),
                parent_id,
            );
            Expression::Binary(binary)
        }
        "not_expr" => {
            let expr = build_expression(
                codebase,
                &node.child_by_field_name("expr").unwrap(),
                source,
                parent_id,
            )?;
            let unary = Rc::new(Unary {
                id: node_id(),
                location: location(node, source),
                operator: UnaryExpressionOperator::Not,
                operand: expr,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Unary(unary.clone())),
                parent_id,
            );
            Expression::Unary(unary)
        }
        "member_access_expr" => {
            let base = build_expression(
                codebase,
                &node.child_by_field_name("base").unwrap(),
                source,
                parent_id,
            )?;
            let member = build_identifier(
                codebase,
                &node.child_by_field_name("member").unwrap(),
                source,
                parent_id,
            )?;
            let arguments_node = node.child_by_field_name("arguments");
            let arguments = if let Some(arguments_node) = arguments_node {
                let arguments: Result<Vec<_>> = arguments_node
                    .children_by_field_name("expr", &mut arguments_node.walk())
                    .map(|arg_node| build_expression(codebase, &arg_node, source, parent_id))
                    .collect();
                Some(arguments?)
            } else {
                None
            };
            let member_access = Rc::new(MemberAccess {
                id: node_id(),
                location: location(node, source),
                base,
                member,
                arguments,
            });
            codebase.add_node(
                NodeType::Expression(Expression::MemberAccess(member_access.clone())),
                parent_id,
            );
            Expression::MemberAccess(member_access)
        }
        "index_access_expr" => {
            let base = build_expression(
                codebase,
                &node.child_by_field_name("base").unwrap(),
                source,
                parent_id,
            )?;
            let index = build_nat(
                codebase,
                &node.child_by_field_name("index").unwrap(),
                source,
                parent_id,
            )?;
            let index_access = Rc::new(IndexAccess {
                id: node_id(),
                location: location(node, source),
                base,
                index,
            });
            codebase.add_node(
                NodeType::Expression(Expression::IndexAccess(index_access.clone())),
                parent_id,
            );
            Expression::IndexAccess(index_access)
        }
        "expr_seq" => {
            let seq = build_expression_sequence(codebase, node, source, parent_id)?;
            codebase.add_node(
                NodeType::Expression(Expression::Sequence(seq.clone())),
                parent_id,
            );
            Expression::Sequence(seq)
        }
        "term" => build_term(codebase, node, source, parent_id)?,
        _ => bail!("Unhandled expression kind: {}", node.kind()),
    };
    Ok(expression)
}

#[allow(clippy::too_many_lines)]
fn build_term(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Expression> {
    let term_node = if node.kind() == "term" {
        &node.child(0).ok_or_else(|| anyhow!("Empty term node"))?
    } else {
        node
    };

    let term = match term_node.kind() {
        "lit" => build_literal(codebase, &term_node.child(0).unwrap(), source, parent_id)?,
        "default_term" => {
            let node_id = node_id();
            let type_node = term_node.child_by_field_name("type").ok_or_else(|| {
                anyhow!(
                    "Missing 'type' field in default term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let ty = build_type(codebase, &type_node, source, node_id)?;
            codebase.add_node(
                NodeType::Expression(Expression::Default(ty.clone())),
                parent_id,
            );
            Expression::Default(ty)
        }
        "map_term" => {
            let node_id = node_id();
            let fun_node = term_node.child_by_field_name("fun").ok_or_else(|| {
                anyhow!(
                    "Missing function in map term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(codebase, &fun_node, source, node_id)?;
            let expr_nodes = term_node
                .children_by_field_name("expr", &mut term_node.walk())
                .collect::<Vec<_>>();
            let expressions: Result<Vec<_>> = expr_nodes
                .into_iter()
                .map(|n| build_expression(codebase, &n, source, node_id))
                .collect();
            let t_map = Rc::new(Map {
                id: node_id,
                location: location(term_node, source),
                function: fun,
                expressions: expressions?,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Map(t_map.clone())),
                parent_id,
            );
            Expression::Map(t_map)
        }
        "fold_term" => {
            let node_id = node_id();
            let fun_node = term_node.child_by_field_name("fun").ok_or_else(|| {
                anyhow!(
                    "Missing function in fold term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(codebase, &fun_node, source, node_id)?;
            let init_value_node = term_node
                .child_by_field_name("init_value")
                .unwrap_or_else(|| term_node.child(4).unwrap());
            let initial_value = build_expression(codebase, &init_value_node, source, node_id)?;
            let exprs: Vec<_> = term_node
                .children_by_field_name("expr", &mut term_node.walk())
                .collect();
            let expressions: Result<Vec<_>> = exprs
                .into_iter()
                .map(|n| build_expression(codebase, &n, source, node_id))
                .collect();
            let e_fold = Rc::new(Fold {
                id: node_id,
                location: location(term_node, source),
                function: fun,
                initial_value,
                expressions: expressions?,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Fold(e_fold.clone())),
                parent_id,
            );
            Expression::Fold(e_fold)
        }
        "disclose_term" => {
            let node_id = node_id();
            let expr_node = term_node.child_by_field_name("expr").ok_or_else(|| {
                anyhow!(
                    "Missing expression in disclose term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let expr = build_expression(codebase, &expr_node, source, node_id)?;
            let disclose = Rc::new(Disclose {
                id: node_id,
                location: location(term_node, source),
                expression: expr,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Disclose(disclose.clone())),
                parent_id,
            );
            Expression::Disclose(disclose)
        }
        "id" => {
            let id = build_identifier(codebase, term_node, source, parent_id)?;
            Expression::Identifier(id)
        }
        "expr_seq_term" => {
            let node_id = node_id();
            let seq =
                build_expression_sequence(codebase, &term_node.child(1).unwrap(), source, node_id)?;
            codebase.add_node(
                NodeType::Expression(Expression::Sequence(seq.clone())),
                parent_id,
            );
            Expression::Sequence(seq)
        }
        "function_call_term" => {
            let fc_id = node_id();
            let fun_node = term_node.child_by_field_name("fun").ok_or_else(|| {
                anyhow!(
                    "Missing 'fun' field in function_call_term: {:?}",
                    term_node.utf8_text(source.as_bytes())
                )
            })?;
            let fun = build_function(codebase, &fun_node, source, fc_id)?;
            let expr_nodes = term_node
                .children_by_field_name("expr", &mut term_node.walk())
                .collect::<Vec<_>>();
            let arguments = expr_nodes
                .into_iter()
                .map(|expr_node| build_expression(codebase, &expr_node, source, fc_id))
                .collect::<Result<Vec<_>>>()?;
            let fun = Rc::new(FunctionCall {
                id: fc_id,
                location: location(term_node, source),
                function: Expression::Function(fun),
                arguments,
            });
            codebase.add_node(
                NodeType::Expression(Expression::FunctionCall(fun.clone())),
                parent_id,
            );
            Expression::FunctionCall(fun)
        }
        "struct_term" => {
            let node_id = node_id();
            let struct_expr = build_struct_expression(codebase, term_node, source, node_id)?;
            codebase.add_node(
                NodeType::Expression(Expression::Struct(struct_expr.clone())),
                parent_id,
            );
            Expression::Struct(struct_expr)
        }
        "[" => {
            let node_id = node_id();
            let expr_nodes = term_node
                .parent()
                .unwrap()
                .children_by_field_name("expr", &mut term_node.walk())
                .collect::<Vec<_>>();
            let elements = expr_nodes
                .into_iter()
                .map(|expr_node| build_expression(codebase, &expr_node, source, node_id))
                .collect::<Result<Vec<_>>>()?;
            let arr = Rc::new(Array {
                id: node_id,
                location: location(term_node, source),
                elements,
            });
            codebase.add_node(
                NodeType::Expression(Expression::Literal(Literal::Array(arr.clone()))),
                parent_id,
            );
            Expression::Literal(Literal::Array(arr))
        }
        _ => bail!("Unhandled term kind: {}", term_node.kind()),
    };
    Ok(term)
}

fn build_function(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Function> {
    let name_node = node.child_by_field_name("id");
    let node_id = node_id();
    if let Some(name_n) = name_node {
        let name = build_identifier(codebase, &name_n, source, node_id)?;
        let generic_parameters_node = node.child_by_field_name("gargs");
        let mut generic_parameters = None;
        if let Some(generics_node) = generic_parameters_node {
            let cursor = &mut generics_node.walk();
            let generic_nodes: Result<Vec<_>> = generics_node
                .children_by_field_name("garg", cursor)
                .map(|type_node| {
                    build_gargument(codebase, &type_node.child(0).unwrap(), source, node_id)
                })
                .collect();
            generic_parameters = Some(generic_nodes?);
        }
        let nf = Rc::new(NamedFunction {
            id: node_id,
            location: location(node, source),
            name,
            generic_parameters,
        });
        codebase.add_node(NodeType::Function(Function::Named(nf.clone())), parent_id);
        Ok(Function::Named(nf))
    } else {
        let cursor = &mut node.walk();
        let pattern_nodes = node
            .children_by_field_name("pattern", cursor)
            .map(|pattern_node| build_pattern(codebase, &pattern_node, source, node_id))
            .collect::<Result<Vec<_>>>()?;
        let parg_nodes = node
            .children_by_field_name("parg", cursor)
            .map(|parg_node| build_pargument(codebase, &parg_node, source, node_id))
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
            Some(build_type(codebase, &return_node, source, node_id)?)
        } else {
            None
        };
        let block_node = node.child_by_field_name("block");
        let block = if let Some(block_node) = block_node {
            Some(build_block(codebase, &block_node, source, node_id)?)
        } else {
            None
        };
        if block.is_some() {
            let af = Rc::new(AnonymousFunction {
                id: node_id,
                location: location(node, source),
                arguments,
                return_type,
                body: block,
                expr_body: None,
            });
            codebase.add_node(
                NodeType::Function(Function::Anonymous(af.clone())),
                parent_id,
            );
            Ok(Function::Anonymous(af))
        } else {
            let expr_node = node.child_by_field_name("expr").ok_or_else(|| {
                anyhow!(
                    "Missing 'expr' field in anonymous function: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let expr = build_expression(codebase, &expr_node, source, parent_id)?;
            let af = Rc::new(AnonymousFunction {
                id: node_id,
                location: location(node, source),
                arguments,
                return_type,
                body: None,
                expr_body: Some(expr),
            });
            codebase.add_node(
                NodeType::Function(Function::Anonymous(af.clone())),
                parent_id,
            );
            Ok(Function::Anonymous(af))
        }
    }
}

fn build_literal(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Expression> {
    let kind = node.kind();
    let literal = match kind {
        "true" => {
            let b = Rc::new(Bool {
                id: node_id(),
                location: location(node, source),
                value: true,
            });
            codebase.add_node(NodeType::Literal(Literal::Bool(b.clone())), parent_id);
            Expression::Literal(Literal::Bool(b))
        }
        "false" => {
            let b = Rc::new(Bool {
                id: node_id(),
                location: location(node, source),
                value: false,
            });
            codebase.add_node(NodeType::Literal(Literal::Bool(b.clone())), parent_id);
            Expression::Literal(Literal::Bool(b))
        }
        "nat" => {
            let nat = build_nat(codebase, node, source, parent_id)?;
            Expression::Literal(Literal::Nat(nat))
        }
        "str" => {
            let str = build_str(codebase, node, source, parent_id)?;
            Expression::Literal(Literal::Str(str))
        }
        "pad" => {
            let node_id = node_id();
            let nat_node = node.child_by_field_name("nat").ok_or_else(|| {
                anyhow!(
                    "Missing 'nat' field in pad literal: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let nat = build_nat(codebase, &nat_node, source, node_id)?;
            let str_node = node.child_by_field_name("str").ok_or_else(|| {
                anyhow!(
                    "Missing 'str' field in pad literal: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let str = build_str(codebase, &str_node, source, node_id)?;
            let pad = Rc::new(Pad {
                id: node_id,
                location: location(node, source),
                number: nat,
                name: str,
            });
            codebase.add_node(NodeType::Literal(Literal::Pad(pad.clone())), parent_id);
            Expression::Literal(Literal::Pad(pad))
        }
        _ => bail!("Unhandled literal kind: {:?}", node),
    };
    Ok(literal)
}

fn build_struct_expression(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<StructExpr>> {
    let struct_expr_id = node_id();
    let tref_node = node
        .child_by_field_name("tref")
        .ok_or_else(|| anyhow!("Missing 'tref' field in struct expression: {:?}", node))?;
    let tref = build_type(codebase, &tref_node, source, struct_expr_id)?;
    let cursor = &mut node.walk();
    let mut struct_args = Vec::new();
    for child in node.children(cursor) {
        if child.kind() == "struct_arg" {
            let struct_arg_node = child.named_child(0).unwrap();
            match struct_arg_node.kind() {
                "expr" => {
                    let expr =
                        build_expression(codebase, &struct_arg_node, source, struct_expr_id)?;
                    struct_args.push(StructExprArg::Expression(expr));
                }
                "struct_named_filed_initializer" => {
                    let struct_named_filed_initializer = node_id();
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
                    let name = build_identifier(
                        codebase,
                        &id_node,
                        source,
                        struct_named_filed_initializer,
                    )?;
                    let expr = build_expression(
                        codebase,
                        &expr_node,
                        source,
                        struct_named_filed_initializer,
                    )?;
                    let struct_arg = StructExprArg::NamedField(Rc::new(StructNamedField {
                        id: struct_named_filed_initializer,
                        location: location(&struct_arg_node, source),
                        name,
                        value: expr,
                    }));
                    codebase.add_node(NodeType::StructExprArg(struct_arg.clone()), struct_expr_id);
                    struct_args.push(struct_arg);
                }
                "struct_update_field" => {
                    let expr_node = struct_arg_node
                        .child_by_field_name("expr")
                        .ok_or_else(|| anyhow!("Missing 'expr' field in struct_update_field"))?;
                    let expr = build_expression(codebase, &expr_node, source, parent_id)?;
                    let struct_arg = StructExprArg::Update(expr.clone());
                    codebase.add_node(NodeType::StructExprArg(struct_arg.clone()), struct_expr_id);
                    struct_args.push(struct_arg);
                }
                _ => bail!("Unhandled struct_arg node: {}", struct_arg_node.kind()),
            }
        }
    }
    let struct_expr = Rc::new(StructExpr {
        id: struct_expr_id,
        location: location(node, source),
        ty: tref,
        args: struct_args,
    });
    codebase.add_node(
        NodeType::Expression(Expression::Struct(struct_expr.clone())),
        parent_id,
    );
    Ok(struct_expr)
}

fn build_expression_sequence(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Sequence>> {
    let node_id = node_id();
    let mut cursor = node.walk();
    let expressions: Result<Vec<_>> = node
        .children_by_field_name("expr", &mut cursor)
        .map(|expr_node| build_expression(codebase, &expr_node, source, node_id))
        .collect();
    let seq = Rc::new(Sequence {
        id: node_id,
        location: location(node, source),
        expressions: expressions?,
    });
    codebase.add_node(
        NodeType::Expression(Expression::Sequence(seq.clone())),
        parent_id,
    );
    Ok(seq)
}

#[allow(clippy::too_many_lines)]
fn build_type(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Type> {
    let node = if node.kind() == "type" {
        &node.child(0).unwrap()
    } else {
        node
    };
    let kind = node.kind();
    match kind {
        "tref" => {
            let node_id = node_id();
            let ref_node = node.child_by_field_name("id").ok_or_else(|| {
                anyhow!(
                    "Missing 'id' field in type reference: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let ref_name = build_identifier(codebase, &ref_node, source, node_id)?;
            let generic_parameters_node = node.child_by_field_name("gargs");
            let mut generic_parameters: Option<Vec<GArgument>> = None;
            if let Some(generics_node) = generic_parameters_node {
                let cursor = &mut generics_node.walk();
                let generic_nodes: Result<Vec<_>> = generics_node
                    .children_by_field_name("garg", cursor)
                    .map(|type_node| {
                        build_gargument(codebase, &type_node.child(0).unwrap(), source, node_id)
                    })
                    .collect();
                generic_parameters = Some(generic_nodes?);
            }
            let t_ref = Type::Ref(Rc::new(Ref {
                id: node_id,
                location: location(node, source),
                name: ref_name,
                generic_parameters,
            }));
            codebase.add_node(NodeType::Type(t_ref.clone()), parent_id);
            Ok(t_ref)
        }
        "Boolean" => {
            let b = Type::Boolean(Rc::new(TypeBool {
                id: node_id(),
                location: location(node, source),
            }));
            codebase.add_node(NodeType::Type(b.clone()), parent_id);
            Ok(b)
        }
        "Field" => {
            let f = Type::Field(Rc::new(TypeField {
                id: node_id(),
                location: location(node, source),
            }));
            codebase.add_node(NodeType::Type(f.clone()), parent_id);
            Ok(f)
        }
        "uint_type" => {
            let node_id = node_id();
            let cursor = &mut node.walk();
            let size_nodes = node
                .children_by_field_name("tsize", cursor)
                .map(|size_node| build_nat(codebase, &size_node, source, node_id).unwrap())
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
            let t_uint = Type::Uint(Rc::new(Uint {
                id: node_id,
                location: location(node, source),
                start,
                end,
            }));
            codebase.add_node(NodeType::Type(t_uint.clone()), parent_id);
            Ok(t_uint)
        }
        "bytes_type" => {
            let node_id = node_id();
            let size_node = node.child_by_field_name("tsize").ok_or_else(|| {
                anyhow!(
                    "Missing 'tsize' field in Bytes type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let nat = build_nat(codebase, &size_node, source, parent_id)?;
            let t_bytes = Type::Bytes(Rc::new(Bytes {
                id: node_id,
                location: location(node, source),
                size: nat,
            }));
            codebase.add_node(NodeType::Type(t_bytes.clone()), parent_id);
            Ok(t_bytes)
        }
        "opaque_type" => {
            let node_id = node_id();
            let size_node = node.child_by_field_name("str").ok_or_else(|| {
                anyhow!(
                    "Missing 'str' field in Opaque type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let str = build_str(codebase, &size_node, source, node_id)?;
            let t_opaque = Type::Opaque(Rc::new(Opaque {
                id: node_id,
                location: location(node, source),
                value: str,
            }));
            codebase.add_node(NodeType::Type(t_opaque.clone()), parent_id);
            Ok(t_opaque)
        }
        "vector_type" => {
            let node_id = node_id();
            let size_node = node.child_by_field_name("tsize").ok_or_else(|| {
                anyhow!(
                    "Missing 'tsize' field in Vector type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let size = match size_node.child(0).unwrap().kind() {
                "nat" => VectorSize::Nat(build_nat(codebase, &size_node, source, node_id)?),
                "id" => VectorSize::Ref(build_identifier(codebase, &size_node, source, node_id)?),
                _ => bail!("Invalid size kind: {:?}", size_node.kind()),
            };
            let element_node = node.child_by_field_name("type").ok_or_else(|| {
                anyhow!(
                    "Missing 'type' field in Vector type: {:?}",
                    node.utf8_text(source.as_bytes())
                )
            })?;
            let element_type = build_type(codebase, &element_node, source, node_id)?;
            let t_vector = Type::Vector(Rc::new(Vector {
                id: node_id,
                location: location(node, source),
                size,
                ty: element_type,
            }));
            codebase.add_node(NodeType::Type(t_vector.clone()), parent_id);
            Ok(t_vector)
        }
        "[" => {
            let node_id = node_id();
            let mut cursor = node.walk();
            let type_nodes: Result<Vec<_>> = node
                .children_by_field_name("type", &mut cursor)
                .map(|size_node| build_type(codebase, &size_node, source, node_id))
                .collect();
            let sizes = type_nodes?;
            let l_sum = Type::Sum(Rc::new(Sum {
                id: node_id,
                location: location(node, source),
                types: sizes,
            }));
            codebase.add_node(NodeType::Type(l_sum.clone()), parent_id);
            Ok(l_sum)
        }
        _ => bail!("Unhandled type kind: {}", kind),
    }
}

fn build_gargument(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<GArgument> {
    match node.kind() {
        "nat" => {
            let nat = build_nat(codebase, node, source, parent_id)?;
            codebase.add_node(NodeType::GArgument(GArgument::Nat(nat.clone())), parent_id);
            Ok(GArgument::Nat(nat))
        }
        "type" => {
            let ty = build_type(codebase, node, source, parent_id)?;
            codebase.add_node(NodeType::GArgument(GArgument::Type(ty.clone())), parent_id);
            Ok(GArgument::Type(ty))
        }
        _ => bail!(
            "Unhandled generic argument kind: {:?}",
            node.child(0).unwrap().kind()
        ),
    }
}

fn build_argument(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Argument>> {
    let node_id = node_id();
    let name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let name = build_identifier(codebase, &name_node, source, node_id)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(codebase, &type_node, source, node_id)?;
    let argument = Rc::new(Argument {
        id: node_id,
        location: location(node, source),
        name,
        ty,
    });
    codebase.add_node(
        NodeType::Declaration(Declaration::Argument(argument.clone())),
        parent_id,
    );
    Ok(argument)
}

fn build_pargument(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<PatternArgument>> {
    let node_id = node_id();
    let pattern_node = node.child_by_field_name("pattern").ok_or_else(|| {
        anyhow!(
            "Missing 'pattern' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let pattern = build_pattern(codebase, &pattern_node.child(0).unwrap(), source, node_id)?;
    let type_node = node.child_by_field_name("type").ok_or_else(|| {
        anyhow!(
            "Missing 'type' field in argument: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ty = build_type(codebase, &type_node, source, node_id)?;
    let pattern = Rc::new(PatternArgument {
        id: node_id,
        location: location(node, source),
        pattern,
        ty,
    });
    codebase.add_node(
        NodeType::Declaration(Declaration::PatternArgument(pattern.clone())),
        parent_id,
    );
    Ok(pattern)
}

fn build_pattern(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Pattern> {
    let node = if node.kind() == "pattern" {
        &node.child(0).unwrap()
    } else {
        node
    };
    let kind = node.kind();
    match kind {
        "id" => {
            let name = build_identifier(codebase, node, source, parent_id)?;
            Ok(Pattern::Identifier(name))
        }
        "[" => {
            let node_id = node_id();
            let cursor = &mut node.walk();
            let patterns: Result<Vec<_>> = node
                .parent()
                .unwrap()
                .children_by_field_name("pattern_tuple_elt", cursor)
                .map(|pattern_node| {
                    build_pattern(codebase, &pattern_node.child(0).unwrap(), source, node_id)
                })
                .collect();
            let patterns = patterns?;
            let tuple_pattern = Rc::new(TuplePattern {
                id: node_id,
                location: location(node, source),
                patterns,
            });
            codebase.add_node(
                NodeType::Pattern(Pattern::Tuple(tuple_pattern.clone())),
                parent_id,
            );
            Ok(Pattern::Tuple(tuple_pattern))
        }
        "{" => {
            let struct_pattern_node_id = node_id();
            let mut cursor = node.walk();
            let field_nodes = node
                .children_by_field_name("pattern_struct_elt", &mut cursor)
                .collect::<Vec<_>>();
            let mut fields = Vec::new();
            for field_node in field_nodes {
                let struct_pattern_node_id = node_id();
                let name_node = field_node.child_by_field_name("id").ok_or_else(|| {
                    anyhow!(
                        "Missing 'id' field in struct pattern field: {:?}",
                        field_node.utf8_text(source.as_bytes())
                    )
                })?;
                let name = build_identifier(codebase, &name_node, source, struct_pattern_node_id)?;
                let pattern_node = field_node.child_by_field_name("pattern").ok_or_else(|| {
                    anyhow!(
                        "Missing 'pattern' field in struct pattern field: {:?}",
                        field_node.utf8_text(source.as_bytes())
                    )
                })?;
                let pattern =
                    build_pattern(codebase, &pattern_node, source, struct_pattern_node_id)?;
                let field = Rc::new(StructPatternField {
                    id: struct_pattern_node_id,
                    location: location(&field_node, source),
                    name,
                    pattern,
                });
                codebase.add_node(
                    NodeType::Declaration(Declaration::StructPatternField(field.clone())),
                    struct_pattern_node_id,
                );
                fields.push(field);
            }
            let struct_pattern = Rc::new(StructPattern {
                id: struct_pattern_node_id,
                location: location(node, source),
                fields,
            });
            codebase.add_node(
                NodeType::Pattern(Pattern::Struct(struct_pattern.clone())),
                parent_id,
            );
            Ok(Pattern::Struct(struct_pattern))
        }
        _ => bail!("Unhandled pattern kind: {:?}", kind),
    }
}

fn build_generic_parameters(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Vec<Rc<Identifier>> {
    let mut cursor = node.walk();
    let generic_nodes: Result<Vec<_>> = node
        .children_by_field_name("gparam", &mut cursor)
        .map(|ident_node| build_identifier(codebase, &ident_node, source, parent_id))
        .collect();
    generic_nodes.unwrap()
}

fn build_identifier(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Identifier>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    let id = Rc::new(Identifier {
        id: node_id(),
        location: location(node, source),
        name: text,
    });
    codebase.add_node(
        NodeType::Expression(Expression::Identifier(id.clone())),
        parent_id,
    );
    Ok(id)
}

fn build_nat(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Nat>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    let value = text
        .parse::<u64>()
        .map_err(|_| anyhow!("Invalid Nat value: {}", text))?;
    let nat = Rc::new(Nat {
        id: node_id(),
        location: location(node, source),
        value,
    });
    codebase.add_node(NodeType::Literal(Literal::Nat(nat.clone())), parent_id);
    Ok(nat)
}

fn build_str(
    codebase: &mut Codebase<OpenState>,
    node: &Node,
    source: &str,
    parent_id: u128,
) -> Result<Rc<Str>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    let str = Rc::new(Str {
        id: node_id(),
        location: location(node, source),
        value: text,
    });
    codebase.add_node(NodeType::Literal(Literal::Str(str.clone())), parent_id);
    Ok(str)
}

fn location(node: &Node, source: &str) -> Location {
    let offset_start = node.start_byte();
    let offset_end = node.end_byte();
    let start_position = node.start_position();
    let end_position = node.end_position();
    let start_line = start_position.row + 1;
    let start_column = start_position.column + 1;
    let end_line = end_position.row + 1;
    let end_column = end_position.column + 1;
    let source = source[offset_start..offset_end].to_string();

    Location {
        offset_start,
        offset_end,
        start_line,
        start_column,
        end_line,
        end_column,
        source,
    }
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

    use crate::codebase::SourceCodeFile;

    use super::*;

    fn parse_content(fname: &str, content: &str) -> Result<SourceCodeFile> {
        let compact_language = tree_sitter_compact::LANGUAGE.into();
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&compact_language)
            .expect("Error loading Inference grammar");
        let tree = parser.parse(content, None).unwrap();
        let root_node = tree.root_node();
        let mut codebase = Codebase::new();
        let ast = build_ast(&mut codebase, &root_node, content)?;
        let source_code_file = SourceCodeFile {
            fname: fname.to_string(),
            ast,
        };
        Ok(source_code_file)
    }

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
