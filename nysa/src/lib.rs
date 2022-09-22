use c3_lang_linearization::{Class, Fn};
use c3_lang_parser::{c3_ast::{PackageDef, ClassNameDef, ClassDef, VarDef, ClassFnImpl, FnDef}};
use syn::parse_quote;


pub fn parse(input: String) {
    let solidty_ast = solidity_parser::parse(&input, 0).unwrap();
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
        let a = expected().to_token_stream();
        // let result = parse(String::from(input));
    }
}
