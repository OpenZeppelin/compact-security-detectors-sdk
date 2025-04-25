/// The `Codebase` module provides functionality for managing and interacting with a codebase
/// represented as Abstract Syntax Trees (ASTs). It includes mechanisms for parsing source code,
/// building symbol tables, linking imports, resolving function calls, and managing the state of
/// the codebase (open or sealed).
///
/// # Overview
///
/// The `Codebase` struct is a generic container that operates in two states:
/// - `OpenState`: Allows modifications such as adding files and nodes.
/// - `SealedState`: Prevents further modifications and provides read-only access to the codebase.
///
/// The module also defines traits (`CodebaseOpen` and `CodebaseSealed`) to enforce state-specific
/// behavior.
///
/// # Key Components
///
/// - `SourceCodeFile`: Represents a source code file and its associated AST.
/// - `NodesStorage`: Manages the storage of AST nodes.
/// - `SymbolTable`: Represents a symbol table for resolving identifiers and types.
///
/// # Public API
///
/// ## Codebase<OpenState>
/// - `new`: Creates a new `Codebase` in the open state.
/// - `add_file`: Parses and adds a source code file to the codebase.
/// - `add_node`: Adds a node to the codebase's storage.
/// - `seal`: Seals the codebase, preventing further modifications and building symbol tables.
///
/// ## Codebase<SealedState>
/// - `files`: Returns an iterator over all source code files in the codebase.
/// - `get_symbol_type_by_id`: Retrieves the type of a symbol by its ID.
/// - `list_assert_nodes`: Lists all `Assert` statement nodes in the codebase.
/// - `list_for_statement_nodes`: Lists all `For` statement nodes in the codebase.
/// - `list_exported_circuits_from_program`: Lists all exported circuits in a program.
/// - `list_non_exported_circuits_from_program`: Lists all non-exported circuits in a program.
/// - `get_parent_container`: Retrieves the parent container (e.g., module or circuit) of a node.
///
/// # Internal Functionality
///
/// - **Linking Imports**: The `link_imports` function resolves import declarations and propagates
///   types from imported files.
/// - **Resolving Function Calls**: The `link_function_calls` function resolves references for
///   function call nodes, including those imported from other files.
/// - **Building Symbol Tables**: The `build_symbol_table_for_file_level_types` function constructs
///   symbol tables for file-level types such as circuits, structures, and enums.
///
/// # Error Handling
///
/// - Functions like `add_file` and `seal` return `Result` to handle errors during parsing or
///   symbol table construction.
/// - Panics are used in cases where critical errors occur, such as failing to load the grammar.
use crate::{
    ast::{
        builder::build_ast,
        declaration::Declaration,
        definition::{Circuit, Definition, Module},
        expression::Expression,
        function::Function,
        node::NodeKind,
        node_type::NodeType,
        program::Program,
        statement::{Assert, For, Statement},
        ty::Type,
    },
    storage::NodesStorage,
    symbol_table::{build_symbol_table, SymbolTable},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData, rc::Rc};

#[allow(dead_code)]
trait CodebaseOpen {}
#[allow(dead_code)]
trait CodebaseSealed {}

/// Represents the open state of the codebase, allowing modifications.
pub struct OpenState;
impl CodebaseOpen for OpenState {}

/// Represents the sealed state of the codebase, preventing modifications.
pub struct SealedState;
impl CodebaseSealed for SealedState {}

/// `SoourceCodeFile` represents a source code file and its associated AST.
///
/// # Fields
///
/// - `file_path`: a path to the source code file.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceCodeFile {
    pub file_path: String,
    pub(crate) ast: Rc<Program>,
}

/// `Codebase` represents a collection of source code files and their associated ASTs with API access functions
///
/// # Fields
///
/// - `storage`: a storage for AST nodes.
/// - `files`: a vector of `SourceCodeFile`
/// - `symbol_tables`: a map <file path: `Rc<SymbolTable>>`
/// - `_state`: A phantom data marker for the state

#[derive(Serialize, Deserialize, Default)]
pub struct Codebase<S> {
    pub(crate) storage: NodesStorage,
    pub(crate) files: Vec<SourceCodeFile>,
    pub(crate) symbol_tables: HashMap<String, Rc<SymbolTable>>,
    pub(crate) _state: PhantomData<S>,
}

