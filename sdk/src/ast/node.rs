#![warn(clippy::pedantic)]

use std::{any::Any, rc::Rc};

use super::expression::Expression;
#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
    #[must_use]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
pub enum NodeKind {
    SameScopeNode(SameScopeNode),
    NewScope(Rc<dyn Node>),
}

pub trait NodeSymbolNode: Node + SymbolNode + Any {}

impl<T> NodeSymbolNode for T where T: Node + SymbolNode + Any {}

impl<'a> From<&'a Rc<dyn NodeSymbolNode>> for &'a dyn Node {
    fn from(node: &'a Rc<dyn NodeSymbolNode>) -> Self {
        node as &'a dyn Node
    }
}

impl Node for Rc<dyn NodeSymbolNode> {
    fn children(&self) -> Vec<Rc<NodeKind>> {
        match self.as_any().downcast_ref::<SameScopeNode>() {
            Some(SameScopeNode::Composite(comp_node)) => comp_node.children(),
            _ => vec![],
        }
    }
}

impl dyn NodeSymbolNode {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub enum SameScopeNode {
    Symbol(Rc<dyn NodeSymbolNode>),
    Composite(Rc<dyn Node>),
}

impl From<Rc<dyn Node>> for NodeKind {
    fn from(node: Rc<dyn Node>) -> Self {
        NodeKind::NewScope(node)
    }
}

pub trait Node: Any + std::fmt::Debug {
    fn children(&self) -> Vec<Rc<NodeKind>>;
}

impl dyn Node {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait SymbolNode {
    fn name(&self) -> String;
    fn type_expr(&self) -> Option<&Expression> {
        None
    }
}

#[macro_export]
macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        $enum_vis:vis enum $name:ident {
            $(
                $(#[$arm_attr:meta])*
                $(@$conv:ident)? $arm:ident $( ( $($tuple:tt)* ) )? $( { $($struct:tt)* } )? ,
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
        $enum_vis enum $name {
            $(
                $(#[$arm_attr])*
                $arm $( ( $($tuple)* ) )? $( { $($struct)* } )? ,
            )*
        }

        impl $name {
            #[must_use]
            pub fn location(&self) -> $crate::ast::node::Location {
                match self {
                    $(
                        $name::$arm(_a) => {
                            ast_enum!(@location _a, $( $conv )?)
                        }
                    )*
                }
            }
        }

        impl From<&$name> for $crate::ast::node::NodeKind {
            fn from(n: &$name) -> Self {
                match n {
                    $(
                        $name::$arm(a) => {
                            ast_enum!(@convert a, $( $conv )?)
                        }
                    )*
                }
            }
        }
    };

    (@location $inner:ident, raw) => {
        $inner.location()
    };

    (@location $inner:ident, symbol) => {
        $inner.location.clone()
    };

    (@location $inner:ident, scope) => {
        $inner.location.clone()
    };

    (@location $inner:ident, block) => {
        $inner.location()
    };

    (@location $inner:ident, skip_location) => {
        $crate::ast::node::Location::default()
    };

    (@location $inner:ident, ) => {
        $inner.location.clone()
    };

    (@convert $inner:ident, raw) => {
        $inner.into()
    };

    (@convert $inner:ident, symbol) => {
        $crate::ast::node::NodeKind::SameScopeNode($crate::ast::node::SameScopeNode::Symbol($inner.clone()))
    };

    (@convert $inner:ident, scope) => {
        $crate::ast::node::NodeKind::NewScope($inner.clone())
    };

    (@convert $inner:ident, skip_location) => {
        $crate::ast::node::NodeKind::SameScopeNode($crate::ast::node::SameScopeNode::Composite($inner.clone()))
    };

    (@convert $inner:ident, ) => {
        $crate::ast::node::NodeKind::SameScopeNode($crate::ast::node::SameScopeNode::Composite($inner.clone()))
    };

}

#[macro_export]
macro_rules! ast_node {
    (
        $(#[$outer:meta])*
        $struct_vis:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
        $struct_vis struct $name {
            pub id: u128,
            pub location: $crate::ast::node::Location,
            $(
                $(#[$field_attr])*
                $field_vis $field_name : $field_ty,
            )*
        }
    };
}

#[macro_export]
macro_rules! ast_nodes {
    (
        $(
            $(#[$outer:meta])*
            $struct_vis:vis struct $name:ident { $($fields:tt)* }
        )+
    ) => {
        $(
            $crate::ast_node! {
                $(#[$outer])*
                $struct_vis struct $name { $($fields)* }
            }
        )+
    };
}
