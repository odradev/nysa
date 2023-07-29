#![allow(unused_variables)]

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, ClassNameDef, PackageDef};
use solidity_parser::pt::{ContractDefinition, SourceUnitPart};
use syn::{parse_quote, Item};

#[cfg(feature = "builder")]
pub mod builder;
mod expr;
mod func;
mod stmt;
mod ty;
mod utils;
mod var;

/// Parses solidity code into a C3 linearized, near compatible ast
pub fn parse(input: String) -> PackageDef {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: &Vec<SourceUnitPart> = &solidity_ast.0 .0;
    let contract = solidity_ast
        .iter()
        .filter_map(|unit| match unit {
            SourceUnitPart::ContractDefinition(contract) => Some(contract),
            _ => None,
        })
        .next()
        .expect("Contract not found");

    let other_code = other_code();
    let class_name = class_name_def(contract);
    let classes = vec![class_def(contract)];
    PackageDef {
        other_code,
        class_name,
        classes,
    }
}

/// Extracts contract name and wraps with c3 ast abstraction.
///
/// May contain one or more class name
fn class_name_def(contract: &ContractDefinition) -> ClassNameDef {
    ClassNameDef {
        classes: vec![class(contract)],
    }
}

/// Extracts contract name and wraps with c3 ast abstraction.
fn class(contract: &ContractDefinition) -> Class {
    Class::from(contract.name.name.clone())
}

/// Builds a c3 contract class definition
fn class_def(contract: &ContractDefinition) -> ClassDef {
    let variables = var::variables_def(contract);
    let functions = func::functions_def(contract, &variables);

    ClassDef {
        struct_attrs: vec![parse_quote! { #[odra::module] }],
        impl_attrs: vec![parse_quote! { #[odra::module] }],
        class: class(contract),
        path: vec![class(contract)],
        variables,
        functions,
    }
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
    use c3_lang_parser::c3_ast::{ClassFnImpl, FnDef, VarDef};
    use pretty_assertions::assert_eq;
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    const INPUT_STATUS_MESSAGE: &str = r#"
    contract StatusMessage {
        mapping(address => string) records;

        function set_status(string status) public payable {
            address account_id = msg.sender;
            records[account_id] = status;
        }

        function get_status(address account_id) public view returns (string memory) {
            return records[account_id];
        }
    }
    "#;

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
    }
    "#;

    #[test]
    fn test_parser() {
        let result: PackageDef = parse(String::from(INPUT_STATUS_MESSAGE));
        
        assert_eq!(result, PackageDef {
            other_code: other_code(),
            class_name: ClassNameDef {
                classes: vec![Class::from("StatusMessage")],
            },
            classes: vec![ClassDef {
                struct_attrs: vec![parse_quote! { #[odra::module] }],
                impl_attrs: vec![parse_quote! { #[odra::module] }],
                class: Class::from("StatusMessage"),
                path: vec![Class::from("StatusMessage")],
                variables: vec![VarDef {
                    ident: parse_quote! { records },
                    ty: parse_quote! { odra::Mapping<odra::types::Address, String> },
                }],
                functions: vec![
                    FnDef {
                        attrs: vec![parse_quote! { #[odra(payable)] }],
                        name: Fn::from("set_status"),
                        args: vec![parse_quote!(&mut self), parse_quote!(status: String)],
                        ret: parse_quote! {},
                        implementations: vec![ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("set_status"),
                            implementation: parse_quote! {{
                                let account_id = odra::contract_env::caller();
                                self.records.set(&account_id, status);
                            }},
                            visibility: syn::Visibility::Public(syn::VisPublic {
                                pub_token: Default::default(),
                            }),
                        }],
                    },
                    FnDef {
                        attrs: vec![],
                        name: Fn::from("get_status"),
                        args: vec![
                            parse_quote!(&self),
                            parse_quote!(account_id: odra::types::Address),
                        ],
                        ret: parse_quote! { -> String },
                        implementations: vec![ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("get_status"),
                            implementation: parse_quote! {{
                                return self.records.get(&account_id).unwrap_or_default();
                            }},
                            visibility: syn::Visibility::Public(syn::VisPublic {
                                pub_token: Default::default(),
                            }),
                        }],
                    },
                ],
            }],
        });
    }

    #[test]
    fn test_owner() {
        let result: PackageDef = parse(String::from(INPUT_OWNABLE));
        let f = result.classes.get(0).unwrap();
        let f = f.functions.get(0).unwrap();
        let f = f.implementations.get(0).unwrap();
        let code = f.implementation.clone().into_token_stream().to_string();
        // dbg!(code);
        // assert!(false); 
        assert_eq!(result, PackageDef {
            other_code: other_code(),
            class_name: ClassNameDef {
                classes: vec![Class::from("Owner")],
            },
            classes: vec![ClassDef {
                struct_attrs: vec![parse_quote! { #[odra::module] }],
                impl_attrs: vec![parse_quote! { #[odra::module] }],
                class: Class::from("Owner"),
                path: vec![Class::from("Owner")],
                variables: vec![VarDef {
                    ident: parse_quote! { owner },
                    ty: parse_quote! { odra::Variable<odra::types::Address> },
                }],
                functions: vec![
                    FnDef {
                        attrs: vec![parse_quote!(#[odra(init)])],
                        name: Fn::from("init"),
                        args: vec![parse_quote!(&mut self)],
                        ret: parse_quote! {},
                        implementations: vec![ClassFnImpl {
                            class: Class::from("Owner"),
                            fun: Fn::from("init"),
                            implementation: parse_quote! {{
                                self.owner.set(odra::contract_env::caller());
                            }},
                            visibility: syn::Visibility::Public(syn::VisPublic {
                                pub_token: Default::default(),
                            }),
                        }],
                    },
                    FnDef {
                        attrs: vec![],
                        name: Fn::from("only_owner"),
                        args: vec![
                            parse_quote!(&self),
                        ],
                        ret: parse_quote!(),
                        implementations: vec![ClassFnImpl {
                            class: Class::from("Owner"),
                            fun: Fn::from("only_owner"),
                            implementation: parse_quote!({
                                if odra::contract_env::caller() == self.owner.get() {
                                    return;
                                } else {
                                    odra::contract_env::revert(odra::types::ExecutionError::new(1, "Only the contract owner can call this function."))
                                };
                            }),
                            visibility: syn::Visibility::Public(syn::VisPublic {
                                pub_token: Default::default(),
                            }),
                        }],
                    },
                ],
            }],
        });
    }
}
