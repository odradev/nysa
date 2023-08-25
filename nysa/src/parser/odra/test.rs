use std::{fs::File, io::Read};

use c3_lang_linearization::{Class, Fn};
use c3_lang_parser::c3_ast::{ClassFnImpl, ClassNameDef, ComplexFnDef, FnDef, PlainFnDef, VarDef};
use pretty_assertions::assert_eq;
use quote::ToTokens;
use syn::parse_quote;

use crate::{
    parse,
    parser::odra::other::{attrs, path_stack_default_impl},
};

use super::*;

#[test]
fn test_constructor() {
    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/1.sol"));
    assert_impl(result, "../resources/constructors/1.rs");

    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/2.sol"));
    assert_impl(result, "../resources/constructors/2.rs");

    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/3.sol"));
    assert_impl(result, "../resources/constructors/3.rs");

    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/4.sol"));
    assert_impl(result, "../resources/constructors/4.rs");

    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/5.sol"));
    assert_impl(result, "../resources/constructors/5.rs");

    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/6.sol"));
    assert_impl(result, "../resources/constructors/6.rs");

    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/constructors/7.sol"));
    assert_impl(result, "../resources/constructors/7.rs");
}

#[test]
fn test_modifier() {
    let result = parse::<OdraParser, _>(include_str!("../../../../resources/modifiers/1.sol"));
    assert_impl(result, "../resources/modifiers/1.rs");
}

#[test]
fn test_owner() {
    let result: PackageDef =
        parse::<OdraParser, _>(include_str!("../../../../resources/ownable.sol"));

    assert_eq!(
        result,
        PackageDef {
            attrs: attrs(),
            other_code: path_stack_default_impl(),
            class_name: ClassNameDef {
                classes: vec![Class::from("Owner")],
            },
            classes: vec![ClassDef {
                struct_attrs: vec![parse_quote!(#[odra::module])],
                impl_attrs: vec![parse_quote!(#[odra::module])],
                class: Class::from("Owner"),
                path: vec![Class::from("Owner")],
                variables: vec![VarDef {
                    ident: parse_quote! { owner },
                    ty: parse_quote! { odra::Variable<Option<odra::types::Address>> },
                }],
                functions: vec![
                    FnDef::Complex(ComplexFnDef {
                        attrs: vec![],
                        name: Fn::from("get_owner"),
                        args: vec![parse_quote!(&self)],
                        ret: parse_quote!(-> Option<odra::types::Address>),
                        implementations: vec![ClassFnImpl {
                            class: Some(Class::from("Owner")),
                            fun: Fn::from("get_owner"),
                            implementation: parse_quote! {{
                                return self.owner.get().unwrap_or(None);
                            }},
                            visibility: parse_quote!(pub),
                        }],
                    }),
                    FnDef::Plain(PlainFnDef {
                        attrs: vec![parse_quote!(#[odra(init)])],
                        name: Fn::from("init"),
                        args: vec![parse_quote!(&mut self)],
                        ret: parse_quote!(),
                        implementation: ClassFnImpl {
                            class: None,
                            fun: Fn::from("init"),
                            implementation: parse_quote! {{
                                self.owner.set(Some(odra::contract_env::caller()));
                            }},
                            visibility: parse_quote!(pub),
                        },
                    }),
                    FnDef::Plain(PlainFnDef {
                        attrs: vec![],
                        name: Fn::from("modifier_before_only_owner"),
                        args: vec![parse_quote!(&mut self),],
                        ret: parse_quote!(),
                        implementation: ClassFnImpl {
                            class: None,
                            fun: Fn::from("modifier_before_only_owner"),
                            implementation: parse_quote!({
                                if !(Some(odra::contract_env::caller())
                                    == self.owner.get().unwrap_or(None))
                                {
                                    odra::contract_env::revert(odra::types::ExecutionError::new(
                                        1u16,
                                        "Only the contract owner can call this function.",
                                    ))
                                };
                            }),
                            visibility: parse_quote!(),
                        },
                    }),
                    FnDef::Plain(PlainFnDef {
                        attrs: vec![],
                        name: Fn::from("modifier_after_only_owner"),
                        args: vec![parse_quote!(&mut self),],
                        ret: parse_quote!(),
                        implementation: ClassFnImpl {
                            class: None,
                            fun: Fn::from("modifier_after_only_owner"),
                            implementation: parse_quote!({}),
                            visibility: parse_quote!(),
                        },
                    }),
                    FnDef::Complex(ComplexFnDef {
                        attrs: vec![],
                        name: Fn::from("transfer_ownership"),
                        args: vec![
                            parse_quote!(&mut self),
                            parse_quote!(new_owner: Option<odra::types::Address>)
                        ],
                        ret: parse_quote!(),
                        implementations: vec![ClassFnImpl {
                            class: Some(Class::from("Owner")),
                            fun: Fn::from("transfer_ownership"),
                            implementation: parse_quote! {{
                                self.modifier_before_only_owner();
                                self.owner.set(new_owner);
                                self.modifier_after_only_owner();
                            }},
                            visibility: parse_quote!(pub),
                        }],
                    }),
                ],
            }],
        }
    );
}

fn assert_impl(result: PackageDef, file_path: &str) {
    let parse = |str| {
        let file = syn::parse_file(str).unwrap();
        prettyplease::unparse(&file)
    };
    let result = result.classes.first().to_token_stream().to_string();

    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    pretty_assertions::assert_eq!(parse(result.as_str()), parse(content.as_str()));
}
