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

#[allow(dead_code)]
const TEST_INPUT: &str = r#"
contract Owner {
    address private _owner;
    mapping(address => mapping(address => uint256)) private _allowances;
    mapping(address => uint256) private _balances;

    function allowance(address owner, address spender) public view virtual returns (uint256) {
        _balances[to] += value;
    }
}
"#;

#[test]
fn test_parser() {
    // let result: PackageDef = parse::<OdraParser>(String::from(include_str!(
    //     "../../../../resources/plascoin.sol"
    // )));
    let result: PackageDef = parse::<OdraParser>(TEST_INPUT.to_string());
    dbg!(result.classes.first().to_token_stream().to_string());
    assert!(true);
}

#[test]
fn test_owner() {
    let result: PackageDef = parse::<OdraParser>(String::from(INPUT_OWNABLE));

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
                                    odra::contract_env::revert(odra::types::ExecutionError::new(
                                        1u16,
                                        "Only the contract owner can call this function.",
                                    ))
                                };
                            }),
                            visibility: parse_quote!(),
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
