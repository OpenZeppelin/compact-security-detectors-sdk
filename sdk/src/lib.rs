#![warn(clippy::pedantic)]
use crate::ast::{declaration::Argument, expression::Identifier};
use crate::ast::{
    declaration::{Declaration, Import, Pattern},
    definition::{Circuit, Definition, Enum as AstEnum, Field, Module, Structure},
    directive::{Directive, Pragma},
    program::{CompactNode, Program},
    statement::{Block, Statement},
    ty::{Ref, Type},
};
use anyhow::{anyhow, bail, Result};
use ast::declaration::{Export, Ledger};
use ast::directive::VersionExpr;
use ast::literal::{Nat, Version, VersionOperator};
use ast::node::Location;
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
            // "wdecl" => {
            //     let witness_decl = build_witness(&child, source)?;
            //     declarations.push(Declaration::Witness(Rc::new(witness_decl)));
            // }
            // // circuit definition
            // "cdefn" => {
            //     let circuit = build_circuit(&child, source)?;
            //     definitions.push(Definition::Circuit(Rc::new(circuit)));
            // }
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
    let ledger_name_node = node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in ledger declaration: {:?}",
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

fn build_type(node: &Node, source: &str) -> Result<Type> {
    let ref_node = node.child_by_field_name("ref").ok_or_else(|| {
        anyhow!(
            "Missing 'ref' field in type declaration: {:?}",
            node.utf8_text(source.as_bytes())
        )
    })?;
    let ref_name_node = ref_node.child_by_field_name("id").ok_or_else(|| {
        anyhow!(
            "Missing 'id' field in type declaration: {:?}",
            ref_node.utf8_text(source.as_bytes())
        )
    })?;
    let ref_name = build_identifier(&ref_name_node, source)?;
    Ok(Type::Ref(Rc::new(Ref {
        id: node_id(),
        location: location(node),
        name: ref_name,
        generic_parameters: None,
    })))
}

// /// Parse a module definition node.
// /// Grammar: mdefn → export^opt "module" module-name gparams^opt "{" pelt* "}"
// fn build_module(node: &Node, source: &str) -> Result<Module> {
//     let mut cursor = node.walk();
//     let mut children = Vec::new();
//     for i in 0..node.named_child_count() {
//         children.push(node.named_child(i).unwrap());
//     }
//     let mut index = 0;
//     let mut is_exported = false;
//     if let Ok(text) = children[index].utf8_text(source.as_bytes()) {
//         if text == "export" {
//             is_exported = true;
//             index += 1;
//         }
//     }
//     // Expect "module"
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("module")
//     {
//         bail!("Expected 'module' keyword");
//     }
//     index += 1;
//     // Module name (an identifier)
//     let module_name_node = children
//         .get(index)
//         .ok_or_else(|| anyhow!("Missing module name"))?;
//     let module_name = module_name_node.utf8_text(source.as_bytes())?.to_string();
//     let name = Rc::new(Identifier { name: module_name });
//     index += 1;
//     // Optional generic parameters – we assume they are delimited by “<” and “>”
//     let mut generic_parameters = None;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("<")
//     {
//         index += 1; // skip "<"
//         let mut params = Vec::new();
//         while index < children.len() {
//             let text = children[index].utf8_text(source.as_bytes())?;
//             if text == ">" {
//                 index += 1;
//                 break;
//             }
//             if text != "," {
//                 params.push(Rc::new(Identifier {
//                     name: text.to_string(),
//                 }));
//             }
//             index += 1;
//         }
//         generic_parameters = Some(params);
//     }
//     // Expect "{" then module body then "}"
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("{")
//     {
//         bail!("Expected '{{' in module definition");
//     }
//     index += 1;
//     let mut body_nodes = Vec::new();
//     while index < children.len() {
//         let text = children[index].utf8_text(source.as_bytes())?;
//         if text == "}" {
//             break;
//         }
//         let child_node = children[index];
//         let compact_node = build_compact_node(&child_node, source)?;
//         body_nodes.push(compact_node);
//         index += 1;
//     }
//     Ok(Module {
//         is_exported,
//         name,
//         generic_parameters,
//         nodes: body_nodes,
//     })
// }

