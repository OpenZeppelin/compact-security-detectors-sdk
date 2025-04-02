#![warn(clippy::pedantic)]

use std::{any::Any, cmp::Reverse, rc::Rc};

use super::expression::Expression;
#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub offset_start: u32,
    pub offset_end: u32,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub source: String,
}

impl Location {
    #[must_use]
    pub fn new(
        offset_start: u32,
        offset_end: u32,
        start_line: u32,
        start_column: u32,
        end_line: u32,
        end_column: u32,
        source: String,
    ) -> Self {
        Self {
            offset_start,
            offset_end,
            start_line,
            start_column,
            end_line,
            end_column,
            source,
        }
    }
}

#[derive(Debug)]
pub enum NodeKind {
    SameScopeNode(SameScopeNode),
    NewScope(Rc<dyn Node>),
}

impl NodeKind {
    #[must_use]
    pub fn id(&self) -> u32 {
        match self {
            NodeKind::SameScopeNode(node) => node.id(),
            NodeKind::NewScope(node) => node.id(),
        }
    }
}

pub trait NodeSymbolNode: Node + SymbolNode + Any {}

impl<T> NodeSymbolNode for T where T: Node + SymbolNode + Any {}

impl<'a> From<&'a Rc<dyn NodeSymbolNode>> for &'a dyn Node {
    fn from(node: &'a Rc<dyn NodeSymbolNode>) -> Self {
        node as &'a dyn Node
    }
}

impl Node for Rc<dyn NodeSymbolNode> {
    fn id(&self) -> u32 {
        match self.as_any().downcast_ref::<SameScopeNode>() {
            Some(SameScopeNode::Composite(comp_node)) => comp_node.id(),
            _ => match self.as_any().downcast_ref::<NodeKind>() {
                Some(NodeKind::NewScope(node)) => node.id(),
                Some(NodeKind::SameScopeNode(node)) => node.id(),
                _ => 0,
            },
        }
    }

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

impl SameScopeNode {
    fn id(&self) -> u32 {
        match self {
            SameScopeNode::Symbol(sym_node) => sym_node.id(),
            SameScopeNode::Composite(comp_node) => comp_node.id(),
        }
    }
}

impl From<Rc<dyn Node>> for NodeKind {
    fn from(node: Rc<dyn Node>) -> Self {
        NodeKind::NewScope(node)
    }
}

pub trait Node: Any + std::fmt::Debug {
    fn id(&self) -> u32;
    fn node_type_name(&self) -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or_default()
            .to_string()
    }
    fn children(&self) -> Vec<Rc<NodeKind>>;
    fn sorted_children(&self) -> Vec<Rc<NodeKind>> {
        let mut children = self.children();
        children.sort_by_key(|c| Reverse(c.id()));
        children
    }
}

impl dyn Node {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait SymbolNode {
    fn name(&self) -> String;
    fn type_expr(&self) -> Option<Expression>;
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
            pub fn id(&self) -> u32 {
                match self {
                    $(
                        $name::$arm(_a) => {
                            ast_enum!(@id _a, $( $conv )?)
                        }
                    )*
                }
            }

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

    (@id $inner:ident, raw) => {
        $inner.id()
    };

    (@id $inner:ident, symbol) => {
        $inner.id
    };

    (@id $inner:ident, scope) => {
        $inner.id
    };

    (@id $inner:ident, block) => {
        $inner.id
    };

    (@id $inner:ident, skip_location) => {
        $inner.id
    };

    (@id $inner:ident, ) => {
        $inner.id
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
            pub id: u32,
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

#[macro_export]
macro_rules! ast_node_impl {
    (
        $(#[$outer:meta])*
        impl Node for $name:ident {
            $(
                $(#[$method_attr:meta])*
                fn $method:ident ( $($args:tt)* ) -> $ret:ty $body:block
            )*
        }
    ) => {
        $(#[$outer])*
        impl Node for $name {
            fn id(&self) -> u32 {
                self.id
            }

            $(
                $(#[$method_attr])*
                fn $method ( $($args)* ) -> $ret $body
            )*
        }
    };
}

#[macro_export]
macro_rules! ast_nodes_impl {
    (
        $(
            $(#[$outer:meta])*
            impl Node for $name:ident {
                $(
                    $(#[$method_attr:meta])*
                    fn $method:ident ( $($args:tt)* ) -> $ret:ty $body:block
                )*
            }
        )+
    ) => {
        $(
            $crate::ast_node_impl! {
                $(#[$outer])*
                impl Node for $name {
                    $(
                        $(#[$method_attr])*
                        fn $method ( $($args)* ) -> $ret $body
                    )*
                }
            }
        )+
    };
}
