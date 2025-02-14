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
macro_rules! ast_node {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident {
            $($body:tt)*
        }
    ) => {
        $(#[$outer])*
        #[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub id: u32,
            pub location: $crate::ast::node::Location,
            $($body)*
        }
    };
}

#[macro_export]
macro_rules! ast_nodes {
    (
        $(
            $(#[$outer:meta])*
            $vis:vis struct $name:ident { $($body:tt)* }
        )+
    ) => {
        $(
            $crate::ast_node! {
                $(#[$outer])*
                $vis struct $name { $($body)* }
            }
        )+
    };
}

ast_node! {
    pub struct Identifier {}
}