// /// Parse a circuit definition node.
// /// Grammar: cdefn → export^opt pure^opt "circuit" function-name gparams^opt "(" parg* ")" ":" type block
// fn build_circuit(node: &Node, source: &str) -> Result<Circuit> {
//     let mut cursor = node.walk();
//     let mut children = Vec::new();
//     for i in 0..node.named_child_count() {
//         children.push(node.named_child(i).unwrap());
//     }
//     let mut index = 0;
//     let mut is_exported = false;
//     let mut is_pure = false;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("export")
//     {
//         is_exported = true;
//         index += 1;
//     }
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("pure")
//     {
//         is_pure = true;
//         index += 1;
//     }
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("circuit")
//     {
//         bail!("Expected 'circuit' keyword");
//     }
//     index += 1;
//     // Function name (an identifier)
//     let func_name_node = children
//         .get(index)
//         .ok_or_else(|| anyhow!("Missing function name"))?;
//     let func_name = func_name_node.utf8_text(source.as_bytes())?.to_string();
//     let name = Rc::new(Identifier { name: func_name });
//     index += 1;
//     // Optional generic parameters (if present)
//     let mut generic_parameters = None;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("<")
//     {
//         index += 1;
//         let mut params = Vec::new();
//         while index < children.len() {
//             let text = children[index].utf8_text(source.as_bytes())?;
//             if text == ">" {
//                 index += 1;
//                 break;
//             }
//             if text != "," {
//                 params.push(Rc::new(Identifier {
//                     name: text.to_string(),
//                 }));
//             }
//             index += 1;
//         }
//         generic_parameters = Some(params);
//     }
//     // Expect "(" then parameters then ")"
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("(")
//     {
//         bail!("Expected '(' in circuit definition");
//     }
//     index += 1;
//     let mut arguments = Vec::new();
//     // For simplicity, assume each parameter is a simple “id : type” without extra punctuation.
//     while index < children.len() {
//         let token = children[index].utf8_text(source.as_bytes())?;
//         if token == ")" {
//             index += 1;
//             break;
//         }
//         // Parameter pattern (an identifier)
//         let pattern = Rc::new(Pattern::Identifier(Rc::new(Identifier {
//             name: token.to_string(),
//         })));
//         index += 1;
//         // Expect ":"
//         if children
//             .get(index)
//             .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//             != Some(":")
//         {
//             bail!("Expected ':' in circuit parameter");
//         }
//         index += 1;
//         // Type (for simplicity, a single identifier token)
//         let type_token = children
//             .get(index)
//             .ok_or_else(|| anyhow!("Missing type in circuit parameter"))?
//             .utf8_text(source.as_bytes())?
//             .to_string();
//         let ty = Type::Ref(Rc::new(Ref {
//             name: Rc::new(Identifier { name: type_token }),
//             generic_parameters: None,
//         }));
//         index += 1;
//         arguments.push(Rc::new(Argument { pattern, ty }));
//         // Skip an optional comma.
//         if index < children.len()
//             && children
//                 .get(index)
//                 .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//                 == Some(",")
//         {
//             index += 1;
//         }
//     }
//     // Expect ":" then return type.
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some(":")
//     {
//         bail!("Expected ':' before circuit return type");
//     }
//     index += 1;
//     let ret_type_token = children
//         .get(index)
//         .ok_or_else(|| anyhow!("Missing circuit return type"))?
//         .utf8_text(source.as_bytes())?
//         .to_string();
//     let ret_type = Type::Ref(Rc::new(Ref {
//         name: Rc::new(Identifier {
//             name: ret_type_token,
//         }),
//         generic_parameters: None,
//     }));
//     index += 1;
//     // Expect block start ("{") – here we create an empty block for simplicity.
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("{")
//     {
//         bail!("Expected '{{' for circuit block");
//     }
//     let body = Some(Rc::new(Block { statements: vec![] }));
//     Ok(Circuit {
//         name,
//         arguments,
//         generic_parameters,
//         is_exported,
//         is_pure,
//         ty: ret_type,
//         body,
//     })
// }

// /// Parse a struct definition node.
// /// Grammar: struct → export^opt "struct" struct-name gparams^opt "{" field* "}" ";"^opt
// fn build_structure(node: &Node, source: &str) -> Result<Structure> {
//     let mut cursor = node.walk();
//     let mut children = Vec::new();
//     for i in 0..node.named_child_count() {
//         children.push(node.named_child(i).unwrap());
//     }
//     let mut index = 0;
//     let mut is_exported = false;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("export")
//     {
//         is_exported = true;
//         index += 1;
//     }
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("struct")
//     {
//         bail!("Expected 'struct' keyword");
//     }
//     index += 1;
//     let struct_name_node = children
//         .get(index)
//         .ok_or_else(|| anyhow!("Missing struct name"))?;
//     let struct_name = struct_name_node.utf8_text(source.as_bytes())?.to_string();
//     let name = Rc::new(Identifier { name: struct_name });
//     index += 1;
//     // Optional generic parameters.
//     let mut generic_parameters = None;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("<")
//     {
//         index += 1;
//         let mut params = Vec::new();
//         while index < children.len() {
//             let token = children[index].utf8_text(source.as_bytes())?;
//             if token == ">" {
//                 index += 1;
//                 break;
//             }
//             if token != "," {
//                 params.push(Rc::new(Identifier {
//                     name: token.to_string(),
//                 }));
//             }
//             index += 1;
//         }
//         generic_parameters = Some(params);
//     }
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("{")
//     {
//         bail!("Expected '{{' in struct definition");
//     }
//     index += 1;
//     let mut fields = Vec::new();
//     // For each field, expect: id ":" type ";"
//     while index < children.len() {
//         let token = children[index].utf8_text(source.as_bytes())?;
//         if token == "}" {
//             break;
//         }
//         // Field name.
//         let field_name = token.to_string();
//         let field_name_rc = Rc::new(Identifier { name: field_name });
//         index += 1;
//         if children
//             .get(index)
//             .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//             != Some(":")
//         {
//             bail!("Expected ':' in struct field");
//         }
//         index += 1;
//         let type_token = children
//             .get(index)
//             .ok_or_else(|| anyhow!("Missing type in struct field"))?
//             .utf8_text(source.as_bytes())?
//             .to_string();
//         let ty = Type::Ref(Rc::new(Ref {
//             name: Rc::new(Identifier { name: type_token }),
//             generic_parameters: None,
//         }));
//         index += 1;
//         // Skip an optional ";".
//         if index < children.len()
//             && children
//                 .get(index)
//                 .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//                 == Some(";")
//         {
//             index += 1;
//         }
//         fields.push(Rc::new(Field {
//             name: field_name_rc,
//             ty,
//         }));
//     }
//     Ok(Structure {
//         is_exported,
//         name,
//         generic_parameters,
//         fields,
//     })
// }

