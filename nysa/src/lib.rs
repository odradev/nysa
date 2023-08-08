#![allow(unused_variables)]

use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use model::ContractData;
use quote::format_ident;
use syn::{parse_quote, Attribute, Item};

#[cfg(feature = "builder")]
pub mod builder;
mod event;
mod expr;
mod errors;
mod func;
mod linearization;
mod model;
mod stmt;
mod ty;
mod utils;
mod var;

use std::{collections::{HashMap, HashSet}, sync::Mutex};

lazy_static::lazy_static! {
    static ref ERROR_MAP: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
    static ref ERRORS: Mutex<u16> = Mutex::new(0);

    static ref MSG_DATA: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref SOLIDITY_ERRORS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

/// Parses solidity code into a C3 linearized, near compatible ast
pub fn parse(input: String) -> PackageDef {
    let solidity_ast = utils::parse_to_solidity_ast(&input);
    let contract_data = ContractData::new(&solidity_ast);
    
    let class_name = contract_data.c3_class_name_def();
    
    let mut classes = vec![];
    classes.extend(event::events_def(&contract_data));
    classes.push(contract_def(&contract_data));
    PackageDef {
        attrs: attrs(),
        other_code: other_code(&contract_data),
        class_name,
        classes,
    }
}

/// Builds a c3 contract class definition
fn contract_def(data: &ContractData) -> ClassDef {
    let variables = var::variables_def(data);
    let functions = func::functions_def(data);
    
    let events = data.c3_events_str().iter().map(|ev| format_ident!("{}", ev)).collect::<Vec<_>>();
    let struct_attrs = match events.len() {
        0 => vec![parse_quote!(#[odra::module])],
        _ => vec![parse_quote!(#[odra::module(events = [ #(#events),* ])])]
    };

    ClassDef {
        struct_attrs,
        impl_attrs: vec![parse_quote!(#[odra::module])],
        class: data.c3_class(),
        path: data.c3_path(),
        variables,
        functions,
    }
}

fn attrs() -> Vec<Attribute> {
    vec![parse_quote!(#![allow(unused_braces, non_snake_case)])]
}

/// Generates code that is not a direct derivative of Solidity code.
fn other_code(data: &ContractData) -> Vec<Item> {
    vec![
        parse_quote! {
            impl odra::types::contract_def::Node for PathStack {
                const COUNT: u32 = 0;
                const IS_LEAF: bool = false;
            }
        },
        parse_quote! {
            impl odra::OdraItem for PathStack {
                fn is_module() -> bool {
                    false
                }
            }
        },
        parse_quote! {
            impl odra::StaticInstance for PathStack {
                fn instance<'a>(keys: &'a [&'a str]) -> (Self, &'a [&'a str]) {
                    (PathStack::default(), keys)
                }
            }
        },
        parse_quote! {
            impl odra::DynamicInstance for PathStack {
                #[allow(unused_variables)]
                fn instance(namespace: &[u8]) -> Self {
                    PathStack::default()
                }
            }
        },
        errors::errors_def(data)
    ]
}

#[cfg(test)]
mod tests {
    use c3_lang_linearization::{Class, Fn};
    use c3_lang_parser::c3_ast::{
        ClassFnImpl, ClassNameDef, ComplexFnDef, FnDef, PlainFnDef, VarDef,
    };
    use pretty_assertions::assert_eq;
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    const INPUT_OWNABLE: &str = r#"
    pragma solidity ^0.8.0;

    contract Owner {
        address private owner;
    
        constructor() {
            owner = msg.sender;
        }

        modifier onlyOwner() {
            require(msg.sender == owner, "Only the contract owner can call this function.");
            _;
        }

        function getOwner() public view returns (address) {
            return owner;
        }

        function transferOwnership(address newOwner) public onlyOwner {
            owner = newOwner;
        }
    }
    "#;

    #[test]
    fn test_parser() {
        let result: PackageDef = parse(String::from(include_str!(
            "../../resources/plascoin.sol"
        )));
        dbg!(result.to_token_stream().to_string());
        assert!(true);
    }

    // #[test]
    // fn test_owner() {
    //     let result: PackageDef = parse(String::from(INPUT_OWNABLE));

    //     assert_eq!(
    //         result,
    //         PackageDef {
    //             attrs: attrs(),
    //             other_code: other_code(),
    //             class_name: ClassNameDef {
    //                 classes: vec![Class::from("Owner")],
    //             },
    //             classes: vec![ClassDef {
    //                 struct_attrs: vec![parse_quote!(#[odra::module])],
    //                 impl_attrs: vec![parse_quote!(#[odra::module])],
    //                 class: Class::from("Owner"),
    //                 path: vec![Class::from("Owner")],
    //                 variables: vec![VarDef {
    //                     ident: parse_quote! { owner },
    //                     ty: parse_quote! { odra::Variable<Option<odra::types::Address>> },
    //                 }],
    //                 functions: vec![
    //                     FnDef::Complex(ComplexFnDef {
    //                         attrs: vec![],
    //                         name: Fn::from("get_owner"),
    //                         args: vec![parse_quote!(&self)],
    //                         ret: parse_quote!(-> Option<odra::types::Address>),
    //                         implementations: vec![ClassFnImpl {
    //                             class: Some(Class::from("Owner")),
    //                             fun: Fn::from("get_owner"),
    //                             implementation: parse_quote! {{
    //                                 return odra::UnwrapOrRevert::unwrap_or_revert(self.owner.get());
    //                             }},
    //                             visibility: parse_quote!(pub),
    //                         }],
    //                     }),
    //                     FnDef::Plain(PlainFnDef {
    //                         attrs: vec![parse_quote!(#[odra(init)])],
    //                         name: Fn::from("init"),
    //                         args: vec![parse_quote!(&mut self)],
    //                         ret: parse_quote!(),
    //                         implementation: ClassFnImpl {
    //                             class: None,
    //                             fun: Fn::from("init"),
    //                             implementation: parse_quote! {{
    //                                 self.owner.set(Some(odra::contract_env::caller()));
    //                             }},
    //                             visibility: parse_quote!(pub),
    //                         },
    //                     }),
    //                     FnDef::Complex(ComplexFnDef {
    //                         attrs: vec![],
    //                         name: Fn::from("only_owner"),
    //                         args: vec![parse_quote!(&self),],
    //                         ret: parse_quote!(),
    //                         implementations: vec![ClassFnImpl {
    //                             class: Some(Class::from("Owner")),
    //                             fun: Fn::from("only_owner"),
    //                             implementation: parse_quote!({
    //                                 if !(Some(odra::contract_env::caller())
    //                                     == odra::UnwrapOrRevert::unwrap_or_revert(self.owner.get()))
    //                                 {
    //                                     odra::contract_env::revert(
    //                                         odra::types::ExecutionError::new(
    //                                             1u16,
    //                                             "Only the contract owner can call this function.",
    //                                         ),
    //                                     )
    //                                 };
    //                             }),
    //                             visibility: parse_quote!(pub),
    //                         }],
    //                     }),
    //                     FnDef::Complex(ComplexFnDef {
    //                         attrs: vec![],
    //                         name: Fn::from("transfer_ownership"),
    //                         args: vec![
    //                             parse_quote!(&mut self),
    //                             parse_quote!(new_owner: Option<odra::types::Address>)
    //                         ],
    //                         ret: parse_quote!(),
    //                         implementations: vec![ClassFnImpl {
    //                             class: Some(Class::from("Owner")),
    //                             fun: Fn::from("transfer_ownership"),
    //                             implementation: parse_quote! {{
    //                                 self.only_owner();
    //                                 self.owner.set(new_owner);
    //                             }},
    //                             visibility: parse_quote!(pub),
    //                         }],
    //                     }),
    //                 ],
    //             }],
    //         }
    //     );
    // }
}