impl Codebase<OpenState> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: NodesStorage::default(),
            files: Vec::new(),
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
            file_path: fname.to_string(),
            ast,
        };
        self.files.push(source_code_file);
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
        let mut local_symbol_tables = HashMap::new();
        for file in &self.files {
            let local_symtab = Codebase::build_symbol_table_for_file_level_types(&file.ast.clone());
            local_symbol_tables.insert(file.file_path.clone(), local_symtab);
        }
        let mut symbol_tables = HashMap::new();
        for file in &self.files {
            let mut parent_symtab = None;
            for node in &self.storage.nodes {
                if let NodeType::Declaration(Declaration::Import(import)) = node {
                    if let Some(node_file) = self.find_node_file(node.id()) {
                        if node_file.file_path == *file.file_path {
                            if let Some(imported) = &import.reference {
                                if let Some(imported_fname) =
                                    self.files.iter().find(|f| f.ast.id == imported.id)
                                {
                                    if let Some(imported_symtab) =
                                        local_symbol_tables.get(&imported_fname.file_path)
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
            let effective_parent =
                parent_symtab.or_else(|| local_symbol_tables.get(&file.file_path).cloned());
            let symbol_table =
                build_symbol_table(Rc::new(NodeKind::from(&file.ast)), effective_parent)?;
            symbol_tables.insert(file.file_path.clone(), symbol_table);
        }
        self.link_function_calls();
        self.storage.seal();
        Ok(Codebase {
            storage: self.storage,
            files: self.files,
            symbol_tables,
            _state: PhantomData,
        })
    }

    fn link_imports(&mut self) {
        for node in &mut self.storage.nodes {
            if let NodeType::Declaration(Declaration::Import(ref mut import)) = node {
                let import_mut = Rc::make_mut(import);
                if let Some(file) = self.files.iter().find(|f| f.file_path == import_mut.name()) {
                    import_mut.reference = Some(file.ast.clone());
                    if let Some(imported_symtab) = self.symbol_tables.get(&file.file_path) {
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

    fn link_function_calls(&mut self) {
        let function_calls: Vec<(u32, String)> = self
            .storage
            .nodes
            .iter()
            .filter_map(|node| {
                if let NodeType::Expression(Expression::FunctionCall(function_call)) = node {
                    if let Expression::Function(Function::Named(function_name)) =
                        &function_call.function
                    {
                        Some((function_call.id, function_name.name().to_owned()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for (call_id, function_name) in function_calls {
            if let Some(file) = self.find_node_file(call_id) {
                let mut circuit_opt = file
                    .ast
                    .circuits()
                    .iter()
                    .find(|c| c.name() == function_name)
                    .cloned();
                if circuit_opt.is_none() {
                    let import_nodes: Vec<_> = self
                        .storage
                        .nodes
                        .iter()
                        .filter_map(|node| {
                            if let NodeType::Declaration(Declaration::Import(import)) = node {
                                // Only consider imports that belong to the same file.
                                if let Some(import_file) = self.find_node_file(node.id()) {
                                    if import_file.file_path == file.file_path {
                                        return Some(import);
                                    }
                                }
                            }
                            None
                        })
                        .collect();
                    for import in import_nodes {
                        if let Some(imported_program) = &import.reference {
                            if let Some(circuit) = imported_program
                                .circuits()
                                .iter()
                                .find(|c| c.name() == function_name)
                            {
                                circuit_opt = Some(circuit.clone());
                                break;
                            }
                        }
                    }
                }
                if let Some(circuit) = circuit_opt {
                    if let Some(NodeType::Expression(Expression::FunctionCall(
                        ref mut function_call_node,
                    ))) = self.storage.find_node_mut(call_id)
                    {
                        let function_call_mut = Rc::make_mut(function_call_node);
                        function_call_mut.reference = Some(circuit);
                    }
                }
            }
        }
    }

    pub(crate) fn build_symbol_table_for_file_level_types(
        program: &Rc<Program>,
    ) -> Rc<SymbolTable> {
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
        self.files.iter().cloned()
    }

    #[must_use = "Use this function to get a type for a symbol (Identifier)"]
    pub fn get_symbol_type_by_id(&self, id: u32) -> Option<Type> {
        if let Some(file) = self.find_node_file(id) {
            self.symbol_tables
                .get(&file.file_path)
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

    #[must_use = "Use this function to get a list of all exported circuits in the file"]
    pub fn list_exported_circuits_from_program(&self, program: &Rc<Program>) -> Vec<Rc<Circuit>> {
        self.list_exported_circuits(program.id)
    }

    #[must_use = "Use this function to get a list of all exported circuits from the module"]
    pub fn list_exported_circuits_from_module(&self, module: &Rc<Module>) -> Vec<Rc<Circuit>> {
        self.list_exported_circuits(module.id)
    }

    fn list_exported_circuits(&self, id: u32) -> Vec<Rc<Circuit>> {
        self.get_children_cmp(id, |node| {
            if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                circuit.is_exported
            } else {
                false
            }
        })
        .into_iter()
        .filter_map(|node| {
            if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                Some(circuit)
            } else {
                None
            }
        })
        .collect()
    }

    #[must_use = "Use this function to get a list of all non-exported circuits in the file"]
    pub fn list_non_exported_circuits_from_program(
        &self,
        program: &Rc<Program>,
    ) -> Vec<Rc<Circuit>> {
        self.list_non_exported_circuits(program.id)
    }

    #[must_use = "Use this function to get a list of all non-exported circuits from the module"]
    pub fn list_non_exported_circuits_from_module(&self, module: &Rc<Module>) -> Vec<Rc<Circuit>> {
        self.list_non_exported_circuits(module.id)
    }

    fn list_non_exported_circuits(&self, id: u32) -> Vec<Rc<Circuit>> {
        self.get_children_cmp(id, |node| {
            if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                !circuit.is_exported
            } else {
                false
            }
        })
        .into_iter()
        .filter_map(|node| {
            if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                Some(circuit)
            } else {
                None
            }
        })
        .collect()
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
        if let Some(file) = self.files.iter().find(|file| file.ast.id == id) {
            Some(file.clone())
        } else {
            let mut node_id = id;
            while let Some(parent) = self.storage.find_parent_node(node_id) {
                if parent == 0 {
                    if let Some(file) = self.storage.find_node(node_id) {
                        match file {
                            NodeType::Program(f) => {
                                if let Some(sf) =
                                    self.files.iter().find(|file| Rc::ptr_eq(&file.ast, &f))
                                {
                                    return Some(SourceCodeFile {
                                        file_path: sf.file_path.clone(),
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
        let mut codebase = Codebase::<OpenState>::new();
        let source_a = r"
            export pure circuit unknown_ship_def(): ShipDef {
              return ShipDef {
                ship: SHIP.unknown,
                ship_cell: Coord { 0, 0 },
                ship_v: false
              };
            }
        ";
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

    #[test]
    fn test_function_call_single_file_reference_resolution() -> anyhow::Result<()> {
        let mut codebase = Codebase::<OpenState>::new();
        let source_a = r"
             export pure circuit unknown_ship_def(): ShipDef {
               return ShipDef {
                 ship: SHIP.unknown,
                 ship_cell: Coord { 0, 0 },
                 ship_v: false
               };
             }

            pure circuit calculate_ship_def(shot_attempt: Coord, ship_state: ShipState, updated_ship_state: ShipState, ships: Ships, player: Bytes<32>): ShotResult {
               return unknown_ship_def();
            }
         ";
        codebase.add_file("./a.compact", source_a);
        let sealed = codebase.seal()?;

        let unknown_ship_def_circuit_node = sealed
            .list_nodes_cmp(|node| {
                if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                    if circuit.name() == "unknown_ship_def" {
                        return Some(circuit.clone());
                    }
                }
                None
            })
            .next()
            .expect("unknown_ship_def node not found");
        let function_call_node = sealed
            .list_nodes_cmp(|node| {
                if let NodeType::Expression(Expression::FunctionCall(func_call)) = node {
                    return Some(func_call.clone());
                }
                None
            })
            .next()
            .expect("Function call node not found");
        assert_eq!(
            function_call_node.reference.as_ref().unwrap().id,
            unknown_ship_def_circuit_node.id,
            "Function call reference should be set to the correct circuit id, expected: {}, found: {}",
            unknown_ship_def_circuit_node.id, function_call_node.reference.as_ref().unwrap().id
        );
        Ok(())
    }

    #[test]
    fn test_function_call_multi_file_reference_resolution() -> anyhow::Result<()> {
        let mut codebase = Codebase::<OpenState>::new();
        let source_a = r"
            export pure circuit unknown_ship_def(): ShipDef {
              return ShipDef {
                ship: SHIP.unknown,
                ship_cell: Coord { 0, 0 },
                ship_v: false
              };
            }
        ";
        let source_b = r#"
            import "./a.compact";
            pure circuit calculate_ship_def(shot_attempt: Coord, ship_state: ShipState, updated_ship_state: ShipState, ships: Ships, player: Bytes<32>): ShotResult {
                return unknown_ship_def();
            }
        "#;
        codebase.add_file("./a.compact", source_a);
        codebase.add_file("./b.compact", source_b);
        let sealed = codebase.seal()?;

        let unknown_ship_def_circuit_node = sealed
            .list_nodes_cmp(|node| {
                if let NodeType::Definition(Definition::Circuit(circuit)) = node {
                    if circuit.name() == "unknown_ship_def" {
                        return Some(circuit.clone());
                    }
                }
                None
            })
            .next()
            .expect("unknown_ship_def node not found");
        let function_call_node = sealed
            .list_nodes_cmp(|node| {
                if let NodeType::Expression(Expression::FunctionCall(func_call)) = node {
                    return Some(func_call.clone());
                }
                None
            })
            .next()
            .expect("Function call node not found");
        assert_eq!(
            function_call_node.reference.as_ref().unwrap().id,
            unknown_ship_def_circuit_node.id,
            "Function call reference should be set to the correct circuit id, expected: {}, found: {}",
            unknown_ship_def_circuit_node.id, function_call_node.reference.as_ref().unwrap().id
        );
        Ok(())
    }
}
