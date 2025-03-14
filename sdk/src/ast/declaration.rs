#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast::expression::Expression, ast_enum, ast_nodes};

use super::{
    definition::{Circuit, Definition},
    expression::Identifier,
    literal::Nat,
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
        // @scope Struct(Rc<Struct>),
        // @scope Enum(Rc<Enum>),
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

ast_enum! {
    pub enum StructArgument {
        // StructField(Rc<StructField>),
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
        pub arguments: Vec<Rc<PatternArgument>>,
        pub body: Rc<Block>
    }

    pub struct Contract {
        pub is_exported: bool,
        pub name: Rc<Identifier>,
        pub circuits: Vec<Rc<Circuit>>,
    }

    // pub struct StructField {
    //     pub name: Rc<Identifier>,
    //     pub expression: Expression,
    // }

    // pub struct Struct {}

    // pub struct Enum {}

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
// impl Node for Struct {
//     fn children(&self) -> Vec<Rc<NodeKind>> {
//         vec![]
//     }
// }
// impl Node for Enum {
//     fn children(&self) -> Vec<Rc<NodeKind>> {
//         vec![]
//     }
// }

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

// impl Node for StructField {
//     fn children(&self) -> Vec<Rc<NodeKind>> {
//         vec![
//             Rc::new(NodeKind::from(&Expression::Identifier(self.name.clone()))),
//             Rc::new(NodeKind::from(&self.expression)),
//         ]
//     }
// }

impl Node for TuplePattern {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        self.patterns
            .iter()
            .map(|pattern| Rc::new(NodeKind::from(&*pattern)))
            .collect()
    }
}
