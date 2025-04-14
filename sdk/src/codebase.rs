#![warn(clippy::pedantic)]
use crate::{
    ast::{
        builder::build_ast,
        declaration::Declaration,
        definition::Definition,
        node::NodeKind,
        node_type::NodeType,
        program::Program,
        statement::{Assert, For, Statement},
        ty::Type,
    },
    passes::{build_symbol_table, SymbolTable},
    storage::NodesStorage,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData, rc::Rc};

#[allow(dead_code)]
trait CodebaseOpen {}
#[allow(dead_code)]
trait CodebaseSealed {}

pub struct OpenState;
impl CodebaseOpen for OpenState {}

pub struct SealedState;
impl CodebaseSealed for SealedState {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceCodeFile {
    pub fname: String,
    pub(crate) ast: Rc<Program>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Codebase<S> {
    pub(crate) storage: NodesStorage,
    pub(crate) fname_ast_map: HashMap<String, SourceCodeFile>,
    pub(crate) symbol_tables: HashMap<String, Rc<SymbolTable>>,
    pub(crate) _state: PhantomData<S>,
}

impl Codebase<OpenState> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: NodesStorage::default(),
            fname_ast_map: HashMap::new(),
            symbol_tables: HashMap::new(),
            _state: PhantomData,
        }
    }

    /// Parses the content of a source code file and returns a `SourceCodeFile` object.
    ///
    /// # Errors
    ///
    /// This function will return an error if the AST cannot be built from the source code.
    ///
    /// # Panics
    ///
    /// This function will panic if there is an error loading the Inference grammar.
    pub fn add_file(&mut self, fname: &str, source_code: &str) {
        let compact_language = tree_sitter_compact::LANGUAGE.into();
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&compact_language)
            .expect("Error loading Inference grammar");
        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();
        let ast = build_ast(self, &root_node, source_code).unwrap();
        let source_code_file = SourceCodeFile {
            fname: fname.to_string(),
            ast,
        };
        self.fname_ast_map
            .insert(source_code_file.fname.clone(), source_code_file);
    }

    pub(crate) fn add_node(&mut self, node: NodeType, parent: u32) {
        self.storage.add_node(node, parent);
    }

    /// Seals the codebase, preventing further modifications.
    ///
    /// # Errors
    ///
    /// This function will return an error if building the symbol table fails.
    ///
    /// # Panics
    ///
    /// This function will panic if the symbol table for a file path is not found.
    pub fn seal(mut self) -> Result<Codebase<SealedState>> {
        self.link_imports();
        // First, build a mapping of each file to its local file-level symbol table.
        let mut local_symbol_tables = HashMap::new();
        for (file_path, source_code_file) in &self.fname_ast_map {
            let local_symtab =
                Codebase::build_symbol_table_for_file_level_types(&source_code_file.ast.clone());
            local_symbol_tables.insert(file_path.clone(), local_symtab);
        }
        let mut symbol_tables = HashMap::new();
        // Now, build the full symbol table for each file.
        for (file_path, source_code_file) in &self.fname_ast_map {
            // Look for an import declaration belonging to this file that has been linked.
            let mut parent_symtab = None;
            for node in &self.storage.nodes {
                if let NodeType::Declaration(Declaration::Import(import)) = node {
                    if let Some(node_file) = self.find_node_file(node.id()) {
                        if node_file.fname == *file_path {
                            // Use the import reference (an id of the imported file) to look up the imported file's local symbol table.
                            if let Some(imported_id) = import.reference {
                                if let Some((imported_fname, _)) = self
                                    .fname_ast_map
                                    .iter()
                                    .find(|(_, file)| file.ast.id == imported_id)
                                {
                                    if let Some(imported_symtab) =
                                        local_symbol_tables.get(imported_fname)
                                    {
                                        parent_symtab = Some(imported_symtab.clone());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // If no parent was found through an import, use the file's own local symbol table.
            let effective_parent =
                parent_symtab.or_else(|| local_symbol_tables.get(file_path).cloned());
            let symbol_table = build_symbol_table(
                Rc::new(NodeKind::from(&source_code_file.ast)),
                effective_parent,
            )?;
            // println!("{}\n{}", &file_path, &symbol_table);
            symbol_tables.insert(file_path.clone(), symbol_table);
        }
        self.storage.seal();
        Ok(Codebase {
            storage: self.storage,
            fname_ast_map: self.fname_ast_map,
            symbol_tables,
            _state: PhantomData,
        })
    }

    fn link_imports(&mut self) {
        for node in &mut self.storage.nodes {
            if let NodeType::Declaration(Declaration::Import(ref mut import)) = node {
                let import_mut = Rc::make_mut(import);
                if let Some(file) = self.fname_ast_map.get(import_mut.name().as_str()) {
                    import_mut.reference = Some(file.ast.id);

                    // Propagate types from the imported file's symbol table
                    if let Some(imported_symtab) = self.symbol_tables.get(&file.fname) {
                        let symbols_to_add: Vec<_> = imported_symtab
                            .symbols
                            .borrow()
                            .iter()
                            .filter_map(|(name, ty)| {
                                ty.as_ref().map(|ty| (name.clone(), ty.clone()))
                            })
                            .collect();

                        for (name, ty) in symbols_to_add {
                            self.symbol_tables
                                .entry(import_mut.name().to_string())
                                .or_insert_with(|| Rc::new(SymbolTable::new(None)))
                                .upsert(0, name, Some(ty));
                        }
                    }
                }
            }
        }
    }

    fn build_symbol_table_for_file_level_types(program: &Rc<Program>) -> Rc<SymbolTable> {
        let rc_symbol_table = Rc::new(SymbolTable::new(None));
        for definition in &program.definitions {
            match definition {
                Definition::Module(_) => {}
                Definition::Circuit(circuit) => {
                    rc_symbol_table.upsert(circuit.id, circuit.name(), Some(circuit.ty.clone()));
                }
                Definition::Structure(structure) => {
                    rc_symbol_table.upsert(structure.id, structure.name(), Some(structure.ty()));
                }
                Definition::Enum(e) => {
                    rc_symbol_table.upsert(e.id, e.name(), Some(e.ty()));
                }
            }
        }
        rc_symbol_table
    }
}

impl Codebase<SealedState> {
    pub fn files(&self) -> impl Iterator<Item = SourceCodeFile> + '_ {
        self.fname_ast_map.values().cloned()
    }

    #[must_use = "Use this function to get a type for a symbol (Identifier)"]
    pub fn get_symbol_type_by_id(&self, id: u32) -> Option<Type> {
        if let Some(file) = self.find_node_file(id) {
            self.symbol_tables
                .get(&file.fname)
                .and_then(|table| table.lookdown_by_id(id))
        } else {
            None
        }
    }

    pub fn list_assert_nodes(&self) -> impl Iterator<Item = Rc<Assert>> + '_ {
        self.list_nodes_cmp(|node| {
            if let NodeType::Statement(Statement::Assert(stmt)) = node {
                Some(stmt.clone())
            } else {
                None
            }
        })
    }

    pub fn list_for_statement_nodes(&self) -> impl Iterator<Item = Rc<For>> + '_ {
        self.list_nodes_cmp(|node| {
            if let NodeType::Statement(Statement::For(stmt)) = node {
                Some(stmt.clone())
            } else {
                None
            }
        })
    }

    #[must_use]
    pub fn get_parent_container(&self, id: u32) -> Option<NodeType> {
        let mut current_id = id;
        while let Some(route) = self.storage.find_parent_node(current_id) {
            current_id = route;
            if let Some(node) = self.storage.find_node(current_id) {
                if let NodeType::Definition(Definition::Circuit(_) | Definition::Module(_)) = node {
                    return self.storage.find_node(node.id());
                }
            }
        }
        None
    }

    pub fn get_children_cmp<F>(&self, id: u32, comparator: F) -> Vec<NodeType>
    where
        F: Fn(&NodeType) -> bool,
    {
        let mut result = Vec::new();
        let mut stack: Vec<NodeType> = Vec::new();

        if let Some(root_node) = self.storage.find_node(id) {
            stack.push(root_node.clone());
        }

        while let Some(current_node) = stack.pop() {
            if comparator(&current_node) {
                result.push(current_node.clone());
            }
            stack.extend(current_node.children());
        }

        result
    }

    fn list_nodes_cmp<'a, T, F>(&'a self, cast: F) -> impl Iterator<Item = T> + 'a
    where
        F: Fn(&NodeType) -> Option<T> + 'a,
        T: Clone + 'static,
    {
        self.storage.nodes.iter().filter_map(cast)
    }
}

impl<T> Codebase<T> {
    #[must_use = "Use this function to get a Node's source file"]
    pub fn find_node_file(&self, id: u32) -> Option<SourceCodeFile> {
        if let Some((_, file)) = self
            .fname_ast_map
            .iter()
            .find(|(_, file)| file.ast.id == id)
        {
            Some(file.clone())
        } else {
            let mut node_id = id;
            while let Some(parent) = self.storage.find_parent_node(node_id) {
                if parent == 0 {
                    if let Some(file) = self.storage.find_node(node_id) {
                        match file {
                            NodeType::Program(f) => {
                                if let Some((fname, _)) = self
                                    .fname_ast_map
                                    .iter()
                                    .find(|(_, file)| Rc::ptr_eq(&file.ast, &f))
                                {
                                    return Some(SourceCodeFile {
                                        fname: fname.clone(),
                                        ast: f.clone(),
                                    });
                                }
                            }
                            _ => return None,
                        }
                    }
                }
                node_id = parent;
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_reference_set_correctly() -> anyhow::Result<()> {
        let mut codebase = Codebase::<OpenState>::new();
        codebase.add_file("./a.compact", r#"import "./b.compact";"#);
        codebase.add_file("./b.compact", r#"import "./a.compact";"#);
        let codebase = codebase.seal()?;
        let imports: Vec<_> = codebase
            .list_nodes_cmp(|node| {
                if let NodeType::Declaration(Declaration::Import(import)) = node {
                    Some(import.clone())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(imports.len(), 2);
        for import in imports {
            assert!(
                import.reference.is_some(),
                "Import reference should be set for all import nodes"
            );
        }
        Ok(())
    }

    #[test]
    fn test_imported_function_types_resolved_correctly() -> anyhow::Result<()> {
        // Create a new open codebase.
        let mut codebase = Codebase::<OpenState>::new();
        // File A defines a function.
        let source_a = r"
            export pure circuit unknown_ship_def(): ShipDef {
              return ShipDef {
                ship: SHIP.unknown,
                ship_cell: Coord { 0, 0 },
                ship_v: false
              };
            }
        ";
        // File B imports file A and calls the function.
        let source_b = r#"
            import "./a.compact";
            pure circuit calculate_ship_def(shot_attempt: Coord, ship_state: ShipState, updated_ship_state: ShipState, ships: Ships, player: Bytes<32>): ShotResult {
                return unknown_ship_def();
            }
        "#;
        codebase.add_file("./a.compact", source_a);
        codebase.add_file("./b.compact", source_b);
        let sealed = codebase.seal()?;
        let unknown_ship_def_node_id = sealed
            .list_nodes_cmp(|node| {
                if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                    if circuit.name() == "unknown_ship_def" {
                        return Some(node.id());
                    }
                }
                None
            })
            .next()
            .expect("unknown_ship_def node not found");
        println!("unknown_ship_def_node_id: {unknown_ship_def_node_id}");
        let ship_def_type = sealed
            .get_symbol_type_by_id(unknown_ship_def_node_id)
            .unwrap_or_else(|| {
                panic!("Type for unknown_ship_def not found [{unknown_ship_def_node_id}]")
            });
        match ship_def_type {
            Type::Ref(ref ty) => {
                assert_eq!(ty.name(), "ShipDef");
            }
            _ => panic!("Expected a reference type for unknown_ship_def"),
        }
        Ok(())
    }
}
