use c3_lang_linearization::{Class, Fn, Var};
use c3_lang_parser::{c3_ast::{PackageDef, ClassNameDef, ClassDef, VarDef, ClassFnImpl, FnDef}};
use proc_macro2::TokenStream;
use quote::format_ident;
use solidity_parser::pt::{self, SourceUnitPart, ContractDefinition, VariableDefinition, ContractPart, Expression};
use syn::{parse_quote, Item};


pub fn parse(input: String) -> PackageDef {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: &SourceUnitPart = &solidity_ast.0.0[0];
    let contract: &ContractDefinition = 
    if let SourceUnitPart::ContractDefinition(contract_def) = solidity_ast {
        contract_def
    } else {
        panic!("Not a contract def.")
    };

    let other_code = other_code();
    let class_name = class_name_def(&contract);
    let classes = vec![class_def(&contract)];
    PackageDef { other_code, class_name, classes }
}

fn class_name_def(contract: &ContractDefinition) -> ClassNameDef {
    ClassNameDef { classes: vec![class(contract)] }
}

fn class(contract: &ContractDefinition) -> Class {
    Class::from(contract.name.name.clone())
}

fn class_def(contract: &ContractDefinition) -> ClassDef {
    let variables = variables_def(contract);
    let functions = vec![];

    ClassDef { 
        struct_attrs: vec![
            parse_quote! { #[near_sdk::near_bindgen] },
            parse_quote! { #[derive(Default, BorshDeserialize, BorshSerialize)] }
        ], 
        impl_attrs: vec![
            parse_quote! { #[near_sdk::near_bindgen] },
        ],
        class: class(contract),
        path: vec![class(contract)],
        variables,
        functions
    }
}

fn variables_def(contract: &ContractDefinition) -> Vec<VarDef> {
    let mut result = Vec::new();
    for maybe_var in &contract.parts {
        if let ContractPart::VariableDefinition(var_def) = maybe_var {
            result.push(variable_def(&var_def));
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
        _ => panic!("Not a type.")
    }
}

fn parse_type(ty: &pt::Type) -> syn::Type {
    match ty {
        pt::Type::Mapping(_, key, value) => {
            let key = parse_type_from_expr(&key);
            let value = parse_type_from_expr(&value);
            parse_quote!{
                std::collections::HashMap<#key, #value>
            }
        },
        pt::Type::Address => parse_quote!( near_sdk:: AccountId ),
        pt::Type::String => parse_quote!( String ),
        _ => panic!("Unexpected type.")
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
        }
    ]
}

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
            }
        ],
        class_name: ClassNameDef {
            classes: vec![Class::from("StatusMessage")],
        },
        classes: vec![ClassDef { 
            struct_attrs: vec![
                parse_quote! { #[near_sdk::near_bindgen] },
                parse_quote! { #[derive(Default, BorshDeserialize, BorshSerialize)] }
            ], 
            impl_attrs: vec![
                parse_quote! { #[near_sdk::near_bindgen] },
            ],
            class: Class::from("StatusMessage"),
            path: vec![Class::from("StatusMessage")],
            variables: vec![VarDef {
                ident: parse_quote! { records },
                ty: parse_quote! { std::collections::HashMap<near_sdk::AccountId, String> },
            }],
            functions: vec![
                FnDef {
                    attrs: vec![
                        parse_quote! { #[payable] }
                    ],
                    name: Fn::from("set_status"),
                    args: vec![parse_quote! { &mut self }, parse_quote! { message: String }],
                    ret: parse_quote! { -> () },
                    implementations: vec![
                        ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("set_status"),
                            implementation: parse_quote! {{
                                let account_id = near_sdk::env::signer_account_id();
                                self.records.insert(account_id, message);
                            }},
                        },
                    ],
                },
                FnDef {
                    attrs: vec![],
                    name: Fn::from("get_status"),
                    args: vec![parse_quote! { &self }, parse_quote! { account_id: near_sdk::AccountId }],
                    ret: parse_quote! { -> String },
                    implementations: vec![
                        ClassFnImpl {
                            class: Class::from("StatusMessage"),
                            fun: Fn::from("get_status"),
                            implementation: parse_quote! {{
                                self.records.get(&account_id).cloned().unwrap_or_default()
                            }},
                        },
                    ],
                },
            ]
        }]
    }
}

#[cfg(test)]
mod tests {

    use c3_lang_parser::c3_ast::PackageDef;
    use pretty_assertions::assert_eq;
    use quote::ToTokens;

    use crate::{parse, expected};

    const input: &str = r#"
    contract StatusMessage {
        mapping(address => string) records;

        function set_status(string status) public {
            address account_id = msg.sender;
            records[account_id] = status;
        }

        function get_status(address account_id) public view returns (string memory) {
            return records[account_id];
        }
    }
    "#;

    #[test]
    fn test_parser() {
        let result: PackageDef = parse(String::from(input));
        assert_eq!(result, expected());
    }
}
