#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast::expression::Expression, ast_enum, ast_nodes, ast_nodes_impl};

use super::{
    definition::{Circuit, Definition},
    expression::Identifier,
    literal::Nat,
    node::{Node, NodeKind, SymbolNode},
    program::Program,
    statement::{Block, Statement},
    ty::Type,
};

ast_enum! {
    pub enum Declaration {
        Argument(Rc<Argument>),
        Import(Rc<Import>),
        Include(Rc<Include>),
        Export(Rc<Export>),
        @symbol Witness(Rc<Witness>),
        @symbol Ledger(Rc<Ledger>),
        @scope Constructor(Rc<Constructor>),
        @scope Contract(Rc<Contract>),
        @raw Definition(Definition), //TODO: why is it here?
        @symbol PatternArgument(Rc<PatternArgument>),
        StructPatternField(Rc<StructPatternField>),
    }
}

ast_enum! {
    pub enum Pattern {
        @symbol Identifier(Rc<Identifier>),
        Tuple(Rc<TuplePattern>),
        Struct(Rc<StructPattern>),
    }
}

ast_enum! {
    pub enum StructArgument {
        @raw StructPatternField(Expression),
        @raw DestructExpression(Expression),
    }
}

ast_enum! {
    pub enum GArgument {
        @raw Type(Type),
        Nat(Rc<Nat>),
    }
}

ast_nodes! {
    pub struct Import {
        pub value: Rc<Identifier>,
        pub generic_parameters: Option<Vec<GArgument>>,
        pub prefix: Option<Rc<Identifier>>,
        pub reference: Option<Rc<Program>>,
    }

    pub struct Include {
        pub path: String,
    }

    pub struct Export {
        pub values: Vec<Rc<Identifier>>,
    }

    pub struct Witness {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<Rc<Identifier>>>,
        pub arguments: Vec<Rc<Argument>>,
        pub ty: Type,
    }

    pub struct Ledger {
        pub is_exported: bool,
        pub is_sealed: bool,
        pub name: Rc<Identifier>,
        pub ty: Type,
    }

    pub struct Constructor {
        pub arguments: Vec<Rc<PatternArgument>>,
        pub body: Rc<Block>
    }

    pub struct Contract {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub circuits: Vec<Rc<Circuit>>,
    }

    pub struct Argument {
        pub name: Rc<Identifier>,
        pub ty: Type,
    }

    pub struct PatternArgument {
        pub pattern: Pattern,
        pub ty: Type,
    }

    pub struct StructPatternField {
        pub name: Rc<Identifier>,
        pub pattern: Pattern,
    }

    pub struct StructPattern {
        pub fields: Vec<Rc<StructPatternField>>,
    }

    pub struct TuplePattern {
        pub patterns: Vec<Pattern>,
    }

}

ast_nodes_impl! {
    impl Node for Import {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = vec![Rc::new(NodeKind::from(&Expression::Identifier(self.value.clone())))];
            if let Some(prefix) = &self.prefix {
                res.push(Rc::new(NodeKind::from(&Expression::Identifier(prefix.clone()))));
            }
            if let Some(generic_parameters) = &self.generic_parameters {
                for param in generic_parameters {
                    res.push(Rc::new(NodeKind::from(param)));
                }
            }
            res
        }
    }
    impl Node for Include {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            vec![]
        }
    }
    impl Node for Export {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            self.values
                .iter()
                .map(|id| Rc::new(NodeKind::from(&Expression::Identifier(id.clone())))
                )
                .collect()
        }
    }
    impl Node for Witness {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = vec![Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())))];
            if let Some(generic_parameters) = &self.generic_parameters {
                for param in generic_parameters {
                    res.push(Rc::new(NodeKind::from(&Expression::Identifier(param.clone()))));
                }
            }
            for arg in &self.arguments {
                res.push(Rc::new(NodeKind::from(&Declaration::Argument(arg.clone()))));
            }
            res.push(Rc::new(NodeKind::from(&self.ty)));
            res
        }
    }
    impl Node for Ledger {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = vec![Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())))];
            res.push(Rc::new(NodeKind::from(&self.ty)));
            res
        }
    }
    impl Node for Constructor {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let arguments: Vec<Rc<NodeKind>> = self
                .arguments
                .iter()
                .map(|arg| Rc::new(NodeKind::from(&Declaration::PatternArgument(arg.clone()))))
                .collect();
            let body = vec![Rc::new(NodeKind::from(&Statement::Block(self.body.clone())))];
            arguments.into_iter().chain(body).collect()
        }
    }
    impl Node for Contract {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = vec![Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone())))];
            for circuit in &self.circuits {
                res.push(Rc::new(NodeKind::from(&Definition::Circuit(circuit.clone()))));
            }
            res
        }
    }
    impl Node for Argument {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            vec![
                Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone()))),
                Rc::new(NodeKind::from(&self.ty)),
            ]
        }
    }
    impl Node for PatternArgument {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            vec![
                Rc::new(NodeKind::from(&self.pattern)),
                Rc::new(NodeKind::from(&self.ty)),
            ]
        }
    }
    impl Node for StructPatternField {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            vec![
                Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone()))),
                Rc::new(NodeKind::from(&self.pattern)),
            ]
        }
    }
    impl Node for StructPattern {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            let mut res = Vec::new();
            for field in &self.fields {
                res.extend(field.children());
            }
            res
        }
    }
    impl Node for TuplePattern {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            self.patterns
                .iter()
                .map(|pattern| Rc::new(NodeKind::from(pattern)))
                .collect()
        }
    }
}

impl Contract {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl Import {
    #[must_use = "Use this function to get the name of the import with trailing quotes and terminator removed."]
    pub fn name(&self) -> String {
        let name = self.value.name.clone();
        if name.starts_with('"') {
            name[1..name.len() - 1].to_string()
        } else {
            name
        }
    }
}

impl Ledger {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl SymbolNode for Ledger {
    fn name(&self) -> String {
        self.name.name.clone()
    }

    fn type_expr(&self) -> Option<Expression> {
        Some(Expression::TypeExpression(self.ty.clone()))
    }
}

impl SymbolNode for PatternArgument {
    fn name(&self) -> String {
        match self.pattern {
            Pattern::Identifier(ref id) => id.name.clone(),
            _ => String::from("_"),
        }
    }

    fn type_expr(&self) -> Option<Expression> {
        Some(Expression::TypeExpression(self.ty.clone()))
    }
}

impl Argument {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl PatternArgument {
    #[must_use = "Use this function to get the name of the pattern argument. If the pattern is an identifier, it will return the name of the identifier."]
    pub fn name(&self) -> Option<String> {
        match &self.pattern {
            Pattern::Identifier(id) => Some(id.name.clone()),
            _ => None,
        }
    }
}

impl Witness {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl SymbolNode for Witness {
    fn name(&self) -> String {
        self.name.name.clone()
    }

    fn type_expr(&self) -> Option<Expression> {
        Some(Expression::TypeExpression(self.ty.clone()))
    }
}