// /// Parse an enum definition node.
// /// Grammar: enumdef → export^opt "enum" enum-name "{" id ("," id)* (",")? "}" ";"^opt
// fn build_enum(node: &Node, source: &str) -> Result<AstEnum> {
//     let mut cursor = node.walk();
//     let mut children = Vec::new();
//     for i in 0..node.named_child_count() {
//         children.push(node.named_child(i).unwrap());
//     }
//     let mut index = 0;
//     let mut is_exported = false;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         == Some("export")
//     {
//         is_exported = true;
//         index += 1;
//     }
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("enum")
//     {
//         bail!("Expected 'enum' keyword");
//     }
//     index += 1;
//     let enum_name_node = children
//         .get(index)
//         .ok_or_else(|| anyhow!("Missing enum name"))?;
//     let enum_name = enum_name_node.utf8_text(source.as_bytes())?.to_string();
//     let name = Rc::new(Identifier { name: enum_name });
//     index += 1;
//     if children
//         .get(index)
//         .and_then(|n| n.utf8_text(source.as_bytes()).ok())
//         != Some("{")
//     {
//         bail!("Expected '{{' in enum definition");
//     }
//     index += 1;
//     let mut options = Vec::new();
//     while index < children.len() {
//         let token = children[index].utf8_text(source.as_bytes())?;
//         if token == "}" {
//             break;
//         }
//         if token != "," {
//             options.push(Rc::new(Identifier {
//                 name: token.to_string(),
//             }));
//         }
//         index += 1;
//     }
//     Ok(AstEnum {
//         is_exported,
//         name,
//         options,
//     })
// }

fn build_identifier(node: &Node, source: &str) -> Result<Rc<Identifier>> {
    let text = node.utf8_text(source.as_bytes())?.to_string();
    Ok(Rc::new(Identifier {
        id: node_id(),
        location: location(node),
        name: text,
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
//         export ledger game_state: GAME_STATE;              // tracks the current game state according to the game state machine
// export ledger shot_attempt: Coord;                 // coordinate of the opponent's shot to be validated during the player's turn
// export ledger last_shot_result: Maybe<ShotResult>; // validated shot result
// export sealed ledger reward_coin_color: Bytes<32>; // identifier of the coins used for rewards
// export ledger reward: QualifiedCoinInfo;           // reference to the funds locked in the contract

// // Player 1 public state
// export ledger p1: Maybe<Bytes<32>>;                     // hash of player 1 secret, used to uniquely identify the player
// export ledger p1_public_key: Maybe<ZswapCoinPublicKey>; // public key of player 1, where to send the reward
// export ledger p1_ship_positions_hash: Bytes<32>;        // hash of player's board layout, to ensure that it is not changed
// export ledger p1_ship_state_hash: Bytes<32>;            // hash of player's ships current state, to ensure it is not changed between turns
// export ledger p1_hit_counter: Counter;                        // counter of hits on player's ships to determine the winner

// // Player 2 public state
// export ledger p2: Maybe<Bytes<32>>;
// export ledger p2_public_key: Maybe<ZswapCoinPublicKey>;
// export ledger p2_ship_positions_hash: Bytes<32>;
// export ledger p2_ship_state_hash: Bytes<32>;
// export ledger p2_hit_counter: Counter;
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
                        assert_eq!(rt.name.name, "GAME_STATE");
                    },
                    _ => panic!("Expected a reference type"),
                }
            }
            _ => panic!("Expected a ledger declaration"),
        }
    }

}
