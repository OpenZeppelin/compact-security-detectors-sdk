#![warn(clippy::pedantic)]
pub mod ast;
pub mod codebase;
mod type_inference;

// fn parse_content(fname: &str, source_code: &str) -> anyhow::Result<codebase::SourceCodeFile> {
//     let compact_language = tree_sitter_compact::language();
//     let mut parser = tree_sitter::Parser::new();
//     parser
//         .set_language(&compact_language)
//         .expect("Error loading Inference grammar");
//     let tree = parser.parse(source_code, None).unwrap();
//     let code = source_code.as_bytes();
//     let root_node = tree.root_node();
// }

// fn build_ast(root: &tree_sitter::Node) -> anyhow::Result<Program> {}
