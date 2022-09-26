#![allow(unused_variables)]

use c3_lang_linearization::Class;
use c3_lang_parser::c3_ast::{ClassDef, ClassNameDef, PackageDef};
use solidity_parser::pt::{ContractDefinition, SourceUnitPart};
use syn::{parse_quote, Item};

mod expr;
mod func;
mod stmt;
mod ty;
mod var;

pub fn parse(input: String) -> PackageDef {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: &Vec<SourceUnitPart> = &solidity_ast.0.0;
    let contract= solidity_ast.iter()
        .filter_map(|unit| match unit {
            SourceUnitPart::ContractDefinition(contract) => Some(contract),
            _ => None
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

fn class_name_def(contract: &ContractDefinition) -> ClassNameDef {
    ClassNameDef {
        classes: vec![class(contract)],
    }
}

fn class(contract: &ContractDefinition) -> Class {
    Class::from(contract.name.name.clone())
}

fn class_def(contract: &ContractDefinition) -> ClassDef {
    let variables = var::variables_def(contract);
    let functions = func::functions_def(contract);

    ClassDef {
        struct_attrs: vec![
            parse_quote! { #[near_sdk::near_bindgen] },
            parse_quote! { #[derive(Default, BorshDeserialize, BorshSerialize)] },
        ],
        impl_attrs: vec![parse_quote! { #[near_sdk::near_bindgen] }],
        class: class(contract),
        path: vec![class(contract)],
        variables,
        functions,
    }
}

fn other_code() -> Vec<Item> {
    vec![
        parse_quote! {
            use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
        },
        parse_quote! {
            impl BorshDeserialize for PathStack {
                fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
                    Ok(Default::default())
                }
            }
        },
        parse_quote! {
            impl BorshSerialize for PathStack {
                fn serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
                    Ok(())
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
    use syn::parse_quote;

    use super::*;

    const INPUT: &str = r#"
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

    fn expected() -> PackageDef {
        PackageDef {
            other_code: vec![
                parse_quote! {
                    use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
                },
                parse_quote! {
                    impl BorshDeserialize for PathStack {
                        fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
                            Ok(Default::default())
                        }
                    }
                },
                parse_quote! {
                    impl BorshSerialize for PathStack {
                        fn serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::io::Result<()> {
                            Ok(())
                        }
                    }
                },
            ],
            class_name: ClassNameDef {
                classes: vec![Class::from("StatusMessage")],
            },
            classes: vec![ClassDef {
                struct_attrs: vec![
                    parse_quote! { #[near_sdk::near_bindgen] },
                    parse_quote! { #[derive(Default, BorshDeserialize, BorshSerialize)] },
                ],
                impl_attrs: vec![parse_quote! { #[near_sdk::near_bindgen] }],
                class: Class::from("StatusMessage"),
                path: vec![Class::from("StatusMessage")],
                variables: vec![VarDef {
                    ident: parse_quote! { records },
                    ty: parse_quote! { std::collections::HashMap<near_sdk::AccountId, String> },
                }],
                functions: vec![
                    FnDef {
                        attrs: vec![parse_quote! { #[payable] }],
                        name: Fn::from("set_status"),
                        args: vec![parse_quote! { &mut self }, parse_quote! { status: String }],
                        ret: parse_quote! {},
                        implementations: vec![ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("set_status"),
                            implementation: parse_quote! {{
                                let account_id = near_sdk::env::signer_account_id().clone();
                                self.records.insert(account_id, status);
                            }},
                        }],
                    },
                    FnDef {
                        attrs: vec![],
                        name: Fn::from("get_status"),
                        args: vec![
                            parse_quote! { &self },
                            parse_quote! { account_id: near_sdk::AccountId },
                        ],
                        ret: parse_quote! { -> String },
                        implementations: vec![ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("get_status"),
                            implementation: parse_quote! {{
                                return self.records.get(&account_id).cloned().unwrap_or_default();
                            }},
                        }],
                    },
                ],
            }],
        }
    }

    #[test]
    fn test_parser() {
        let result: PackageDef = parse(String::from(INPUT));
        assert_eq!(result, expected());
    }
}
