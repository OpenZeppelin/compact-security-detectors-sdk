use std::{rc::Rc, vec};

use crate::{ast::statement::Statement, ast_enum, ast_nodes, ast_nodes_impl};

use super::{
    declaration::{Argument, Declaration, GArgument, PatternArgument},
    expression::{Expression, Identifier},
    node::{Node, NodeKind},
    program::CompactNode,
    statement::Block,
    ty::{Ref, Type},
};

ast_enum! {
    pub enum Definition {
        @scope Module(Rc<Module>),
        @scope Circuit(Rc<Circuit>),
        @scope Structure(Rc<Structure>),
        @scope Enum(Rc<Enum>),
    }
}

ast_nodes! {
    pub struct Module {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub nodes: Vec<CompactNode>,
    }

    pub struct Circuit {
        pub name: Rc<Identifier>,
        pub arguments: Vec<Rc<PatternArgument>>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub is_exported: bool,
        pub is_pure: bool,
        pub ty: Type,
        pub body: Option<Rc<Block>>,
    }

    pub struct Structure {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub fields: Vec<Rc<Argument>>,
    }

    pub struct Enum {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub options: Vec<Rc<Identifier>>,
    }
}

ast_nodes_impl! {
    impl Node for Module {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let name = Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())));
            let generic_parameters: Vec<Rc<NodeKind>> = self
                .generic_parameters
                .iter()
                .flatten()
                .map(|arg| Rc::new(NodeKind::from(&Expression::Identifier(arg.clone())))
                )
                .collect();
            let nodes: Vec<Rc<NodeKind>> = self.nodes.iter().map(|node| Rc::new(NodeKind::from(node))).collect();
            vec![name]
                .into_iter()
                .chain(generic_parameters)
                .chain(nodes)
                .collect()
        }
    }
    impl Node for Circuit {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let name = Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())));
            let arguments: Vec<Rc<NodeKind>> = self
                .arguments
                .iter()
                .map(|arg| Rc::new(NodeKind::from(&Declaration::PatternArgument(arg.clone()))))
                .collect();
            let ty = Rc::new(NodeKind::from(&self.ty));
            let body = if let Some(body) = &self.body {
                vec![Rc::new(NodeKind::from(&Statement::Block(body.clone())))]
            } else {
                vec![]
            };
            vec![name]
                .into_iter()
                .chain(arguments)
                .chain(vec![ty])
                .chain(body)
                // .rev()
                .collect()
        }
    }
    impl Node for Structure {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let name = Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())));
            let generic_parameters: Vec<Rc<NodeKind>> = self
                .generic_parameters
                .iter()
                .flatten()
                .map(|arg| Rc::new(NodeKind::from(&Expression::Identifier(arg.clone())))
                )
                .collect();
            let fields: Vec<Rc<NodeKind>> = self
                .fields
                .iter()
                .map(|arg| Rc::new(NodeKind::from(&Declaration::Argument(arg.clone())))
                )
                .collect();
            vec![name]
                .into_iter()
                .chain(generic_parameters)
                .chain(fields)
                .collect()
        }
    }
    impl Node for Enum {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let name = Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())));
            let options: Vec<Rc<NodeKind>> = self
                .options
                .iter()
                .map(|arg| Rc::new(NodeKind::from(&Expression::Identifier(arg.clone())))
                )
                .collect();
            vec![name]
                .into_iter()
                .chain(options)
                .collect()
        }
    }
}

impl Module {
    #[must_use = "This method returns the name of the module"]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl Structure {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }

    #[must_use]
    pub fn ty(&self) -> Type {
        Type::Ref(Rc::new(Ref {
            name: self.name.clone(),
            generic_parameters: self.generic_parameters.as_ref().map(|params| {
                params
                    .iter()
                    .map(|ident| {
                        GArgument::Type(Type::Ref(Rc::new(Ref {
                            name: ident.clone(),
                            generic_parameters: None,
                            id: u32::MAX,
                            location: ident.location.clone(),
                        })))
                    })
                    .collect()
            }),
            id: u32::MAX,
            location: self.name.location.clone(),
        }))
    }
}

impl Enum {
    #[must_use = "This method returns the name of the enum"]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }

    #[must_use = "This method returns the type of the enum"]
    pub fn ty(&self) -> Type {
        Type::Ref(Rc::new(Ref {
            name: self.name.clone(),
            generic_parameters: None,
            id: u32::MAX,
            location: self.name.location.clone(),
        }))
    }
}

impl Circuit {
    #[must_use = "This method returns the name of the circuit"]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }

    #[must_use = "This method to check if the circuit is external"]
    pub fn is_external(&self) -> bool {
        self.body.is_none()
    }

    #[must_use]
    pub fn inline_function_calls(&self) -> Vec<Statement> {
        if let Some(body) = &self.body {
            let mut inlined_statements = Vec::new();
            for stmt in &body.statements {
                match stmt {
                    Statement::Expression(Expression::FunctionCall(func_call)) => {
                        if let Some(circuit) = &func_call.reference {
                            if circuit.body.is_some() {
                                inlined_statements.extend(circuit.inline_function_calls());
                            }
                        }
                    }
                    _ => {
                        inlined_statements.push(stmt.clone());
                    }
                }
            }
            inlined_statements
        } else {
            vec![]
        }
    }
}
