#![allow(unused_variables)]

use c3_lang_linearization::C3;
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use model::ContractData;
use solidity_parser::pt::{ContractDefinition, SourceUnitPart};
use syn::{parse_quote, Item, Attribute};
use utils::classes;

#[cfg(feature = "builder")]
pub mod builder;
mod event;
mod expr;
mod func;
mod linearization;
mod model;
mod stmt;
mod ty;
mod utils;
mod var;

use std::{collections::HashMap, sync::Mutex};

lazy_static::lazy_static! {
    static ref ERROR_MAP: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
    static ref ERRORS: Mutex<u16> = Mutex::new(0);
}

/// Parses solidity code into a C3 linearized, near compatible ast
pub fn parse(input: String) -> PackageDef {
    let solidity_ast = parse_to_solidity_ast(&input);
    let contracts: Vec<&ContractDefinition> = utils::extract_contracts(&solidity_ast);
    let top_level_contract = contracts.last().expect("Contract not found");

    let c3 = linearization::c3_linearization(&contracts);
    let base_contracts = get_base_contracts(top_level_contract, &contracts, &c3);

    let contract_data = ContractData::new(top_level_contract, base_contracts, c3);

    let attrs = attrs();
    let other_code = other_code();
    let class_name = contract_data.c3_class_name_def();
    let mut classes = vec![];
    classes.extend(event::events_def(&solidity_ast));
    classes.push(contract_def(&contract_data));
    PackageDef {
        attrs,
        other_code,
        class_name,
        classes,
    }
}

pub(crate) fn parse_to_solidity_ast(input: &str) -> Vec<SourceUnitPart> {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: Vec<SourceUnitPart> = solidity_ast.0 .0;
    solidity_ast
}

fn get_base_contracts<'a>(
    top_lvl_contract: &'a ContractDefinition,
    contracts: &'a [&ContractDefinition],
    c3: &C3,
) -> Vec<&'a ContractDefinition> {
    let classes = classes(top_lvl_contract, c3)
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();
    contracts
        .iter()
        .filter(|c| classes.contains(&c.name.name))
        .map(|c| *c)
        .collect::<Vec<_>>()
}

/// Builds a c3 contract class definition
fn contract_def(data: &ContractData) -> ClassDef {
    let variables = var::variables_def(data);
    let functions = func::functions_def(data);

    ClassDef {
        struct_attrs: vec![parse_quote!(#[odra::module])],
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
fn other_code() -> Vec<Item> {
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
            "../../examples/owned-token/nysa/src/owned_token.sol"
        )));
        dbg!(result.to_token_stream().to_string());
        assert!(true);
    }

    #[test]
    fn test_owner() {
        let result: PackageDef = parse(String::from(INPUT_OWNABLE));

        assert_eq!(
            result,
            PackageDef {
                attrs: attrs(),
                other_code: other_code(),
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
                                    return odra::UnwrapOrRevert::unwrap_or_revert(self.owner.get());
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
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("only_owner"),
                            args: vec![parse_quote!(&self),],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("Owner")),
                                fun: Fn::from("only_owner"),
                                implementation: parse_quote!({
                                    if !(Some(odra::contract_env::caller())
                                        == odra::UnwrapOrRevert::unwrap_or_revert(self.owner.get()))
                                    {
                                        odra::contract_env::revert(
                                            odra::types::ExecutionError::new(
                                                1u16,
                                                "Only the contract owner can call this function.",
                                            ),
                                        )
                                    };
                                }),
                                visibility: parse_quote!(pub),
                            }],
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
                                    self.only_owner();
                                    self.owner.set(new_owner);
                                }},
                                visibility: parse_quote!(pub),
                            }],
                        }),
                    ],
                }],
            }
        );
    }
}
