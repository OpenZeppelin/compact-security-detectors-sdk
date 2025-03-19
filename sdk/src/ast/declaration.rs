#![warn(clippy::pedantic)]
use std::rc::Rc;

use crate::{ast::expression::Expression, ast_enum, ast_nodes};

use super::{
    definition::{Circuit, Definition},
    expression::Identifier,
    literal::Nat,
    node::{Node, NodeKind, SymbolNode},
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
        @symbol PatternArgument(Rc<PatternArgument>),
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
    #[must_use]
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
    #[must_use]
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

impl SymbolNode for PatternArgument {
    fn id(&self) -> u128 {
        match self.pattern {
            Pattern::Identifier(ref id) => id.id,
            _ => 0,
        }
    }

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

impl PatternArgument {
    #[must_use = "Use this function to get the name of the pattern argument. If the pattern is an identifier, it will return the name of the identifier."]
    pub fn name(&self) -> Option<String> {
        match &self.pattern {
            Pattern::Identifier(id) => Some(id.name.clone()),
            _ => None,
        }
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
            .map(|pattern| Rc::new(NodeKind::from(pattern)))
            .collect()
    }
}
