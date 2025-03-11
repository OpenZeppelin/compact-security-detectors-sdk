#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast::expression::Expression, ast_enum, ast_nodes};

use super::{
    definition::{Circuit, Definition},
    expression::Identifier,
    node::{Node, NodeKind},
    statement::Block,
    ty::Type,
};

ast_enum! {
    pub enum Declaration {
        Import(Rc<Import>),
        Export(Rc<Export>),
        External(Rc<External>),
        Witness(Rc<Witness>),
        Ledger(Rc<Ledger>),
        @scope Constructor(Rc<Constructor>),
        @scope Contract(Rc<Contract>),
        @scope Struct(Rc<Struct>),
        @scope Enum(Rc<Enum>),
        @raw Definition(Definition),
    }
}

ast_enum! {
    pub enum Pattern {
        Identifier(Rc<Identifier>),
        Tuple(Rc<TuplePattern>),
        Struct(Rc<StructPattern>),
    }
}

impl Node for Pattern {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        match self {
            Pattern::Identifier(i) => i.children(),
            Pattern::Tuple(pattern) => pattern.children(),
            Pattern::Struct(pattern) => pattern.children(),
        }
    }
}

ast_nodes! {
    pub struct Import {
        pub value: Rc<Identifier>,
    }

    pub struct Export {
        pub values: Vec<Rc<Identifier>>,
    }

    pub struct External {}

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
        pub arguments: Vec<Rc<Argument>>,
        pub body: Rc<Block>
    }

    pub struct Contract {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub circuits: Vec<Rc<Circuit>>,
    }

    pub struct Struct {}

    pub struct Enum {}

    pub struct Argument {
        pub name: Rc<Identifier>,
        pub ty: Type,
    }

    pub struct PArgument {
        pub pattern: Rc<Pattern>,
        pub ty: Type,
    }

    pub struct StructPattern {
        pub fields: Vec<Rc<StructPatternField>>,
    }

    pub struct StructPatternField {
        pub name: Rc<Identifier>,
        pub pattern: Rc<Pattern>,
    }

    pub struct TuplePattern {
        pub patterns: Vec<Rc<Pattern>>,
    }

}

impl Node for Import {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Import {
    pub fn name(&self) -> String {
        self.value.name.clone()
    }
}

impl Node for Export {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
impl Node for External {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
impl Node for Witness {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Ledger {
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl Node for Ledger {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
impl Node for Constructor {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
impl Node for Contract {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
impl Node for Struct {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
impl Node for Enum {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
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

impl Node for PArgument {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&*self.pattern)),
            Rc::new(NodeKind::from(&self.ty)),
        ]
    }
}

impl Node for StructPatternField {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![
            Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone()))),
            Rc::new(NodeKind::from(&*self.pattern)),
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
            .map(|pattern| Rc::new(NodeKind::from(&**pattern)))
            .collect()
    }
}
