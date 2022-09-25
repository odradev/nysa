#![allow(unused_variables)]

use c3_lang_linearization::{Class, Fn};
use c3_lang_parser::c3_ast::{ClassDef, ClassFnImpl, ClassNameDef, FnDef, PackageDef, VarDef};
use quote::format_ident;
use solidity_parser::pt::{
    self, ContractDefinition, ContractPart, Expression, FunctionDefinition, SourceUnitPart,
    VariableDefinition,
};
use syn::{parse_quote, FnArg, Item};

mod functions;

pub fn parse(input: String) -> PackageDef {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: &SourceUnitPart = &solidity_ast.0 .0[0];
    let contract: &ContractDefinition =
        if let SourceUnitPart::ContractDefinition(contract_def) = solidity_ast {
            contract_def
        } else {
            panic!("Not a contract def.")
        };

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
    let variables = variables_def(contract);
    let functions = functions_def(contract);

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

fn functions_def(contract: &ContractDefinition) -> Vec<FnDef> {
    let class: Class = class(contract);
    contract
        .parts
        .iter()
        .filter_map(|part| match part {
            ContractPart::FunctionDefinition(func) => Some(func),
            _ => None,
        })
        .filter(|func| func.name.is_some())
        .map(|func| function_def(func, class.clone()))
        .collect::<Vec<_>>()
}

fn function_def(func: &FunctionDefinition, class: Class) -> FnDef {
    functions::check_function_type(&func.ty);

    let name: Fn = func.name.to_owned().unwrap().name.into();

    let mut args: Vec<FnArg> = func
        .params
        .iter()
        .filter_map(|p| p.1.as_ref())
        .map(functions::parse_parameter)
        .collect();

    if let Some(receiver) = functions::parse_attrs_to_receiver_param(&func.attributes) {
        args.insert(0, receiver);
    }

    let attrs = functions::parse_attrs(&func.attributes);
    let ret = functions::parse_ret_type(func);
    let block: syn::Block = functions::parse_body(&func.body);

    FnDef {
        attrs,
        name: name.clone(),
        args,
        ret,
        implementations: vec![ClassFnImpl {
            class,
            fun: name,
            implementation: block,
        }],
    }
}

fn variables_def(contract: &ContractDefinition) -> Vec<VarDef> {
    let mut result = Vec::new();
    for maybe_var in &contract.parts {
        if let ContractPart::VariableDefinition(var_def) = maybe_var {
            result.push(variable_def(var_def));
        }
    }
    result
}

fn variable_def(v: &VariableDefinition) -> VarDef {
    let ident: proc_macro2::Ident = format_ident!("{}", v.name.name);
    let ty = parse_type_from_expr(&v.ty);
    VarDef { ident, ty }
}

fn parse_type_from_expr(ty: &Expression) -> syn::Type {
    match ty {
        Expression::Type(_, ty) => parse_type(ty),
        _ => panic!("Not a type."),
    }
}

fn parse_type(ty: &pt::Type) -> syn::Type {
    match ty {
        pt::Type::Mapping(_, key, value) => {
            let key = parse_type_from_expr(key);
            let value = parse_type_from_expr(value);
            parse_quote! {
                std::collections::HashMap<#key, #value>
            }
        }
        pt::Type::Address => parse_quote!(near_sdk::AccountId),
        pt::Type::AddressPayable => parse_quote!(near_sdk::AccountId),
        pt::Type::String => parse_quote!(String),
        pt::Type::Bool => parse_quote!(bool),
        pt::Type::Int(_) => parse_quote!(i16),
        pt::Type::Uint(_) => parse_quote!(u16),
        _ => panic!("Unsupported type."),
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

    pub fn expected() -> PackageDef {
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
        // assert!(false);
        assert_eq!(result, expected());
    }
}
