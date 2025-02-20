#![warn(clippy::pedantic)]
#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub source_code: String,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
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

        impl From<&$name> for $crate::passes::NodeKind {
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

    (@convert $inner:ident, raw) => {
        $inner.into()
    };

    (@convert $inner:ident, symbol) => {
        $crate::passes::NodeKind::SameScopeNode($crate::passes::SameScopeNode::Symbol($inner.clone()))
    };

    (@convert $inner:ident, scope) => {
        $crate::passes::NodeKind::NewScope($inner.clone())
    };

    (@convert $inner:ident, ) => {
        $crate::passes::NodeKind::SameScopeNode($crate::passes::SameScopeNode::Composite($inner.clone()))
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
