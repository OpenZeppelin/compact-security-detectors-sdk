#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast_enum, ast_nodes};

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
        @skip_location Tuple(Rc<Pattern>),
        Struct(Rc<StructPatternItem>),
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
        pub pattern: Rc<Pattern>,
        pub ty: Type,
    }

    pub struct StructPatternItem {
        pub name: Rc<Identifier>,
        pub pattern: Option<Rc<Pattern>>,
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
        vec![Rc::new(NodeKind::from(&*self.pattern))]
    }
}

impl Node for StructPatternItem {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        if let Some(ref pattern) = self.pattern {
            vec![Rc::new(NodeKind::from(&**pattern))]
        } else {
            vec![]
        }
    }
}
