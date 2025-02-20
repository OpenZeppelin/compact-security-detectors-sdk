#![feature(prelude_import)]
#![warn(clippy::pedantic)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod ast {
    //! Abstract Syntax Tree (AST) for the Compact language.
    pub mod declaration {
        #![warn(clippy::pedantic)]
        use std::rc::Rc;
        use crate::{ast_enum, ast_nodes};
        use super::definition::Definition;
        pub enum Declaration {
            Import(Rc<Import>),
            Export(Rc<Export>),
            External(Rc<External>),
            Witness(Rc<Witness>),
            Ledger(Rc<Ledger>),
            Ctor(Rc<Ctor>),
            Contract(Rc<Contract>),
            Struct(Rc<Struct>),
            Enum(Rc<Enum>),
            Definition(Rc<Definition>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Declaration {
            #[inline]
            fn clone(&self) -> Declaration {
                match self {
                    Declaration::Import(__self_0) => {
                        Declaration::Import(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Export(__self_0) => {
                        Declaration::Export(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::External(__self_0) => {
                        Declaration::External(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Witness(__self_0) => {
                        Declaration::Witness(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Ledger(__self_0) => {
                        Declaration::Ledger(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Ctor(__self_0) => {
                        Declaration::Ctor(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Contract(__self_0) => {
                        Declaration::Contract(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Struct(__self_0) => {
                        Declaration::Struct(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Enum(__self_0) => {
                        Declaration::Enum(::core::clone::Clone::clone(__self_0))
                    }
                    Declaration::Definition(__self_0) => {
                        Declaration::Definition(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Declaration {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Declaration {
            #[inline]
            fn eq(&self, other: &Declaration) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            Declaration::Import(__self_0),
                            Declaration::Import(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Declaration::Export(__self_0),
                            Declaration::Export(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Declaration::External(__self_0),
                            Declaration::External(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Declaration::Witness(__self_0),
                            Declaration::Witness(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Declaration::Ledger(__self_0),
                            Declaration::Ledger(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (Declaration::Ctor(__self_0), Declaration::Ctor(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Declaration::Contract(__self_0),
                            Declaration::Contract(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Declaration::Struct(__self_0),
                            Declaration::Struct(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (Declaration::Enum(__self_0), Declaration::Enum(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Declaration::Definition(__self_0),
                            Declaration::Definition(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Declaration {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rc<Import>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Export>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<External>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Witness>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Ledger>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Ctor>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Contract>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Struct>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Enum>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Definition>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Declaration {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Declaration::Import(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Import",
                            &__self_0,
                        )
                    }
                    Declaration::Export(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Export",
                            &__self_0,
                        )
                    }
                    Declaration::External(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "External",
                            &__self_0,
                        )
                    }
                    Declaration::Witness(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Witness",
                            &__self_0,
                        )
                    }
                    Declaration::Ledger(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ledger",
                            &__self_0,
                        )
                    }
                    Declaration::Ctor(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ctor",
                            &__self_0,
                        )
                    }
                    Declaration::Contract(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Contract",
                            &__self_0,
                        )
                    }
                    Declaration::Struct(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Struct",
                            &__self_0,
                        )
                    }
                    Declaration::Enum(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Enum",
                            &__self_0,
                        )
                    }
                    Declaration::Definition(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Definition",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Declaration {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Declaration::Import(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                0u32,
                                "Import",
                                __field0,
                            )
                        }
                        Declaration::Export(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                1u32,
                                "Export",
                                __field0,
                            )
                        }
                        Declaration::External(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                2u32,
                                "External",
                                __field0,
                            )
                        }
                        Declaration::Witness(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                3u32,
                                "Witness",
                                __field0,
                            )
                        }
                        Declaration::Ledger(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                4u32,
                                "Ledger",
                                __field0,
                            )
                        }
                        Declaration::Ctor(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                5u32,
                                "Ctor",
                                __field0,
                            )
                        }
                        Declaration::Contract(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                6u32,
                                "Contract",
                                __field0,
                            )
                        }
                        Declaration::Struct(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                7u32,
                                "Struct",
                                __field0,
                            )
                        }
                        Declaration::Enum(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                8u32,
                                "Enum",
                                __field0,
                            )
                        }
                        Declaration::Definition(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Declaration",
                                9u32,
                                "Definition",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Declaration {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 10",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Import" => _serde::__private::Ok(__Field::__field0),
                                "Export" => _serde::__private::Ok(__Field::__field1),
                                "External" => _serde::__private::Ok(__Field::__field2),
                                "Witness" => _serde::__private::Ok(__Field::__field3),
                                "Ledger" => _serde::__private::Ok(__Field::__field4),
                                "Ctor" => _serde::__private::Ok(__Field::__field5),
                                "Contract" => _serde::__private::Ok(__Field::__field6),
                                "Struct" => _serde::__private::Ok(__Field::__field7),
                                "Enum" => _serde::__private::Ok(__Field::__field8),
                                "Definition" => _serde::__private::Ok(__Field::__field9),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Import" => _serde::__private::Ok(__Field::__field0),
                                b"Export" => _serde::__private::Ok(__Field::__field1),
                                b"External" => _serde::__private::Ok(__Field::__field2),
                                b"Witness" => _serde::__private::Ok(__Field::__field3),
                                b"Ledger" => _serde::__private::Ok(__Field::__field4),
                                b"Ctor" => _serde::__private::Ok(__Field::__field5),
                                b"Contract" => _serde::__private::Ok(__Field::__field6),
                                b"Struct" => _serde::__private::Ok(__Field::__field7),
                                b"Enum" => _serde::__private::Ok(__Field::__field8),
                                b"Definition" => _serde::__private::Ok(__Field::__field9),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Declaration>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Declaration;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum Declaration",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Import>,
                                        >(__variant),
                                        Declaration::Import,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Export>,
                                        >(__variant),
                                        Declaration::Export,
                                    )
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<External>,
                                        >(__variant),
                                        Declaration::External,
                                    )
                                }
                                (__Field::__field3, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Witness>,
                                        >(__variant),
                                        Declaration::Witness,
                                    )
                                }
                                (__Field::__field4, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Ledger>,
                                        >(__variant),
                                        Declaration::Ledger,
                                    )
                                }
                                (__Field::__field5, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Ctor>,
                                        >(__variant),
                                        Declaration::Ctor,
                                    )
                                }
                                (__Field::__field6, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Contract>,
                                        >(__variant),
                                        Declaration::Contract,
                                    )
                                }
                                (__Field::__field7, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Struct>,
                                        >(__variant),
                                        Declaration::Struct,
                                    )
                                }
                                (__Field::__field8, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Enum>,
                                        >(__variant),
                                        Declaration::Enum,
                                    )
                                }
                                (__Field::__field9, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Definition>,
                                        >(__variant),
                                        Declaration::Definition,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Import",
                        "Export",
                        "External",
                        "Witness",
                        "Ledger",
                        "Ctor",
                        "Contract",
                        "Struct",
                        "Enum",
                        "Definition",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Declaration",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Declaration>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&Declaration> for crate::passes::NodeKind {
            fn from(n: &Declaration) -> Self {
                match n {
                    Declaration::Import(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Export(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::External(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Witness(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Ledger(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Ctor(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Contract(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Struct(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Enum(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Declaration::Definition(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                }
            }
        }
        pub struct Import {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Import {
            #[inline]
            fn clone(&self) -> Import {
                Import {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Import {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Import {
            #[inline]
            fn eq(&self, other: &Import) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Import {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Import {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Import",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Import {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Import",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Import {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Import>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Import;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Import",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Import with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Import with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Import {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Import {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Import",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Import>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Export {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Export {
            #[inline]
            fn clone(&self) -> Export {
                Export {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Export {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Export {
            #[inline]
            fn eq(&self, other: &Export) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Export {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Export {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Export",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Export {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Export",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Export {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Export>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Export;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Export",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Export with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Export with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Export {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Export {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Export",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Export>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct External {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for External {
            #[inline]
            fn clone(&self) -> External {
                External {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for External {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for External {
            #[inline]
            fn eq(&self, other: &External) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for External {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for External {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "External",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for External {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "External",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for External {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<External>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = External;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct External",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct External with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct External with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(External {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(External {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "External",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<External>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Witness {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Witness {
            #[inline]
            fn clone(&self) -> Witness {
                Witness {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Witness {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Witness {
            #[inline]
            fn eq(&self, other: &Witness) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Witness {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Witness {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Witness",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Witness {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Witness",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Witness {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Witness>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Witness;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Witness",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Witness with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Witness with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Witness {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Witness {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Witness",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Witness>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Ledger {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Ledger {
            #[inline]
            fn clone(&self) -> Ledger {
                Ledger {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Ledger {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Ledger {
            #[inline]
            fn eq(&self, other: &Ledger) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Ledger {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Ledger {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Ledger",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Ledger {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Ledger",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Ledger {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Ledger>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Ledger;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Ledger",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Ledger with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Ledger with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Ledger {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Ledger {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Ledger",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Ledger>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Ctor {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Ctor {
            #[inline]
            fn clone(&self) -> Ctor {
                Ctor {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Ctor {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Ctor {
            #[inline]
            fn eq(&self, other: &Ctor) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Ctor {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Ctor {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Ctor",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Ctor {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Ctor",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Ctor {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Ctor>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Ctor;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Ctor",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Ctor with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Ctor with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Ctor {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Ctor {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Ctor",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Ctor>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Contract {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Contract {
            #[inline]
            fn clone(&self) -> Contract {
                Contract {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Contract {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Contract {
            #[inline]
            fn eq(&self, other: &Contract) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Contract {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Contract {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Contract",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Contract {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Contract",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Contract {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Contract>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Contract;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Contract",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Contract with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Contract with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Contract {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Contract {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Contract",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Contract>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Struct {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Struct {
            #[inline]
            fn clone(&self) -> Struct {
                Struct {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Struct {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Struct {
            #[inline]
            fn eq(&self, other: &Struct) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Struct {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Struct {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Struct",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Struct {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Struct",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Struct {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Struct>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Struct;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Struct",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Struct with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Struct with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Struct {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Struct {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Struct",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Struct>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Enum {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Enum {
            #[inline]
            fn clone(&self) -> Enum {
                Enum {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Enum {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Enum {
            #[inline]
            fn eq(&self, other: &Enum) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Enum {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Enum {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Enum",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Enum {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Enum",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Enum {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Enum>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Enum;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Enum",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Enum with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Enum with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Enum {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Enum {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Enum",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Enum>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod definition {
        #![warn(clippy::pedantic)]
        use std::rc::Rc;
        use crate::{ast_enum, ast_nodes};
        pub enum Definition {
            Module(Rc<Module>),
            Circuit(Rc<Circuit>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Definition {
            #[inline]
            fn clone(&self) -> Definition {
                match self {
                    Definition::Module(__self_0) => {
                        Definition::Module(::core::clone::Clone::clone(__self_0))
                    }
                    Definition::Circuit(__self_0) => {
                        Definition::Circuit(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Definition {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Definition {
            #[inline]
            fn eq(&self, other: &Definition) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Definition::Module(__self_0), Definition::Module(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Definition::Circuit(__self_0),
                            Definition::Circuit(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Definition {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rc<Module>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Circuit>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Definition {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Definition::Module(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Module",
                            &__self_0,
                        )
                    }
                    Definition::Circuit(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Circuit",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Definition {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Definition::Module(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Definition",
                                0u32,
                                "Module",
                                __field0,
                            )
                        }
                        Definition::Circuit(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Definition",
                                1u32,
                                "Circuit",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Definition {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Module" => _serde::__private::Ok(__Field::__field0),
                                "Circuit" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Module" => _serde::__private::Ok(__Field::__field0),
                                b"Circuit" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Definition>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Definition;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum Definition",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Module>,
                                        >(__variant),
                                        Definition::Module,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Circuit>,
                                        >(__variant),
                                        Definition::Circuit,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Module", "Circuit"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Definition",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Definition>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&Definition> for crate::passes::NodeKind {
            fn from(n: &Definition) -> Self {
                match n {
                    Definition::Module(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Definition::Circuit(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                }
            }
        }
        pub struct Module {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Module {
            #[inline]
            fn clone(&self) -> Module {
                Module {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Module {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Module {
            #[inline]
            fn eq(&self, other: &Module) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Module {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Module {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Module",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Module {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Module",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Module {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Module>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Module;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Module",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Module with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Module with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Module {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Module {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Module",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Module>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Circuit {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Circuit {
            #[inline]
            fn clone(&self) -> Circuit {
                Circuit {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Circuit {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Circuit {
            #[inline]
            fn eq(&self, other: &Circuit) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Circuit {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Circuit {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Circuit",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Circuit {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Circuit",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Circuit {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Circuit>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Circuit;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Circuit",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Circuit with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Circuit with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Circuit {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Circuit {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Circuit",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Circuit>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod directive {
        #![warn(clippy::pedantic)]
        use std::rc::Rc;
        use crate::{ast_enum, ast_nodes};
        pub enum Directive {
            Pragma(Rc<Pragma>),
            Include(Rc<Include>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Directive {
            #[inline]
            fn clone(&self) -> Directive {
                match self {
                    Directive::Pragma(__self_0) => {
                        Directive::Pragma(::core::clone::Clone::clone(__self_0))
                    }
                    Directive::Include(__self_0) => {
                        Directive::Include(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Directive {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Directive {
            #[inline]
            fn eq(&self, other: &Directive) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Directive::Pragma(__self_0), Directive::Pragma(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Directive::Include(__self_0), Directive::Include(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Directive {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rc<Pragma>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Include>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Directive {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Directive::Pragma(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Pragma",
                            &__self_0,
                        )
                    }
                    Directive::Include(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Include",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Directive {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Directive::Pragma(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Directive",
                                0u32,
                                "Pragma",
                                __field0,
                            )
                        }
                        Directive::Include(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Directive",
                                1u32,
                                "Include",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Directive {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Pragma" => _serde::__private::Ok(__Field::__field0),
                                "Include" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Pragma" => _serde::__private::Ok(__Field::__field0),
                                b"Include" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Directive>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Directive;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum Directive",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Pragma>,
                                        >(__variant),
                                        Directive::Pragma,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Include>,
                                        >(__variant),
                                        Directive::Include,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Pragma", "Include"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Directive",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Directive>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&Directive> for crate::passes::NodeKind {
            fn from(n: &Directive) -> Self {
                match n {
                    Directive::Pragma(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Directive::Include(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                }
            }
        }
        pub struct Pragma {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Pragma {
            #[inline]
            fn clone(&self) -> Pragma {
                Pragma {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Pragma {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Pragma {
            #[inline]
            fn eq(&self, other: &Pragma) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Pragma {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Pragma {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Pragma",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Pragma {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Pragma",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Pragma {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Pragma>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Pragma;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Pragma",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Pragma with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Pragma with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Pragma {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Pragma {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Pragma",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Pragma>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Include {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Include {
            #[inline]
            fn clone(&self) -> Include {
                Include {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Include {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Include {
            #[inline]
            fn eq(&self, other: &Include) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Include {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Include {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Include",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Include {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Include",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Include {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Include>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Include;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Include",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Include with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Include with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Include {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Include {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Include",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Include>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod expression {
        use std::rc::Rc;
        use crate::{
            ast_enum, ast_nodes, passes::{Node, NodeKind, SameScopeNode, SymbolNode},
        };
        use super::literal::Literal;
        pub enum Expression {
            Conditional(Rc<Conditional>),
            Binary(Rc<Binary>),
            Cast(Rc<Cast>),
            IndexAccess(Rc<IndexAccess>),
            MemberAccess(Rc<MemberAccess>),
            FunctionCall(Rc<FunctionCall>),
            Literal(Literal),
            Identifier(Rc<Identifier>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Expression {
            #[inline]
            fn clone(&self) -> Expression {
                match self {
                    Expression::Conditional(__self_0) => {
                        Expression::Conditional(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::Binary(__self_0) => {
                        Expression::Binary(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::Cast(__self_0) => {
                        Expression::Cast(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::IndexAccess(__self_0) => {
                        Expression::IndexAccess(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::MemberAccess(__self_0) => {
                        Expression::MemberAccess(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::FunctionCall(__self_0) => {
                        Expression::FunctionCall(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::Literal(__self_0) => {
                        Expression::Literal(::core::clone::Clone::clone(__self_0))
                    }
                    Expression::Identifier(__self_0) => {
                        Expression::Identifier(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Expression {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Expression {
            #[inline]
            fn eq(&self, other: &Expression) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            Expression::Conditional(__self_0),
                            Expression::Conditional(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (Expression::Binary(__self_0), Expression::Binary(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Expression::Cast(__self_0), Expression::Cast(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            Expression::IndexAccess(__self_0),
                            Expression::IndexAccess(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Expression::MemberAccess(__self_0),
                            Expression::MemberAccess(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Expression::FunctionCall(__self_0),
                            Expression::FunctionCall(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Expression::Literal(__self_0),
                            Expression::Literal(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            Expression::Identifier(__self_0),
                            Expression::Identifier(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Expression {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rc<Conditional>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Binary>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Cast>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<IndexAccess>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<MemberAccess>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<FunctionCall>>;
                let _: ::core::cmp::AssertParamIsEq<Literal>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Identifier>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Expression {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Expression::Conditional(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Conditional",
                            &__self_0,
                        )
                    }
                    Expression::Binary(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Binary",
                            &__self_0,
                        )
                    }
                    Expression::Cast(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Cast",
                            &__self_0,
                        )
                    }
                    Expression::IndexAccess(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "IndexAccess",
                            &__self_0,
                        )
                    }
                    Expression::MemberAccess(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MemberAccess",
                            &__self_0,
                        )
                    }
                    Expression::FunctionCall(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "FunctionCall",
                            &__self_0,
                        )
                    }
                    Expression::Literal(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Literal",
                            &__self_0,
                        )
                    }
                    Expression::Identifier(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Identifier",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Expression {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Expression::Conditional(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                0u32,
                                "Conditional",
                                __field0,
                            )
                        }
                        Expression::Binary(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                1u32,
                                "Binary",
                                __field0,
                            )
                        }
                        Expression::Cast(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                2u32,
                                "Cast",
                                __field0,
                            )
                        }
                        Expression::IndexAccess(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                3u32,
                                "IndexAccess",
                                __field0,
                            )
                        }
                        Expression::MemberAccess(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                4u32,
                                "MemberAccess",
                                __field0,
                            )
                        }
                        Expression::FunctionCall(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                5u32,
                                "FunctionCall",
                                __field0,
                            )
                        }
                        Expression::Literal(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                6u32,
                                "Literal",
                                __field0,
                            )
                        }
                        Expression::Identifier(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Expression",
                                7u32,
                                "Identifier",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Expression {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 8",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Conditional" => _serde::__private::Ok(__Field::__field0),
                                "Binary" => _serde::__private::Ok(__Field::__field1),
                                "Cast" => _serde::__private::Ok(__Field::__field2),
                                "IndexAccess" => _serde::__private::Ok(__Field::__field3),
                                "MemberAccess" => _serde::__private::Ok(__Field::__field4),
                                "FunctionCall" => _serde::__private::Ok(__Field::__field5),
                                "Literal" => _serde::__private::Ok(__Field::__field6),
                                "Identifier" => _serde::__private::Ok(__Field::__field7),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Conditional" => _serde::__private::Ok(__Field::__field0),
                                b"Binary" => _serde::__private::Ok(__Field::__field1),
                                b"Cast" => _serde::__private::Ok(__Field::__field2),
                                b"IndexAccess" => _serde::__private::Ok(__Field::__field3),
                                b"MemberAccess" => _serde::__private::Ok(__Field::__field4),
                                b"FunctionCall" => _serde::__private::Ok(__Field::__field5),
                                b"Literal" => _serde::__private::Ok(__Field::__field6),
                                b"Identifier" => _serde::__private::Ok(__Field::__field7),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Expression>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Expression;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum Expression",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Conditional>,
                                        >(__variant),
                                        Expression::Conditional,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Binary>,
                                        >(__variant),
                                        Expression::Binary,
                                    )
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Cast>,
                                        >(__variant),
                                        Expression::Cast,
                                    )
                                }
                                (__Field::__field3, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<IndexAccess>,
                                        >(__variant),
                                        Expression::IndexAccess,
                                    )
                                }
                                (__Field::__field4, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<MemberAccess>,
                                        >(__variant),
                                        Expression::MemberAccess,
                                    )
                                }
                                (__Field::__field5, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<FunctionCall>,
                                        >(__variant),
                                        Expression::FunctionCall,
                                    )
                                }
                                (__Field::__field6, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Literal,
                                        >(__variant),
                                        Expression::Literal,
                                    )
                                }
                                (__Field::__field7, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Identifier>,
                                        >(__variant),
                                        Expression::Identifier,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Conditional",
                        "Binary",
                        "Cast",
                        "IndexAccess",
                        "MemberAccess",
                        "FunctionCall",
                        "Literal",
                        "Identifier",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Expression",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Expression>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&Expression> for crate::passes::NodeKind {
            fn from(n: &Expression) -> Self {
                match n {
                    Expression::Conditional(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Expression::Binary(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Expression::Cast(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Expression::IndexAccess(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Expression::MemberAccess(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Expression::FunctionCall(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Expression::Literal(a) => a.into(),
                    Expression::Identifier(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Symbol(a.clone()),
                        )
                    }
                }
            }
        }
        impl From<&NodeKind> for Expression {
            fn from(node: &NodeKind) -> Self {
                match node {
                    NodeKind::SameScopeNode(SameScopeNode::Composite(cond)) => {
                        if let Some(cond) = cond
                            .as_any()
                            .downcast_ref::<Rc<Conditional>>()
                        {
                            Expression::Conditional(cond.clone())
                        } else if let Some(binary) = cond
                            .as_any()
                            .downcast_ref::<Rc<Binary>>()
                        {
                            Expression::Binary(binary.clone())
                        } else if let Some(cast) = cond
                            .as_any()
                            .downcast_ref::<Rc<Cast>>()
                        {
                            Expression::Cast(cast.clone())
                        } else if let Some(index_access) = cond
                            .as_any()
                            .downcast_ref::<Rc<IndexAccess>>()
                        {
                            Expression::IndexAccess(index_access.clone())
                        } else if let Some(member_access) = cond
                            .as_any()
                            .downcast_ref::<Rc<MemberAccess>>()
                        {
                            Expression::MemberAccess(member_access.clone())
                        } else if let Some(function_call) = cond
                            .as_any()
                            .downcast_ref::<Rc<FunctionCall>>()
                        {
                            Expression::FunctionCall(function_call.clone())
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                    NodeKind::SameScopeNode(SameScopeNode::Symbol(identifier)) => {
                        if let Some(identifier) = identifier
                            .as_any()
                            .downcast_ref::<Rc<Identifier>>()
                        {
                            Expression::Identifier(identifier.clone())
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        /// E.g. `const a = bool ? 1 : 2`
        pub struct Conditional {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub condition: Rc<Expression>,
            pub then_branch: Rc<Expression>,
            pub else_branch: Rc<Expression>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Conditional {
            #[inline]
            fn clone(&self) -> Conditional {
                Conditional {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    condition: ::core::clone::Clone::clone(&self.condition),
                    then_branch: ::core::clone::Clone::clone(&self.then_branch),
                    else_branch: ::core::clone::Clone::clone(&self.else_branch),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Conditional {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Conditional {
            #[inline]
            fn eq(&self, other: &Conditional) -> bool {
                self.id == other.id && self.location == other.location
                    && self.condition == other.condition
                    && self.then_branch == other.then_branch
                    && self.else_branch == other.else_branch
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Conditional {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Conditional {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Conditional",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "condition",
                    &self.condition,
                    "then_branch",
                    &self.then_branch,
                    "else_branch",
                    &&self.else_branch,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Conditional {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Conditional",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "condition",
                        &self.condition,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "then_branch",
                        &self.then_branch,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "else_branch",
                        &self.else_branch,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Conditional {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "condition" => _serde::__private::Ok(__Field::__field2),
                                "then_branch" => _serde::__private::Ok(__Field::__field3),
                                "else_branch" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"condition" => _serde::__private::Ok(__Field::__field2),
                                b"then_branch" => _serde::__private::Ok(__Field::__field3),
                                b"else_branch" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Conditional>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Conditional;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Conditional",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Conditional with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Conditional with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Conditional with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Conditional with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Conditional with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Conditional {
                                id: __field0,
                                location: __field1,
                                condition: __field2,
                                then_branch: __field3,
                                else_branch: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "condition",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "then_branch",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "else_branch",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("condition")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("then_branch")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("else_branch")?
                                }
                            };
                            _serde::__private::Ok(Conditional {
                                id: __field0,
                                location: __field1,
                                condition: __field2,
                                then_branch: __field3,
                                else_branch: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "condition",
                        "then_branch",
                        "else_branch",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Conditional",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Conditional>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Binary {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub left_operand: Expression,
            pub right_operand: Expression,
            pub operator: BinaryExpressionOperator,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Binary {
            #[inline]
            fn clone(&self) -> Binary {
                Binary {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    left_operand: ::core::clone::Clone::clone(&self.left_operand),
                    right_operand: ::core::clone::Clone::clone(&self.right_operand),
                    operator: ::core::clone::Clone::clone(&self.operator),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Binary {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Binary {
            #[inline]
            fn eq(&self, other: &Binary) -> bool {
                self.id == other.id && self.location == other.location
                    && self.left_operand == other.left_operand
                    && self.right_operand == other.right_operand
                    && self.operator == other.operator
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Binary {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Expression>;
                let _: ::core::cmp::AssertParamIsEq<BinaryExpressionOperator>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Binary {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Binary",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "left_operand",
                    &self.left_operand,
                    "right_operand",
                    &self.right_operand,
                    "operator",
                    &&self.operator,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Binary {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Binary",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "left_operand",
                        &self.left_operand,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "right_operand",
                        &self.right_operand,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "operator",
                        &self.operator,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Binary {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "left_operand" => _serde::__private::Ok(__Field::__field2),
                                "right_operand" => _serde::__private::Ok(__Field::__field3),
                                "operator" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"left_operand" => _serde::__private::Ok(__Field::__field2),
                                b"right_operand" => _serde::__private::Ok(__Field::__field3),
                                b"operator" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Binary>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Binary;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Binary",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Binary with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Binary with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Binary with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Binary with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                BinaryExpressionOperator,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Binary with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Binary {
                                id: __field0,
                                location: __field1,
                                left_operand: __field2,
                                right_operand: __field3,
                                operator: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Expression> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Expression> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                BinaryExpressionOperator,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "left_operand",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "right_operand",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "operator",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                BinaryExpressionOperator,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("left_operand")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("right_operand")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("operator")?
                                }
                            };
                            _serde::__private::Ok(Binary {
                                id: __field0,
                                location: __field1,
                                left_operand: __field2,
                                right_operand: __field3,
                                operator: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "left_operand",
                        "right_operand",
                        "operator",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Binary",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Binary>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Cast {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub expression: Rc<Expression>,
            pub target_type: Rc<Expression>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Cast {
            #[inline]
            fn clone(&self) -> Cast {
                Cast {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    expression: ::core::clone::Clone::clone(&self.expression),
                    target_type: ::core::clone::Clone::clone(&self.target_type),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Cast {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Cast {
            #[inline]
            fn eq(&self, other: &Cast) -> bool {
                self.id == other.id && self.location == other.location
                    && self.expression == other.expression
                    && self.target_type == other.target_type
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Cast {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Cast {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Cast",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "expression",
                    &self.expression,
                    "target_type",
                    &&self.target_type,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Cast {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Cast",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "expression",
                        &self.expression,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "target_type",
                        &self.target_type,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Cast {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "expression" => _serde::__private::Ok(__Field::__field2),
                                "target_type" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"expression" => _serde::__private::Ok(__Field::__field2),
                                b"target_type" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Cast>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Cast;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Cast",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Cast with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Cast with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Cast with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Cast with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Cast {
                                id: __field0,
                                location: __field1,
                                expression: __field2,
                                target_type: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "expression",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "target_type",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("expression")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("target_type")?
                                }
                            };
                            _serde::__private::Ok(Cast {
                                id: __field0,
                                location: __field1,
                                expression: __field2,
                                target_type: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "expression",
                        "target_type",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Cast",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Cast>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct IndexAccess {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub array: Rc<Expression>,
            pub index: Rc<Expression>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IndexAccess {
            #[inline]
            fn clone(&self) -> IndexAccess {
                IndexAccess {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    array: ::core::clone::Clone::clone(&self.array),
                    index: ::core::clone::Clone::clone(&self.index),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IndexAccess {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IndexAccess {
            #[inline]
            fn eq(&self, other: &IndexAccess) -> bool {
                self.id == other.id && self.location == other.location
                    && self.array == other.array && self.index == other.index
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for IndexAccess {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IndexAccess {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "IndexAccess",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "array",
                    &self.array,
                    "index",
                    &&self.index,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for IndexAccess {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "IndexAccess",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "array",
                        &self.array,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "index",
                        &self.index,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for IndexAccess {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "array" => _serde::__private::Ok(__Field::__field2),
                                "index" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"array" => _serde::__private::Ok(__Field::__field2),
                                b"index" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<IndexAccess>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = IndexAccess;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct IndexAccess",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct IndexAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct IndexAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct IndexAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct IndexAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(IndexAccess {
                                id: __field0,
                                location: __field1,
                                array: __field2,
                                index: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("array"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("array")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("index")?
                                }
                            };
                            _serde::__private::Ok(IndexAccess {
                                id: __field0,
                                location: __field1,
                                array: __field2,
                                index: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "array",
                        "index",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "IndexAccess",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<IndexAccess>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct MemberAccess {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub base: Rc<Expression>,
            pub member: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MemberAccess {
            #[inline]
            fn clone(&self) -> MemberAccess {
                MemberAccess {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    base: ::core::clone::Clone::clone(&self.base),
                    member: ::core::clone::Clone::clone(&self.member),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MemberAccess {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MemberAccess {
            #[inline]
            fn eq(&self, other: &MemberAccess) -> bool {
                self.id == other.id && self.location == other.location
                    && self.base == other.base && self.member == other.member
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for MemberAccess {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MemberAccess {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "MemberAccess",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "base",
                    &self.base,
                    "member",
                    &&self.member,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for MemberAccess {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "MemberAccess",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "base",
                        &self.base,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "member",
                        &self.member,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for MemberAccess {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "base" => _serde::__private::Ok(__Field::__field2),
                                "member" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"base" => _serde::__private::Ok(__Field::__field2),
                                b"member" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<MemberAccess>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = MemberAccess;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct MemberAccess",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct MemberAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct MemberAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct MemberAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct MemberAccess with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(MemberAccess {
                                id: __field0,
                                location: __field1,
                                base: __field2,
                                member: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("base"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("member"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("base")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("member")?
                                }
                            };
                            _serde::__private::Ok(MemberAccess {
                                id: __field0,
                                location: __field1,
                                base: __field2,
                                member: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "base",
                        "member",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "MemberAccess",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<MemberAccess>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct FunctionCall {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub function: Rc<Expression>,
            pub arguments: Vec<Rc<Expression>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FunctionCall {
            #[inline]
            fn clone(&self) -> FunctionCall {
                FunctionCall {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    function: ::core::clone::Clone::clone(&self.function),
                    arguments: ::core::clone::Clone::clone(&self.arguments),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for FunctionCall {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for FunctionCall {
            #[inline]
            fn eq(&self, other: &FunctionCall) -> bool {
                self.id == other.id && self.location == other.location
                    && self.function == other.function
                    && self.arguments == other.arguments
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for FunctionCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Rc<Expression>>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FunctionCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "FunctionCall",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "function",
                    &self.function,
                    "arguments",
                    &&self.arguments,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for FunctionCall {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "FunctionCall",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "function",
                        &self.function,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "arguments",
                        &self.arguments,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for FunctionCall {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "function" => _serde::__private::Ok(__Field::__field2),
                                "arguments" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"function" => _serde::__private::Ok(__Field::__field2),
                                b"arguments" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<FunctionCall>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = FunctionCall;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct FunctionCall",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct FunctionCall with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct FunctionCall with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Rc<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct FunctionCall with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Vec<Rc<Expression>>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct FunctionCall with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(FunctionCall {
                                id: __field0,
                                location: __field1,
                                function: __field2,
                                arguments: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Rc<Expression>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Vec<Rc<Expression>>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "function",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "arguments",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<Rc<Expression>>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("function")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("arguments")?
                                }
                            };
                            _serde::__private::Ok(FunctionCall {
                                id: __field0,
                                location: __field1,
                                function: __field2,
                                arguments: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "function",
                        "arguments",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "FunctionCall",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<FunctionCall>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Identifier {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub name: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Identifier {
            #[inline]
            fn clone(&self) -> Identifier {
                Identifier {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    name: ::core::clone::Clone::clone(&self.name),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Identifier {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Identifier {
            #[inline]
            fn eq(&self, other: &Identifier) -> bool {
                self.id == other.id && self.location == other.location
                    && self.name == other.name
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Identifier {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Identifier {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Identifier",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "name",
                    &&self.name,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Identifier {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Identifier",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Identifier {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "name" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"name" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Identifier>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Identifier",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Identifier with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Identifier with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Identifier with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Identifier {
                                id: __field0,
                                location: __field1,
                                name: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            _serde::__private::Ok(Identifier {
                                id: __field0,
                                location: __field1,
                                name: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location", "name"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Identifier",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Identifier>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub enum BinaryExpressionOperator {
            Add,
            Sub,
            Mul,
            Div,
            Mod,
            Pow,
            Eq,
            Ne,
            Lt,
            Le,
            Gt,
            Ge,
            And,
            Or,
            BitAnd,
            BitOr,
            BitXor,
            BitNot,
            Shl,
            Shr,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BinaryExpressionOperator {
            #[inline]
            fn clone(&self) -> BinaryExpressionOperator {
                match self {
                    BinaryExpressionOperator::Add => BinaryExpressionOperator::Add,
                    BinaryExpressionOperator::Sub => BinaryExpressionOperator::Sub,
                    BinaryExpressionOperator::Mul => BinaryExpressionOperator::Mul,
                    BinaryExpressionOperator::Div => BinaryExpressionOperator::Div,
                    BinaryExpressionOperator::Mod => BinaryExpressionOperator::Mod,
                    BinaryExpressionOperator::Pow => BinaryExpressionOperator::Pow,
                    BinaryExpressionOperator::Eq => BinaryExpressionOperator::Eq,
                    BinaryExpressionOperator::Ne => BinaryExpressionOperator::Ne,
                    BinaryExpressionOperator::Lt => BinaryExpressionOperator::Lt,
                    BinaryExpressionOperator::Le => BinaryExpressionOperator::Le,
                    BinaryExpressionOperator::Gt => BinaryExpressionOperator::Gt,
                    BinaryExpressionOperator::Ge => BinaryExpressionOperator::Ge,
                    BinaryExpressionOperator::And => BinaryExpressionOperator::And,
                    BinaryExpressionOperator::Or => BinaryExpressionOperator::Or,
                    BinaryExpressionOperator::BitAnd => BinaryExpressionOperator::BitAnd,
                    BinaryExpressionOperator::BitOr => BinaryExpressionOperator::BitOr,
                    BinaryExpressionOperator::BitXor => BinaryExpressionOperator::BitXor,
                    BinaryExpressionOperator::BitNot => BinaryExpressionOperator::BitNot,
                    BinaryExpressionOperator::Shl => BinaryExpressionOperator::Shl,
                    BinaryExpressionOperator::Shr => BinaryExpressionOperator::Shr,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for BinaryExpressionOperator {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for BinaryExpressionOperator {
            #[inline]
            fn eq(&self, other: &BinaryExpressionOperator) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for BinaryExpressionOperator {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BinaryExpressionOperator {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        BinaryExpressionOperator::Add => "Add",
                        BinaryExpressionOperator::Sub => "Sub",
                        BinaryExpressionOperator::Mul => "Mul",
                        BinaryExpressionOperator::Div => "Div",
                        BinaryExpressionOperator::Mod => "Mod",
                        BinaryExpressionOperator::Pow => "Pow",
                        BinaryExpressionOperator::Eq => "Eq",
                        BinaryExpressionOperator::Ne => "Ne",
                        BinaryExpressionOperator::Lt => "Lt",
                        BinaryExpressionOperator::Le => "Le",
                        BinaryExpressionOperator::Gt => "Gt",
                        BinaryExpressionOperator::Ge => "Ge",
                        BinaryExpressionOperator::And => "And",
                        BinaryExpressionOperator::Or => "Or",
                        BinaryExpressionOperator::BitAnd => "BitAnd",
                        BinaryExpressionOperator::BitOr => "BitOr",
                        BinaryExpressionOperator::BitXor => "BitXor",
                        BinaryExpressionOperator::BitNot => "BitNot",
                        BinaryExpressionOperator::Shl => "Shl",
                        BinaryExpressionOperator::Shr => "Shr",
                    },
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for BinaryExpressionOperator {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        BinaryExpressionOperator::Add => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                0u32,
                                "Add",
                            )
                        }
                        BinaryExpressionOperator::Sub => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                1u32,
                                "Sub",
                            )
                        }
                        BinaryExpressionOperator::Mul => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                2u32,
                                "Mul",
                            )
                        }
                        BinaryExpressionOperator::Div => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                3u32,
                                "Div",
                            )
                        }
                        BinaryExpressionOperator::Mod => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                4u32,
                                "Mod",
                            )
                        }
                        BinaryExpressionOperator::Pow => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                5u32,
                                "Pow",
                            )
                        }
                        BinaryExpressionOperator::Eq => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                6u32,
                                "Eq",
                            )
                        }
                        BinaryExpressionOperator::Ne => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                7u32,
                                "Ne",
                            )
                        }
                        BinaryExpressionOperator::Lt => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                8u32,
                                "Lt",
                            )
                        }
                        BinaryExpressionOperator::Le => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                9u32,
                                "Le",
                            )
                        }
                        BinaryExpressionOperator::Gt => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                10u32,
                                "Gt",
                            )
                        }
                        BinaryExpressionOperator::Ge => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                11u32,
                                "Ge",
                            )
                        }
                        BinaryExpressionOperator::And => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                12u32,
                                "And",
                            )
                        }
                        BinaryExpressionOperator::Or => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                13u32,
                                "Or",
                            )
                        }
                        BinaryExpressionOperator::BitAnd => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                14u32,
                                "BitAnd",
                            )
                        }
                        BinaryExpressionOperator::BitOr => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                15u32,
                                "BitOr",
                            )
                        }
                        BinaryExpressionOperator::BitXor => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                16u32,
                                "BitXor",
                            )
                        }
                        BinaryExpressionOperator::BitNot => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                17u32,
                                "BitNot",
                            )
                        }
                        BinaryExpressionOperator::Shl => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                18u32,
                                "Shl",
                            )
                        }
                        BinaryExpressionOperator::Shr => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "BinaryExpressionOperator",
                                19u32,
                                "Shr",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for BinaryExpressionOperator {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __field19,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                19u64 => _serde::__private::Ok(__Field::__field19),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 20",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Add" => _serde::__private::Ok(__Field::__field0),
                                "Sub" => _serde::__private::Ok(__Field::__field1),
                                "Mul" => _serde::__private::Ok(__Field::__field2),
                                "Div" => _serde::__private::Ok(__Field::__field3),
                                "Mod" => _serde::__private::Ok(__Field::__field4),
                                "Pow" => _serde::__private::Ok(__Field::__field5),
                                "Eq" => _serde::__private::Ok(__Field::__field6),
                                "Ne" => _serde::__private::Ok(__Field::__field7),
                                "Lt" => _serde::__private::Ok(__Field::__field8),
                                "Le" => _serde::__private::Ok(__Field::__field9),
                                "Gt" => _serde::__private::Ok(__Field::__field10),
                                "Ge" => _serde::__private::Ok(__Field::__field11),
                                "And" => _serde::__private::Ok(__Field::__field12),
                                "Or" => _serde::__private::Ok(__Field::__field13),
                                "BitAnd" => _serde::__private::Ok(__Field::__field14),
                                "BitOr" => _serde::__private::Ok(__Field::__field15),
                                "BitXor" => _serde::__private::Ok(__Field::__field16),
                                "BitNot" => _serde::__private::Ok(__Field::__field17),
                                "Shl" => _serde::__private::Ok(__Field::__field18),
                                "Shr" => _serde::__private::Ok(__Field::__field19),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Add" => _serde::__private::Ok(__Field::__field0),
                                b"Sub" => _serde::__private::Ok(__Field::__field1),
                                b"Mul" => _serde::__private::Ok(__Field::__field2),
                                b"Div" => _serde::__private::Ok(__Field::__field3),
                                b"Mod" => _serde::__private::Ok(__Field::__field4),
                                b"Pow" => _serde::__private::Ok(__Field::__field5),
                                b"Eq" => _serde::__private::Ok(__Field::__field6),
                                b"Ne" => _serde::__private::Ok(__Field::__field7),
                                b"Lt" => _serde::__private::Ok(__Field::__field8),
                                b"Le" => _serde::__private::Ok(__Field::__field9),
                                b"Gt" => _serde::__private::Ok(__Field::__field10),
                                b"Ge" => _serde::__private::Ok(__Field::__field11),
                                b"And" => _serde::__private::Ok(__Field::__field12),
                                b"Or" => _serde::__private::Ok(__Field::__field13),
                                b"BitAnd" => _serde::__private::Ok(__Field::__field14),
                                b"BitOr" => _serde::__private::Ok(__Field::__field15),
                                b"BitXor" => _serde::__private::Ok(__Field::__field16),
                                b"BitNot" => _serde::__private::Ok(__Field::__field17),
                                b"Shl" => _serde::__private::Ok(__Field::__field18),
                                b"Shr" => _serde::__private::Ok(__Field::__field19),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<BinaryExpressionOperator>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = BinaryExpressionOperator;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum BinaryExpressionOperator",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Add)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Sub)
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Mul)
                                }
                                (__Field::__field3, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Div)
                                }
                                (__Field::__field4, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Mod)
                                }
                                (__Field::__field5, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Pow)
                                }
                                (__Field::__field6, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Eq)
                                }
                                (__Field::__field7, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Ne)
                                }
                                (__Field::__field8, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Lt)
                                }
                                (__Field::__field9, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Le)
                                }
                                (__Field::__field10, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Gt)
                                }
                                (__Field::__field11, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Ge)
                                }
                                (__Field::__field12, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::And)
                                }
                                (__Field::__field13, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Or)
                                }
                                (__Field::__field14, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::BitAnd)
                                }
                                (__Field::__field15, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::BitOr)
                                }
                                (__Field::__field16, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::BitXor)
                                }
                                (__Field::__field17, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::BitNot)
                                }
                                (__Field::__field18, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Shl)
                                }
                                (__Field::__field19, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(BinaryExpressionOperator::Shr)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Add",
                        "Sub",
                        "Mul",
                        "Div",
                        "Mod",
                        "Pow",
                        "Eq",
                        "Ne",
                        "Lt",
                        "Le",
                        "Gt",
                        "Ge",
                        "And",
                        "Or",
                        "BitAnd",
                        "BitOr",
                        "BitXor",
                        "BitNot",
                        "Shl",
                        "Shr",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "BinaryExpressionOperator",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                BinaryExpressionOperator,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&BinaryExpressionOperator> for crate::passes::NodeKind {
            fn from(n: &BinaryExpressionOperator) -> Self {
                match n {
                    BinaryExpressionOperator::Add(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Sub(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Mul(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Div(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Mod(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Pow(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Eq(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Ne(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Lt(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Le(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Gt(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Ge(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::And(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Or(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::BitAnd(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::BitOr(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::BitXor(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::BitNot(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Shl(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    BinaryExpressionOperator::Shr(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                }
            }
        }
        impl Node for Conditional {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Rc::new(NodeKind::from(&*self.condition)),
                        Rc::new(NodeKind::from(&*self.then_branch)),
                        Rc::new(NodeKind::from(&*self.else_branch)),
                    ]),
                )
            }
        }
        impl Node for Binary {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Rc::new(NodeKind::from(&self.left_operand)),
                        Rc::new(NodeKind::from(&self.right_operand)),
                    ]),
                )
            }
        }
        impl Node for Cast {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Rc::new(NodeKind::from(&*self.expression)),
                        Rc::new(NodeKind::from(&*self.target_type)),
                    ]),
                )
            }
        }
        impl Node for IndexAccess {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Rc::new(NodeKind::from(&*self.array)),
                        Rc::new(NodeKind::from(&*self.index)),
                    ]),
                )
            }
        }
        impl Node for MemberAccess {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([Rc::new(NodeKind::from(&*self.base))]),
                )
            }
        }
        impl Node for FunctionCall {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                let mut children = <[_]>::into_vec(
                    ::alloc::boxed::box_new([Rc::new(NodeKind::from(&*self.function))]),
                );
                children
                    .extend(
                        self.arguments.iter().map(|arg| Rc::new(NodeKind::from(&**arg))),
                    );
                children
            }
        }
        impl Node for Identifier {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                ::alloc::vec::Vec::new()
            }
        }
        impl SymbolNode for Identifier {
            fn name(&self) -> String {
                self.name.clone()
            }
            fn type_expr(&self) -> Option<&Expression> {
                None
            }
        }
    }
    pub mod literal {
        #![warn(clippy::pedantic)]
        use std::rc::Rc;
        use crate::{ast_enum, ast_nodes, passes::{Node, NodeKind, SameScopeNode}};
        pub enum Literal {
            Nat(Rc<Nat>),
            Bool(Rc<Bool>),
            Str(Rc<Str>),
            Version(Rc<Version>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Literal {
            #[inline]
            fn clone(&self) -> Literal {
                match self {
                    Literal::Nat(__self_0) => {
                        Literal::Nat(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::Bool(__self_0) => {
                        Literal::Bool(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::Str(__self_0) => {
                        Literal::Str(::core::clone::Clone::clone(__self_0))
                    }
                    Literal::Version(__self_0) => {
                        Literal::Version(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Literal {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Literal {
            #[inline]
            fn eq(&self, other: &Literal) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Literal::Nat(__self_0), Literal::Nat(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Literal::Bool(__self_0), Literal::Bool(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Literal::Str(__self_0), Literal::Str(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Literal::Version(__self_0), Literal::Version(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Literal {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rc<Nat>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Bool>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Str>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Version>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Literal {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Literal::Nat(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Nat",
                            &__self_0,
                        )
                    }
                    Literal::Bool(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Bool",
                            &__self_0,
                        )
                    }
                    Literal::Str(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Str",
                            &__self_0,
                        )
                    }
                    Literal::Version(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Version",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Literal {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Literal::Nat(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Literal",
                                0u32,
                                "Nat",
                                __field0,
                            )
                        }
                        Literal::Bool(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Literal",
                                1u32,
                                "Bool",
                                __field0,
                            )
                        }
                        Literal::Str(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Literal",
                                2u32,
                                "Str",
                                __field0,
                            )
                        }
                        Literal::Version(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Literal",
                                3u32,
                                "Version",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Literal {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 4",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Nat" => _serde::__private::Ok(__Field::__field0),
                                "Bool" => _serde::__private::Ok(__Field::__field1),
                                "Str" => _serde::__private::Ok(__Field::__field2),
                                "Version" => _serde::__private::Ok(__Field::__field3),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Nat" => _serde::__private::Ok(__Field::__field0),
                                b"Bool" => _serde::__private::Ok(__Field::__field1),
                                b"Str" => _serde::__private::Ok(__Field::__field2),
                                b"Version" => _serde::__private::Ok(__Field::__field3),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Literal>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Literal;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum Literal",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Nat>,
                                        >(__variant),
                                        Literal::Nat,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Bool>,
                                        >(__variant),
                                        Literal::Bool,
                                    )
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Str>,
                                        >(__variant),
                                        Literal::Str,
                                    )
                                }
                                (__Field::__field3, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Version>,
                                        >(__variant),
                                        Literal::Version,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Nat",
                        "Bool",
                        "Str",
                        "Version",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Literal",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Literal>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&Literal> for crate::passes::NodeKind {
            fn from(n: &Literal) -> Self {
                match n {
                    Literal::Nat(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Literal::Bool(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Literal::Str(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Literal::Version(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                }
            }
        }
        pub struct Nat {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Nat {
            #[inline]
            fn clone(&self) -> Nat {
                Nat {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Nat {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Nat {
            #[inline]
            fn eq(&self, other: &Nat) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Nat {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Nat {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Nat",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Nat {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Nat",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Nat {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Nat>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Nat;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Nat",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Nat with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Nat with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Nat {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Nat {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Nat",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Nat>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Bool {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Bool {
            #[inline]
            fn clone(&self) -> Bool {
                Bool {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Bool {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Bool {
            #[inline]
            fn eq(&self, other: &Bool) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Bool {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Bool {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Bool",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Bool {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Bool",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Bool {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Bool>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Bool;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Bool",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Bool with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Bool with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Bool {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Bool {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Bool",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Bool>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Str {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Str {
            #[inline]
            fn clone(&self) -> Str {
                Str {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Str {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Str {
            #[inline]
            fn eq(&self, other: &Str) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Str {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Str {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Str",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Str {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Str",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Str {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Str>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Str;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Str",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Str with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Str with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Str {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Str {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Str",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Str>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Version {
            pub id: u128,
            pub location: crate::ast::node::Location,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Version {
            #[inline]
            fn clone(&self) -> Version {
                Version {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Version {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Version {
            #[inline]
            fn eq(&self, other: &Version) -> bool {
                self.id == other.id && self.location == other.location
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Version {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Version {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Version",
                    "id",
                    &self.id,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Version {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Version",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Version {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Version>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Version;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Version",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Version with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Version with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Version {
                                id: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(Version {
                                id: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Version",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Version>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Node for Nat {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                ::alloc::vec::Vec::new()
            }
        }
        impl Node for Bool {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                ::alloc::vec::Vec::new()
            }
        }
        impl Node for Str {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                ::alloc::vec::Vec::new()
            }
        }
        impl Node for Version {
            fn children(&self) -> Vec<Rc<crate::passes::NodeKind>> {
                ::alloc::vec::Vec::new()
            }
        }
    }
    pub mod node {
        #![warn(clippy::pedantic)]
        pub struct Location {
            pub source_code: String,
            pub start_line: usize,
            pub start_col: usize,
            pub end_line: usize,
            pub end_col: usize,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Location {
            #[inline]
            fn clone(&self) -> Location {
                Location {
                    source_code: ::core::clone::Clone::clone(&self.source_code),
                    start_line: ::core::clone::Clone::clone(&self.start_line),
                    start_col: ::core::clone::Clone::clone(&self.start_col),
                    end_line: ::core::clone::Clone::clone(&self.end_line),
                    end_col: ::core::clone::Clone::clone(&self.end_col),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Location {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Location {
            #[inline]
            fn eq(&self, other: &Location) -> bool {
                self.source_code == other.source_code
                    && self.start_line == other.start_line
                    && self.start_col == other.start_col
                    && self.end_line == other.end_line && self.end_col == other.end_col
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Location {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Location {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Location",
                    "source_code",
                    &self.source_code,
                    "start_line",
                    &self.start_line,
                    "start_col",
                    &self.start_col,
                    "end_line",
                    &self.end_line,
                    "end_col",
                    &&self.end_col,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Location {
            #[inline]
            fn default() -> Location {
                Location {
                    source_code: ::core::default::Default::default(),
                    start_line: ::core::default::Default::default(),
                    start_col: ::core::default::Default::default(),
                    end_line: ::core::default::Default::default(),
                    end_col: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Location {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Location",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "source_code",
                        &self.source_code,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_line",
                        &self.start_line,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "start_col",
                        &self.start_col,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "end_line",
                        &self.end_line,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "end_col",
                        &self.end_col,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Location {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "source_code" => _serde::__private::Ok(__Field::__field0),
                                "start_line" => _serde::__private::Ok(__Field::__field1),
                                "start_col" => _serde::__private::Ok(__Field::__field2),
                                "end_line" => _serde::__private::Ok(__Field::__field3),
                                "end_col" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"source_code" => _serde::__private::Ok(__Field::__field0),
                                b"start_line" => _serde::__private::Ok(__Field::__field1),
                                b"start_col" => _serde::__private::Ok(__Field::__field2),
                                b"end_line" => _serde::__private::Ok(__Field::__field3),
                                b"end_col" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Location>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Location;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Location",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Location with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Location with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Location with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Location with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                usize,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Location with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Location {
                                source_code: __field0,
                                start_line: __field1,
                                start_col: __field2,
                                end_line: __field3,
                                end_col: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<usize> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<usize> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "source_code",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_line",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "start_col",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "end_line",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "end_col",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("source_code")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("start_line")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("start_col")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("end_line")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("end_col")?
                                }
                            };
                            _serde::__private::Ok(Location {
                                source_code: __field0,
                                start_line: __field1,
                                start_col: __field2,
                                end_line: __field3,
                                end_col: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "source_code",
                        "start_line",
                        "start_col",
                        "end_line",
                        "end_col",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Location",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Location>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod program {
        #![warn(clippy::pedantic)]
        use super::{definition::Module, directive::Directive};
        pub struct Program {
            pub directives: Vec<Directive>,
            pub modules: Vec<Module>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Program {
            #[inline]
            fn clone(&self) -> Program {
                Program {
                    directives: ::core::clone::Clone::clone(&self.directives),
                    modules: ::core::clone::Clone::clone(&self.modules),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Program {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Program {
            #[inline]
            fn eq(&self, other: &Program) -> bool {
                self.directives == other.directives && self.modules == other.modules
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Program {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Vec<Directive>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Module>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Program {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Program",
                    "directives",
                    &self.directives,
                    "modules",
                    &&self.modules,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Program {
            #[inline]
            fn default() -> Program {
                Program {
                    directives: ::core::default::Default::default(),
                    modules: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Program {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Program",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "directives",
                        &self.directives,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "modules",
                        &self.modules,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Program {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "directives" => _serde::__private::Ok(__Field::__field0),
                                "modules" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"directives" => _serde::__private::Ok(__Field::__field0),
                                b"modules" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Program>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Program;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Program",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Vec<Directive>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Program with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Vec<Module>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Program with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Program {
                                directives: __field0,
                                modules: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Vec<Directive>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<Module>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "directives",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<Directive>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "modules",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<Module>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("directives")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("modules")?
                                }
                            };
                            _serde::__private::Ok(Program {
                                directives: __field0,
                                modules: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["directives", "modules"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Program",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Program>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub mod statement {
        use std::rc::Rc;
        use crate::{
            ast_enum, ast_nodes, passes::{Node, NodeKind, SameScopeNode, SymbolNode},
        };
        use super::expression::{Expression, Identifier};
        pub enum Statement {
            Assign(Rc<Assign>),
            Assert(Rc<Assert>),
            Return(Rc<Return>),
            Block(Rc<Block>),
            If(Rc<If>),
            Var(Rc<Var>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Statement {
            #[inline]
            fn clone(&self) -> Statement {
                match self {
                    Statement::Assign(__self_0) => {
                        Statement::Assign(::core::clone::Clone::clone(__self_0))
                    }
                    Statement::Assert(__self_0) => {
                        Statement::Assert(::core::clone::Clone::clone(__self_0))
                    }
                    Statement::Return(__self_0) => {
                        Statement::Return(::core::clone::Clone::clone(__self_0))
                    }
                    Statement::Block(__self_0) => {
                        Statement::Block(::core::clone::Clone::clone(__self_0))
                    }
                    Statement::If(__self_0) => {
                        Statement::If(::core::clone::Clone::clone(__self_0))
                    }
                    Statement::Var(__self_0) => {
                        Statement::Var(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Statement {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Statement {
            #[inline]
            fn eq(&self, other: &Statement) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Statement::Assign(__self_0), Statement::Assign(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Statement::Assert(__self_0), Statement::Assert(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Statement::Return(__self_0), Statement::Return(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Statement::Block(__self_0), Statement::Block(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Statement::If(__self_0), Statement::If(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Statement::Var(__self_0), Statement::Var(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Statement {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rc<Assign>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Assert>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Return>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Block>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<If>>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Var>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Statement {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Statement::Assign(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Assign",
                            &__self_0,
                        )
                    }
                    Statement::Assert(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Assert",
                            &__self_0,
                        )
                    }
                    Statement::Return(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Return",
                            &__self_0,
                        )
                    }
                    Statement::Block(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Block",
                            &__self_0,
                        )
                    }
                    Statement::If(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "If",
                            &__self_0,
                        )
                    }
                    Statement::Var(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Var",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Statement {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Statement::Assign(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Statement",
                                0u32,
                                "Assign",
                                __field0,
                            )
                        }
                        Statement::Assert(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Statement",
                                1u32,
                                "Assert",
                                __field0,
                            )
                        }
                        Statement::Return(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Statement",
                                2u32,
                                "Return",
                                __field0,
                            )
                        }
                        Statement::Block(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Statement",
                                3u32,
                                "Block",
                                __field0,
                            )
                        }
                        Statement::If(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Statement",
                                4u32,
                                "If",
                                __field0,
                            )
                        }
                        Statement::Var(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Statement",
                                5u32,
                                "Var",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Statement {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 6",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Assign" => _serde::__private::Ok(__Field::__field0),
                                "Assert" => _serde::__private::Ok(__Field::__field1),
                                "Return" => _serde::__private::Ok(__Field::__field2),
                                "Block" => _serde::__private::Ok(__Field::__field3),
                                "If" => _serde::__private::Ok(__Field::__field4),
                                "Var" => _serde::__private::Ok(__Field::__field5),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Assign" => _serde::__private::Ok(__Field::__field0),
                                b"Assert" => _serde::__private::Ok(__Field::__field1),
                                b"Return" => _serde::__private::Ok(__Field::__field2),
                                b"Block" => _serde::__private::Ok(__Field::__field3),
                                b"If" => _serde::__private::Ok(__Field::__field4),
                                b"Var" => _serde::__private::Ok(__Field::__field5),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Statement>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Statement;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum Statement",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Assign>,
                                        >(__variant),
                                        Statement::Assign,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Assert>,
                                        >(__variant),
                                        Statement::Assert,
                                    )
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Return>,
                                        >(__variant),
                                        Statement::Return,
                                    )
                                }
                                (__Field::__field3, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Block>,
                                        >(__variant),
                                        Statement::Block,
                                    )
                                }
                                (__Field::__field4, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<If>,
                                        >(__variant),
                                        Statement::If,
                                    )
                                }
                                (__Field::__field5, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<
                                            Rc<Var>,
                                        >(__variant),
                                        Statement::Var,
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Assign",
                        "Assert",
                        "Return",
                        "Block",
                        "If",
                        "Var",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Statement",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Statement>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&Statement> for crate::passes::NodeKind {
            fn from(n: &Statement) -> Self {
                match n {
                    Statement::Assign(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Statement::Assert(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Statement::Return(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Statement::Block(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Statement::If(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    Statement::Var(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Symbol(a.clone()),
                        )
                    }
                }
            }
        }
        pub struct Assign {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub target: Expression,
            pub value: Expression,
            pub operator: AssignOperator,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Assign {
            #[inline]
            fn clone(&self) -> Assign {
                Assign {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    target: ::core::clone::Clone::clone(&self.target),
                    value: ::core::clone::Clone::clone(&self.value),
                    operator: ::core::clone::Clone::clone(&self.operator),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Assign {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Assign {
            #[inline]
            fn eq(&self, other: &Assign) -> bool {
                self.id == other.id && self.location == other.location
                    && self.target == other.target && self.value == other.value
                    && self.operator == other.operator
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Assign {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Expression>;
                let _: ::core::cmp::AssertParamIsEq<AssignOperator>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Assign {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Assign",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "target",
                    &self.target,
                    "value",
                    &self.value,
                    "operator",
                    &&self.operator,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Assign {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Assign",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "target",
                        &self.target,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "value",
                        &self.value,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "operator",
                        &self.operator,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Assign {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "target" => _serde::__private::Ok(__Field::__field2),
                                "value" => _serde::__private::Ok(__Field::__field3),
                                "operator" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"target" => _serde::__private::Ok(__Field::__field2),
                                b"value" => _serde::__private::Ok(__Field::__field3),
                                b"operator" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Assign>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Assign;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Assign",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Assign with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Assign with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Assign with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Assign with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                AssignOperator,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Assign with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Assign {
                                id: __field0,
                                location: __field1,
                                target: __field2,
                                value: __field3,
                                operator: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Expression> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Expression> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                AssignOperator,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("target"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "operator",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                AssignOperator,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("target")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("value")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("operator")?
                                }
                            };
                            _serde::__private::Ok(Assign {
                                id: __field0,
                                location: __field1,
                                target: __field2,
                                value: __field3,
                                operator: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "target",
                        "value",
                        "operator",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Assign",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Assign>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Return {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub value: Option<Expression>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Return {
            #[inline]
            fn clone(&self) -> Return {
                Return {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Return {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Return {
            #[inline]
            fn eq(&self, other: &Return) -> bool {
                self.id == other.id && self.location == other.location
                    && self.value == other.value
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Return {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Option<Expression>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Return {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Return",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "value",
                    &&self.value,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Return {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Return",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "value",
                        &self.value,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Return {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "value" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"value" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Return>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Return;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Return",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Return with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Return with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Return with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Return {
                                id: __field0,
                                location: __field1,
                                value: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<Expression>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("value")?
                                }
                            };
                            _serde::__private::Ok(Return {
                                id: __field0,
                                location: __field1,
                                value: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "location", "value"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Return",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Return>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct If {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub condition: Expression,
            pub then_branch: Statement,
            pub else_branch: Option<Statement>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for If {
            #[inline]
            fn clone(&self) -> If {
                If {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    condition: ::core::clone::Clone::clone(&self.condition),
                    then_branch: ::core::clone::Clone::clone(&self.then_branch),
                    else_branch: ::core::clone::Clone::clone(&self.else_branch),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for If {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for If {
            #[inline]
            fn eq(&self, other: &If) -> bool {
                self.id == other.id && self.location == other.location
                    && self.condition == other.condition
                    && self.then_branch == other.then_branch
                    && self.else_branch == other.else_branch
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for If {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Expression>;
                let _: ::core::cmp::AssertParamIsEq<Statement>;
                let _: ::core::cmp::AssertParamIsEq<Option<Statement>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for If {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "If",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "condition",
                    &self.condition,
                    "then_branch",
                    &self.then_branch,
                    "else_branch",
                    &&self.else_branch,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for If {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "If",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "condition",
                        &self.condition,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "then_branch",
                        &self.then_branch,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "else_branch",
                        &self.else_branch,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for If {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "condition" => _serde::__private::Ok(__Field::__field2),
                                "then_branch" => _serde::__private::Ok(__Field::__field3),
                                "else_branch" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"condition" => _serde::__private::Ok(__Field::__field2),
                                b"then_branch" => _serde::__private::Ok(__Field::__field3),
                                b"else_branch" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<If>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = If;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct If",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct If with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct If with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct If with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Statement,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct If with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<Statement>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct If with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(If {
                                id: __field0,
                                location: __field1,
                                condition: __field2,
                                then_branch: __field3,
                                else_branch: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Expression> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Statement> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<Statement>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "condition",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "then_branch",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Statement>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "else_branch",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Statement>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("condition")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("then_branch")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("else_branch")?
                                }
                            };
                            _serde::__private::Ok(If {
                                id: __field0,
                                location: __field1,
                                condition: __field2,
                                then_branch: __field3,
                                else_branch: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "condition",
                        "then_branch",
                        "else_branch",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "If",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<If>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct For {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub init: Option<Statement>,
            pub condition: Option<Expression>,
            pub update: Option<Statement>,
            pub body: Statement,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for For {
            #[inline]
            fn clone(&self) -> For {
                For {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    init: ::core::clone::Clone::clone(&self.init),
                    condition: ::core::clone::Clone::clone(&self.condition),
                    update: ::core::clone::Clone::clone(&self.update),
                    body: ::core::clone::Clone::clone(&self.body),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for For {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for For {
            #[inline]
            fn eq(&self, other: &For) -> bool {
                self.id == other.id && self.location == other.location
                    && self.init == other.init && self.condition == other.condition
                    && self.update == other.update && self.body == other.body
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for For {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Option<Statement>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Expression>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Statement>>;
                let _: ::core::cmp::AssertParamIsEq<Statement>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for For {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "location",
                    "init",
                    "condition",
                    "update",
                    "body",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.location,
                    &self.init,
                    &self.condition,
                    &self.update,
                    &&self.body,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "For",
                    names,
                    values,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for For {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "For",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "init",
                        &self.init,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "condition",
                        &self.condition,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "update",
                        &self.update,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "body",
                        &self.body,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for For {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "init" => _serde::__private::Ok(__Field::__field2),
                                "condition" => _serde::__private::Ok(__Field::__field3),
                                "update" => _serde::__private::Ok(__Field::__field4),
                                "body" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"init" => _serde::__private::Ok(__Field::__field2),
                                b"condition" => _serde::__private::Ok(__Field::__field3),
                                b"update" => _serde::__private::Ok(__Field::__field4),
                                b"body" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<For>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = For;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct For",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct For with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct For with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<Statement>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct For with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Option<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct For with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<Statement>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct For with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Statement,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct For with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(For {
                                id: __field0,
                                location: __field1,
                                init: __field2,
                                condition: __field3,
                                update: __field4,
                                body: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<Statement>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<Expression>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<Statement>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Statement> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("init"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Statement>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "condition",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("update"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Statement>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("body"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Statement>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("init")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("condition")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("update")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("body")?
                                }
                            };
                            _serde::__private::Ok(For {
                                id: __field0,
                                location: __field1,
                                init: __field2,
                                condition: __field3,
                                update: __field4,
                                body: __field5,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "init",
                        "condition",
                        "update",
                        "body",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "For",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<For>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Assert {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub condition: Expression,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Assert {
            #[inline]
            fn clone(&self) -> Assert {
                Assert {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    condition: ::core::clone::Clone::clone(&self.condition),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Assert {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Assert {
            #[inline]
            fn eq(&self, other: &Assert) -> bool {
                self.id == other.id && self.location == other.location
                    && self.condition == other.condition
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Assert {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Expression>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Assert {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Assert",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "condition",
                    &&self.condition,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Assert {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Assert",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "condition",
                        &self.condition,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Assert {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "condition" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"condition" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Assert>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Assert;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Assert",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Assert with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Assert with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Assert with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Assert {
                                id: __field0,
                                location: __field1,
                                condition: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Expression> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "condition",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("condition")?
                                }
                            };
                            _serde::__private::Ok(Assert {
                                id: __field0,
                                location: __field1,
                                condition: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "condition",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Assert",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Assert>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Var {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub name: Rc<Identifier>,
            pub value: Expression,
            pub ty_: Option<Expression>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Var {
            #[inline]
            fn clone(&self) -> Var {
                Var {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    name: ::core::clone::Clone::clone(&self.name),
                    value: ::core::clone::Clone::clone(&self.value),
                    ty_: ::core::clone::Clone::clone(&self.ty_),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Var {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Var {
            #[inline]
            fn eq(&self, other: &Var) -> bool {
                self.id == other.id && self.location == other.location
                    && self.name == other.name && self.value == other.value
                    && self.ty_ == other.ty_
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Var {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Rc<Identifier>>;
                let _: ::core::cmp::AssertParamIsEq<Expression>;
                let _: ::core::cmp::AssertParamIsEq<Option<Expression>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Var {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Var",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "name",
                    &self.name,
                    "value",
                    &self.value,
                    "ty_",
                    &&self.ty_,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Var {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Var",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "value",
                        &self.value,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "ty_",
                        &self.ty_,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Var {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "name" => _serde::__private::Ok(__Field::__field2),
                                "value" => _serde::__private::Ok(__Field::__field3),
                                "ty_" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"name" => _serde::__private::Ok(__Field::__field2),
                                b"value" => _serde::__private::Ok(__Field::__field3),
                                b"ty_" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Var>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Var;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Var",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Var with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Var with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Rc<Identifier>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Var with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Expression,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Var with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Option<Expression>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Var with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Var {
                                id: __field0,
                                location: __field1,
                                name: __field2,
                                value: __field3,
                                ty_: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Rc<Identifier>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Expression> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<Expression>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Rc<Identifier>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Expression>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("ty_"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Expression>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("value")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("ty_")?
                                }
                            };
                            _serde::__private::Ok(Var {
                                id: __field0,
                                location: __field1,
                                name: __field2,
                                value: __field3,
                                ty_: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "name",
                        "value",
                        "ty_",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Var",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Var>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Block {
            pub id: u128,
            pub location: crate::ast::node::Location,
            pub statements: Vec<Statement>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Block {
            #[inline]
            fn clone(&self) -> Block {
                Block {
                    id: ::core::clone::Clone::clone(&self.id),
                    location: ::core::clone::Clone::clone(&self.location),
                    statements: ::core::clone::Clone::clone(&self.statements),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Block {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Block {
            #[inline]
            fn eq(&self, other: &Block) -> bool {
                self.id == other.id && self.location == other.location
                    && self.statements == other.statements
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Block {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u128>;
                let _: ::core::cmp::AssertParamIsEq<crate::ast::node::Location>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Statement>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Block {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Block",
                    "id",
                    &self.id,
                    "location",
                    &self.location,
                    "statements",
                    &&self.statements,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Block {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Block",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "statements",
                        &self.statements,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Block {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                "statements" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                b"statements" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Block>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Block;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Block",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u128,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Block with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                crate::ast::node::Location,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Block with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<Statement>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Block with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Block {
                                id: __field0,
                                location: __field1,
                                statements: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u128> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                crate::ast::node::Location,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Vec<Statement>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u128>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                crate::ast::node::Location,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "statements",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<Statement>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("statements")?
                                }
                            };
                            _serde::__private::Ok(Block {
                                id: __field0,
                                location: __field1,
                                statements: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "location",
                        "statements",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Block",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Block>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub enum AssignOperator {
            Simple,
            Add,
            Sub,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AssignOperator {
            #[inline]
            fn clone(&self) -> AssignOperator {
                match self {
                    AssignOperator::Simple => AssignOperator::Simple,
                    AssignOperator::Add => AssignOperator::Add,
                    AssignOperator::Sub => AssignOperator::Sub,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AssignOperator {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AssignOperator {
            #[inline]
            fn eq(&self, other: &AssignOperator) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for AssignOperator {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AssignOperator {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        AssignOperator::Simple => "Simple",
                        AssignOperator::Add => "Add",
                        AssignOperator::Sub => "Sub",
                    },
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for AssignOperator {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        AssignOperator::Simple => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "AssignOperator",
                                0u32,
                                "Simple",
                            )
                        }
                        AssignOperator::Add => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "AssignOperator",
                                1u32,
                                "Add",
                            )
                        }
                        AssignOperator::Sub => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "AssignOperator",
                                2u32,
                                "Sub",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for AssignOperator {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 3",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Simple" => _serde::__private::Ok(__Field::__field0),
                                "Add" => _serde::__private::Ok(__Field::__field1),
                                "Sub" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Simple" => _serde::__private::Ok(__Field::__field0),
                                b"Add" => _serde::__private::Ok(__Field::__field1),
                                b"Sub" => _serde::__private::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<AssignOperator>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = AssignOperator;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum AssignOperator",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(AssignOperator::Simple)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(AssignOperator::Add)
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(AssignOperator::Sub)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Simple", "Add", "Sub"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "AssignOperator",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<AssignOperator>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl From<&AssignOperator> for crate::passes::NodeKind {
            fn from(n: &AssignOperator) -> Self {
                match n {
                    AssignOperator::Simple(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    AssignOperator::Add(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                    AssignOperator::Sub(a) => {
                        crate::passes::NodeKind::SameScopeNode(
                            crate::passes::SameScopeNode::Composite(a.clone()),
                        )
                    }
                }
            }
        }
        impl Node for Assign {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Rc::new(NodeKind::from(&self.target)),
                        Rc::new(NodeKind::from(&self.value)),
                    ]),
                )
            }
        }
        impl Node for Return {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                self.value.iter().map(|expr| Rc::new(NodeKind::from(expr))).collect()
            }
        }
        impl Node for If {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                let mut children = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        Rc::new(NodeKind::from(&self.condition)),
                        Rc::new(NodeKind::from(&self.then_branch)),
                    ]),
                );
                if let Some(else_branch) = &self.else_branch {
                    children.push(Rc::new(NodeKind::from(else_branch)));
                }
                children
            }
        }
        impl Node for Assert {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([Rc::new(NodeKind::from(&self.condition))]),
                )
            }
        }
        impl Node for Var {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([Rc::new(NodeKind::from(&self.value))]),
                )
            }
        }
        impl SymbolNode for crate::ast::statement::Var {
            fn name(&self) -> String {
                self.name.name.clone()
            }
            fn type_expr(&self) -> Option<&Expression> {
                self.ty_.as_ref().or(Some(&self.value))
            }
        }
        impl Node for Block {
            fn children(&self) -> Vec<Rc<NodeKind>> {
                self.statements
                    .iter()
                    .map(|stmt| Rc::new(NodeKind::from(stmt)))
                    .collect()
            }
        }
    }
}
pub mod codebase {
    #![warn(clippy::pedantic)]
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, marker::PhantomData};
    #[allow(dead_code)]
    trait CodebaseOpen {}
    #[allow(dead_code)]
    trait CodebaseSealed {}
    pub struct OpenState;
    impl CodebaseOpen for OpenState {}
    pub struct SealedState;
    impl CodebaseSealed for SealedState {}
    #[allow(dead_code)]
    pub struct SourceCodeFile {
        pub(crate) fname: String,
        pub(crate) content: String,
    }
    #[allow(dead_code)]
    pub struct Codebase<S> {
        #[serde(skip)]
        pub(crate) fname_ast_map: Option<HashMap<String, SourceCodeFile>>,
        pub(crate) _state: PhantomData<S>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<S> _serde::Serialize for Codebase<S> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Codebase",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "_state",
                    &self._state,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, S> _serde::Deserialize<'de> for Codebase<S> {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "_state" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"_state" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, S> {
                    marker: _serde::__private::PhantomData<Codebase<S>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de, S> _serde::de::Visitor<'de> for __Visitor<'de, S> {
                    type Value = Codebase<S>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Codebase",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = _serde::__private::Default::default();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            PhantomData<S>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Codebase with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Codebase {
                            fname_ast_map: __field0,
                            _state: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<PhantomData<S>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("_state"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            PhantomData<S>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("_state")?
                            }
                        };
                        _serde::__private::Ok(Codebase {
                            fname_ast_map: _serde::__private::Default::default(),
                            _state: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["_state"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Codebase",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Codebase<S>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Codebase<SealedState> {}
}
mod passes {
    #![allow(dead_code)]
    use anyhow::{anyhow, Ok, Result};
    use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};
    use crate::ast::{
        expression::{BinaryExpressionOperator, Expression},
        literal::Literal,
    };
    pub enum Type {
        Int,
        Bool,
        String,
        Vector(u128, Box<Type>),
        #[default]
        Unknown,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Type {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Type::Int => ::core::fmt::Formatter::write_str(f, "Int"),
                Type::Bool => ::core::fmt::Formatter::write_str(f, "Bool"),
                Type::String => ::core::fmt::Formatter::write_str(f, "String"),
                Type::Vector(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Vector",
                        __self_0,
                        &__self_1,
                    )
                }
                Type::Unknown => ::core::fmt::Formatter::write_str(f, "Unknown"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Type {
        #[inline]
        fn default() -> Type {
            Self::Unknown
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Type {
        #[inline]
        fn clone(&self) -> Type {
            match self {
                Type::Int => Type::Int,
                Type::Bool => Type::Bool,
                Type::String => Type::String,
                Type::Vector(__self_0, __self_1) => {
                    Type::Vector(
                        ::core::clone::Clone::clone(__self_0),
                        ::core::clone::Clone::clone(__self_1),
                    )
                }
                Type::Unknown => Type::Unknown,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Type {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Type {
        #[inline]
        fn eq(&self, other: &Type) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        Type::Vector(__self_0, __self_1),
                        Type::Vector(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Type {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u128>;
            let _: ::core::cmp::AssertParamIsEq<Box<Type>>;
        }
    }
    enum TypeError {
        Undefined(String),
        Mismatch(Type, Type),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TypeError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                TypeError::Undefined(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Undefined",
                        &__self_0,
                    )
                }
                TypeError::Mismatch(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Mismatch",
                        __self_0,
                        &__self_1,
                    )
                }
            }
        }
    }
    pub enum NodeKind {
        SameScopeNode(SameScopeNode),
        NewScope(Rc<dyn Node>),
    }
    pub trait NodeSymbolNode: Node + SymbolNode + Any {}
    impl<T> NodeSymbolNode for T
    where
        T: Node + SymbolNode + Any,
    {}
    impl<'a> From<&'a Rc<dyn NodeSymbolNode>> for &'a dyn Node {
        fn from(node: &'a Rc<dyn NodeSymbolNode>) -> Self {
            node as &'a dyn Node
        }
    }
    impl Node for Rc<dyn NodeSymbolNode> {
        fn children(&self) -> Vec<Rc<NodeKind>> {
            match self.as_any().downcast_ref::<SameScopeNode>() {
                Some(SameScopeNode::Composite(comp_node)) => comp_node.children(),
                _ => ::alloc::vec::Vec::new(),
            }
        }
    }
    impl dyn NodeSymbolNode {
        pub fn as_any(&self) -> &dyn Any {
            self
        }
    }
    pub enum SameScopeNode {
        Symbol(Rc<dyn NodeSymbolNode>),
        Composite(Rc<dyn Node>),
    }
    impl From<Rc<dyn Node>> for NodeKind {
        fn from(node: Rc<dyn Node>) -> Self {
            NodeKind::NewScope(node)
        }
    }
    pub trait Node: Any {
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
    pub struct SymbolTable {
        pub symbols: RefCell<HashMap<String, Type>>,
        pub parent: Option<Rc<SymbolTable>>,
        pub children: RefCell<Vec<Rc<SymbolTable>>>,
    }
    impl SymbolTable {
        pub fn new(parent: Option<Rc<SymbolTable>>) -> Self {
            Self {
                symbols: RefCell::new(HashMap::new()),
                children: RefCell::new(Vec::new()),
                parent,
            }
        }
        #[allow(clippy::map_entry)]
        pub fn insert(&self, name: String, ty: Type) -> Result<()> {
            let mut syms = self.symbols.borrow_mut();
            if syms.contains_key(&name) {
                Err(
                    ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Symbol {0} already exists", name),
                        );
                        error
                    }),
                )
            } else {
                syms.insert(name, ty);
                Ok(())
            }
        }
        pub fn lookup(&self, name: &str) -> Option<Type> {
            let syms = self.symbols.borrow();
            if let Some(sym) = syms.get(name) {
                Some(sym.clone())
            } else if let Some(ref parent) = self.parent {
                parent.lookup(name)
            } else {
                None
            }
        }
    }
    pub fn build_symbol_table(
        node_kind: Rc<NodeKind>,
        parent: Option<Rc<SymbolTable>>,
    ) -> anyhow::Result<Rc<SymbolTable>> {
        let symbol_table = Rc::new(SymbolTable::new(parent));
        let mut nodes: Vec<Rc<NodeKind>> = <[_]>::into_vec(
            ::alloc::boxed::box_new([node_kind]),
        );
        while let Some(node) = nodes.pop() {
            match node.as_ref() {
                NodeKind::NewScope(node) => {
                    let child_scope = build_symbol_table(
                        Rc::new(NodeKind::NewScope(node.clone())),
                        Some(symbol_table.clone()),
                    )?;
                    symbol_table.children.borrow_mut().push(child_scope);
                }
                NodeKind::SameScopeNode(same) => {
                    match same {
                        SameScopeNode::Composite(comp_node) => {
                            for child in comp_node.children() {
                                nodes.push(child);
                            }
                        }
                        SameScopeNode::Symbol(symbol_node) => {
                            let symbol_name = symbol_node.name();
                            let symbol_type = if let Some(type_expr) = symbol_node
                                .type_expr()
                            {
                                infer_expr(type_expr, &symbol_table)?
                            } else {
                                Type::Unknown
                            };
                            if symbol_table.symbols.borrow().contains_key(&symbol_name) {
                                if let Some(st) = symbol_table.lookup(&symbol_name) {
                                    if st == Type::Unknown {
                                        return Err(
                                            ::anyhow::__private::must_use({
                                                let error = ::anyhow::__private::format_err(
                                                    format_args!(
                                                        "Symbol {0} without a type in a symbol table",
                                                        symbol_name,
                                                    ),
                                                );
                                                error
                                            }),
                                        );
                                    } else if symbol_type == Type::Unknown {
                                        symbol_table
                                            .symbols
                                            .borrow_mut()
                                            .insert(symbol_name.clone(), st);
                                    } else {
                                        return Err(
                                            ::anyhow::__private::must_use({
                                                let error = ::anyhow::__private::format_err(
                                                    format_args!("Symbol {0} already exists", symbol_name),
                                                );
                                                error
                                            }),
                                        );
                                    }
                                }
                            } else {
                                symbol_table
                                    .symbols
                                    .borrow_mut()
                                    .insert(symbol_name.clone(), symbol_type);
                            }
                            for child in symbol_node.children() {
                                nodes.push(child);
                            }
                        }
                    }
                }
            }
        }
        Ok(symbol_table)
    }
    fn infer_expr(expr: &Expression, env: &Rc<SymbolTable>) -> Result<Type> {
        match expr {
            Expression::Literal(lit) => {
                match lit {
                    Literal::Nat(_) => Ok(Type::Int),
                    Literal::Bool(_) => Ok(Type::Bool),
                    Literal::Str(_) => Ok(Type::String),
                    Literal::Version(_) => Ok(Type::Unknown),
                }
            }
            Expression::Binary(bin_expr) => {
                let left = infer_expr(&bin_expr.left_operand, env)?;
                let right = infer_expr(&bin_expr.right_operand, env)?;
                match bin_expr.operator {
                    BinaryExpressionOperator::Add
                    | BinaryExpressionOperator::Sub
                    | BinaryExpressionOperator::Mul
                    | BinaryExpressionOperator::Div
                    | BinaryExpressionOperator::Mod
                    | BinaryExpressionOperator::Pow
                    | BinaryExpressionOperator::BitAnd
                    | BinaryExpressionOperator::BitOr
                    | BinaryExpressionOperator::BitXor
                    | BinaryExpressionOperator::BitNot
                    | BinaryExpressionOperator::Shl
                    | BinaryExpressionOperator::Shr => {
                        if left == right {
                            Ok(left)
                        } else {
                            Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("Type mismatch"),
                                    );
                                    error
                                }),
                            )
                        }
                    }
                    BinaryExpressionOperator::Eq
                    | BinaryExpressionOperator::Ne
                    | BinaryExpressionOperator::Lt
                    | BinaryExpressionOperator::Le
                    | BinaryExpressionOperator::Gt
                    | BinaryExpressionOperator::Ge
                    | BinaryExpressionOperator::And
                    | BinaryExpressionOperator::Or => {
                        if left == right {
                            Ok(Type::Bool)
                        } else {
                            Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("Type mismatch"),
                                    );
                                    error
                                }),
                            )
                        }
                    }
                }
            }
            Expression::Conditional(conditional) => {
                let then_type = infer_expr(&conditional.then_branch, env)?;
                let else_type = infer_expr(&conditional.else_branch, env)?;
                if then_type == else_type {
                    Ok(then_type)
                } else {
                    Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("Type mismatch"),
                            );
                            error
                        }),
                    )
                }
            }
            Expression::Cast(cast) => infer_expr(&cast.target_type, env),
            Expression::IndexAccess(index_access) => {
                let vec_type = infer_expr(&index_access.array, env)?;
                if let Type::Vector(_, ty) = vec_type {
                    Ok(*ty)
                } else {
                    Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("Type mismatch"),
                            );
                            error
                        }),
                    )
                }
            }
            Expression::MemberAccess(member_access) => {
                infer_expr(&member_access.base, env)
            }
            Expression::FunctionCall(function_call) => {
                infer_expr(&function_call.function, env)
            }
            Expression::Identifier(identifier) => {
                let symbol_type = env
                    .lookup(&identifier.name)
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Undefined identifier"),
                        );
                        error
                    }))?;
                Ok(symbol_type)
            }
        }
    }
}
