use super::{
    declaration::GArgument,
    expression::Identifier,
    literal::{Bool, Nat, Str},
    node::{Node, NodeKind},
};
use crate::{ast_enum, ast_nodes};
use std::rc::Rc;

ast_enum! {
    pub enum Type {
        Nat(Rc<TypeNat>),
        Bool(Rc<TypeBool>),
        String(Rc<TypeString>),
        Field(Rc<TypeField>),
        Uint(Rc<Uint>),
        Vector(Rc<Vector>),
        Opaque(Rc<Opaque>),
        Bytes(Rc<Bytes>),
        Ref(Rc<Ref>),
        Sum(Rc<Sum>),
    }
}

impl Type {
    #[must_use]
    pub fn matches(&self, ty: &Type) -> bool {
        matches!(
            (self, ty),
            (Type::Nat(_), Type::Nat(_))
                | (Type::Bool(_), Type::Bool(_))
                | (Type::String(_), Type::String(_))
                | (Type::Field(_), Type::Field(_))
                | (Type::Uint(_), Type::Uint(_))
                | (Type::Vector(_), Type::Vector(_))
                | (Type::Opaque(_), Type::Opaque(_))
                | (Type::Bytes(_), Type::Bytes(_))
                | (Type::Ref(_), Type::Ref(_))
                | (Type::Sum(_), Type::Sum(_))
        )
    }
}

ast_nodes! {
    #[derive(Default)]
    pub struct TypeNat{}
    #[derive(Default)]
    pub struct TypeBool{}
    #[derive(Default)]
    pub struct TypeString{}
    #[derive(Default)]
    pub struct TypeField {}
    pub struct Uint {
        pub start: Rc<Nat>,
        pub end: Option<Rc<Nat>>,
    }
    pub struct Vector {
        pub size: Rc<Nat>,
        pub ty: Type,
    }
    pub struct Opaque {
        pub value: Rc<Str>,
    }
    pub struct Bytes {
        pub size: Rc<Nat>,
    }
    pub struct Ref {
        pub name: Rc<Identifier>,
        pub generic_parameters: Option<Vec<GArgument>>,
    }
    pub struct Sum {
        pub types: Vec<Type>,
    }
}

impl TypeNat {
    #[must_use]
    pub fn new(nat: &Rc<Nat>) -> Self {
        Self {
            id: nat.id,
            location: nat.location.clone(),
        }
    }
}

impl Node for TypeNat {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl TypeBool {
    #[must_use]
    pub fn new(bool: &Rc<Bool>) -> Self {
        Self {
            id: bool.id,
            location: bool.location.clone(),
        }
    }
}

impl Node for TypeBool {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl TypeString {
    #[must_use]
    pub fn new(str: &Rc<Str>) -> Self {
        Self {
            id: str.id,
            location: str.location.clone(),
        }
    }
}

impl Node for TypeString {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for TypeField {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Uint {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Vector {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Opaque {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Bytes {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Ref {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}

impl Node for Sum {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        vec![]
    }
}
