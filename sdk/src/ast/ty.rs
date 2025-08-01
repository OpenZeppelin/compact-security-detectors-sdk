use super::{
    declaration::GArgument,
    expression::Identifier,
    literal::{Bool, Nat, Str},
    node::{Node, NodeKind},
};
use crate::{ast_enum, ast_nodes, ast_nodes_impl};
use std::{fmt::Display, rc::Rc};

ast_enum! {
    pub enum Type {
        Nat(Rc<TypeNat>),
        Boolean(Rc<TypeBool>),
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

ast_enum! {
    pub enum VectorSize {
        Nat(Rc<Nat>),
        Ref(Rc<Identifier>),
    }
}

impl Type {
    #[must_use]
    pub fn matches(&self, ty: &Type) -> bool {
        matches!(
            (self, ty),
            (Type::Nat(_), Type::Nat(_))
                | (Type::Boolean(_), Type::Boolean(_))
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

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Nat(_) => write!(f, "nat"),
            Type::Boolean(_) => write!(f, "bool"),
            Type::String(_) => write!(f, "string"),
            Type::Field(_) => write!(f, "field"),
            Type::Uint(_) => write!(f, "uint"),
            Type::Vector(_) => write!(f, "vector"),
            Type::Opaque(_) => write!(f, "opaque"),
            Type::Bytes(_) => write!(f, "bytes"),
            Type::Ref(_) => write!(f, "ref"),
            Type::Sum(_) => write!(f, "sum"),
        }
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
        pub size: VectorSize,
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

ast_nodes_impl! {
    impl Node for TypeNat {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            vec![]
        }
    }
    impl Node for TypeBool {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            vec![]
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

impl TypeBool {
    #[must_use]
    pub fn new(bool: &Rc<Bool>) -> Self {
        Self {
            id: bool.id,
            location: bool.location.clone(),
        }
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

impl Ref {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.name.clone()
    }
}

impl Vector {
    #[must_use]
    pub fn size_nat(&self) -> Option<u64> {
        match &self.size {
            VectorSize::Nat(nat) => Some(nat.value),
            VectorSize::Ref(_) => None,
        }
    }
}
