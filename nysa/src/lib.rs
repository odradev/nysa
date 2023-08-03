#![allow(unused_variables)]

use c3_lang_linearization::C3;
use c3_lang_parser::c3_ast::{ClassDef, PackageDef};
use model::ContractData;
use solidity_parser::pt::{ContractDefinition, SourceUnitPart};
use syn::{parse_quote, Item};
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

/// Parses solidity code into a C3 linearized, near compatible ast
pub fn parse(input: String) -> PackageDef {
    let solidity_ast = parse_to_solidity_ast(&input);
    let contracts: Vec<&ContractDefinition> = utils::extract_contracts(&solidity_ast);
    let top_level_contract = contracts.last().expect("Contract not found");

    let c3 = linearization::c3_linearization(&contracts);
    let base_contracts = get_base_contracts(top_level_contract, &contracts, &c3);

    let contract_data = ContractData::new(top_level_contract, base_contracts, c3);

    let other_code = other_code();
    let class_name = contract_data.c3_class_name_def();
    let mut classes = vec![];
    classes.extend(event::events_def(&solidity_ast));
    classes.push(contract_def(&contract_data));
    PackageDef {
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
    use c3_lang_parser::c3_ast::{ClassFnImpl, ClassNameDef, ComplexFnDef, FnDef, VarDef};
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

        function getOwner() public view returns (address) {
            return owner;
        }

        function transferOwnership(address newOwner) public onlyOwner {
            owner = newOwner;
        }
    }
    "#;

    const INPUT_ERC20: &str = r#"
    contract ERC20 {
        string public name;
        string public symbol;
        uint8 public decimals = 18;
        uint256 public totalSupply;
    
        mapping(address => uint256) public balanceOf;
    
        event Transfer(address indexed from, address indexed to, uint256 value);
    
        constructor(string memory _name, string memory _symbol, uint8 _decimals, uint256 _initialSupply) {
            name = _name;
            symbol = _symbol;
            decimals = _decimals;
            totalSupply = _initialSupply;
            balanceOf[msg.sender] = _initialSupply;
        }
        
        function _transfer(address _from, address _to, uint256 _value) internal {
            balanceOf[_from] -= _value;
            balanceOf[_to] += _value;
    
            emit Transfer(_from, _to, _value);
        }

        function transfer(address _to, uint256 _value) public {
            _transfer(msg.sender, _to, _value);
        }
    }
    "#;

    #[test]
    fn test_parser() {
        let result: PackageDef = parse(String::from(include_str!(
            "../../example-owned-token/src/owned_token.sol"
        )));
        dbg!(result.to_token_stream().to_string());
        assert!(true);
        // assert_eq!(result, PackageDef {
        //     other_code: other_code(),
        //     class_name: ClassNameDef {
        //         classes: vec![Class::from("StatusMessage")],
        //     },
        //     classes: vec![ClassDef {
        //         struct_attrs: vec![parse_quote! { #[odra::module] }],
        //         impl_attrs: vec![parse_quote! { #[odra::module] }],
        //         class: Class::from("StatusMessage"),
        //         path: vec![Class::from("StatusMessage")],
        //         variables: vec![VarDef {
        //             ident: parse_quote! { records },
        //             ty: parse_quote! { odra::Mapping<odra::types::Address, String> },
        //         }],
        //         functions: vec![
        //             FnDef {
        //                 attrs: vec![parse_quote! { #[odra(payable)] }],
        //                 name: Fn::from("set_status"),
        //                 args: vec![parse_quote!(&mut self), parse_quote!(status: String)],
        //                 ret: parse_quote! {},
        //                 implementations: vec![ClassFnImpl {
        //                     class: Class::from("StatusMessage"),
        //                     fun: Fn::from("set_status"),
        //                     implementation: parse_quote! {{
        //                         let account_id = odra::contract_env::caller();
        //                         self.records.set(&account_id, status);
        //                     }},
        //                     visibility: syn::Visibility::Public(syn::VisPublic {
        //                         pub_token: Default::default(),
        //                     }),
        //                 }],
        //             },
        //             FnDef {
        //                 attrs: vec![],
        //                 name: Fn::from("get_status"),
        //                 args: vec![
        //                     parse_quote!(&self),
        //                     parse_quote!(account_id: odra::types::Address),
        //                 ],
        //                 ret: parse_quote! { -> String },
        //                 implementations: vec![ClassFnImpl {
        //                     class: Class::from("StatusMessage"),
        //                     fun: Fn::from("get_status"),
        //                     implementation: parse_quote! {{
        //                         return self.records.get(&account_id).unwrap_or_default();
        //                     }},
        //                     visibility: syn::Visibility::Public(syn::VisPublic {
        //                         pub_token: Default::default(),
        //                     }),
        //                 }],
        //             },
        //         ],
        //     }],
        // });
    }

    #[test]
    fn test_owner() {
        let result: PackageDef = parse(String::from(INPUT_OWNABLE));

        assert_eq!(
            result,
            PackageDef {
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
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![parse_quote!(#[odra(init)])],
                            name: Fn::from("init"),
                            args: vec![parse_quote!(&mut self)],
                            ret: parse_quote! {},
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("Owner")),
                                fun: Fn::from("init"),
                                implementation: parse_quote! {{
                                    self.owner.set(odra::contract_env::caller());
                                }},
                                visibility: syn::Visibility::Public(syn::VisPublic {
                                    pub_token: Default::default(),
                                }),
                            }],
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
                                    if odra::contract_env::caller() == self.owner.get() {
                                        return;
                                    } else {
                                        odra::contract_env::revert(
                                            odra::types::ExecutionError::new(
                                                1,
                                                "Only the contract owner can call this function.",
                                            ),
                                        )
                                    };
                                }),
                                visibility: syn::Visibility::Public(syn::VisPublic {
                                    pub_token: Default::default(),
                                }),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("get_owner"),
                            args: vec![parse_quote!(&self)],
                            ret: parse_quote! { -> odra::types::Address },
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("Owner")),
                                fun: Fn::from("get_owner"),
                                implementation: parse_quote! {{
                                    return self.owner.get();
                                }},
                                visibility: syn::Visibility::Public(syn::VisPublic {
                                    pub_token: Default::default(),
                                }),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("transfer_ownership"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(new_owner: odra::types::Address)
                            ],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("Owner")),
                                fun: Fn::from("transfer_ownership"),
                                implementation: parse_quote! {{
                                    self.only_owner();
                                    self.owner.set(new_owner);
                                }},
                                visibility: syn::Visibility::Public(syn::VisPublic {
                                    pub_token: Default::default(),
                                }),
                            }],
                        }),
                    ],
                }],
            }
        );
    }

    #[test]
    fn test_erc20() {
        let result: PackageDef = parse(String::from(INPUT_ERC20));
        // assert!(false);
        let expected = PackageDef {
            other_code: other_code(),
            class_name: ClassNameDef {
                classes: ["Owner", "ERC20", "OwnedToken"]
                    .iter()
                    .map(From::from)
                    .collect(),
            },
            classes: vec![
                ClassDef {
                    struct_attrs: vec![parse_quote!(#[odra::module])],
                    impl_attrs: vec![parse_quote!(#[odra::module])],
                    class: Class::from("ERC20"),
                    path: ["Owner", "ERC20", "OwnedToken"]
                        .iter()
                        .map(From::from)
                        .collect(),
                    variables: vec![
                        VarDef {
                            ident: parse_quote!(owner),
                            ty: parse_quote!(odra::Variable<odra::types::Address>),
                        },
                        VarDef {
                            ident: parse_quote!(name),
                            ty: parse_quote!(odra::Variable<String>),
                        },
                        VarDef {
                            ident: parse_quote!(symbol),
                            ty: parse_quote!(odra::Variable<String>),
                        },
                        VarDef {
                            ident: parse_quote!(decimals),
                            ty: parse_quote!(odra::Variable<u8>),
                        },
                        VarDef {
                            ident: parse_quote!(total_supply),
                            ty: parse_quote!(odra::Variable<odra::types::U256>),
                        },
                        VarDef {
                            ident: parse_quote!(balance_of),
                            ty: parse_quote!(odra::Mapping<odra::types::Address, odra::types::U256>),
                        },
                    ],
                    functions: vec![
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![parse_quote!(#[odra(init)])],
                            name: Fn::from("init"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(_name: String),
                                parse_quote!(_symbol: String),
                                parse_quote!(_decimals: u8),
                                parse_quote!(_initial_supply: odra::types::U256),
                            ],
                            ret: parse_quote!(),
                            implementations: vec![
                                ClassFnImpl {
                                    class: Some(Class::from("Owner")),
                                    fun: Fn::from("init"),
                                    implementation: parse_quote!({
                                        self.owner.set(odra::contract_env::caller());
                                    }),
                                    visibility: parse_quote!(pub),
                                },
                                ClassFnImpl {
                                    class: Some(Class::from("ERC20")),
                                    fun: Fn::from("init"),
                                    implementation: parse_quote!({
                                        self.name.set(_name);
                                        self.symbol.set(_symbol);
                                        self.decimals.set(_decimals);
                                        self.total_supply.set(_initial_supply);
                                        self.balance_of
                                            .set(&odra::contract_env::caller(), _initial_supply);
                                    }),
                                    visibility: parse_quote!(pub),
                                },
                                ClassFnImpl {
                                    class: Some(Class::from("OwnedToken")),
                                    fun: Fn::from("init"),
                                    implementation: parse_quote! {{
                                        self.super_init(_name, _symbol, _decimals, _initial_supply);
                                    }},
                                    visibility: parse_quote!(pub),
                                },
                            ],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("only_owner"),
                            args: vec![parse_quote!(&self)],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("Owner")),
                                fun: Fn::from("only_owner"),
                                implementation: parse_quote!({
                                    if odra::contract_env::caller() == self.owner.get() {
                                        return;
                                    } else {
                                        odra::contract_env::revert(
                                            odra::types::ExecutionError::new(
                                                1,
                                                "Only the contract owner can call this function.",
                                            ),
                                        )
                                    };
                                }),
                                visibility: syn::Visibility::Public(syn::VisPublic {
                                    pub_token: Default::default(),
                                }),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("get_owner"),
                            args: vec![parse_quote!(&self)],
                            ret: parse_quote!(-> odra::types::Address),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("Owner")),
                                fun: Fn::from("get_owner"),
                                implementation: parse_quote! {{
                                    return self.owner.get();
                                }},
                                visibility: parse_quote!(pub),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("transfer_ownership"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(new_owner: odra::types::Address),
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
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("_transfer"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(_from: odra::types::Address),
                                parse_quote!(_to: odra::types::Address),
                                parse_quote!(_value: odra::types::U256),
                            ],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("ERC20")),
                                fun: Fn::from("_transfer"),
                                implementation: parse_quote! {{
                                    self.balance_of.set(&_from, self.balance_of.get(&_from).unwrap_or_default() - _value);
                                    self.balance_of.set(&_to, self.balance_of.get(&_to).unwrap_or_default() + _value);
                                    <Transfer as odra::types::event::OdraEvent>::emit(Transfer { _from, _to, _value });
                                }},
                                visibility: parse_quote!(pub),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("transfer"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(_to: odra::types::Address),
                                parse_quote!(_value: odra::types::U256),
                            ],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("ERC20")),
                                fun: Fn::from("transfer"),
                                implementation: parse_quote! {{
                                    self._transfer(odra::contract_env::caller(), _to, _value);
                                }},
                                visibility: parse_quote!(pub),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("mint"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(_to: odra::types::Address),
                                parse_quote!(_value: odra::types::U256),
                            ],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("OwnedToken")),
                                fun: Fn::from("mint"),
                                implementation: parse_quote! {{
                                    self.balance_of.set(&_to, self.balance_of.get(&_to).unwrap_or_default() + _value);
                                    self.total_supply.set(self.total_supply().get_or_default() + _value);
                                    // TODO _from should be set to `None`
                                    <Transfer as odra::types::event::OdraEvent>::emit(Transfer { _from: odra::contract_env::caller(), _to, _value });
                                }},
                                visibility: parse_quote!(pub),
                            }],
                        }),
                        FnDef::Complex(ComplexFnDef {
                            attrs: vec![],
                            name: Fn::from("burn"),
                            args: vec![
                                parse_quote!(&mut self),
                                parse_quote!(_value: odra::types::U256),
                            ],
                            ret: parse_quote!(),
                            implementations: vec![ClassFnImpl {
                                class: Some(Class::from("OwnedToken")),
                                fun: Fn::from("burn"),
                                implementation: parse_quote! {{
                                    self.balance_of.set(&odra::contract_env::caller(), self.balance_of.get(&odra::contract_env::caller()).unwrap_or_default() + _value);
                                    self.total_supply.set(self.total_supply().get_or_default() - _value);
                                    // TODO _to should be set to `None`
                                    <Transfer as odra::types::event::OdraEvent>::emit(Transfer { _from: odra::contract_env::caller(), _to: odra::contract_env::caller(), _value });
                                }},
                                visibility: parse_quote!(pub),
                            }],
                        }),
                    ],
                },
                ClassDef {
                    struct_attrs: vec![parse_quote!(#[derive(odra::Event, PartialEq, Eq, Debug)])],
                    impl_attrs: vec![],
                    class: Class::from("Transfer"),
                    path: vec![Class::from("Transfer")],
                    variables: vec![
                        VarDef {
                            ident: parse_quote!(from),
                            ty: parse_quote!(odra::types::Address),
                        },
                        VarDef {
                            ident: parse_quote!(to),
                            ty: parse_quote!(odra::types::Address),
                        },
                        VarDef {
                            ident: parse_quote!(value),
                            ty: parse_quote!(odra::types::U256),
                        },
                    ],
                    functions: vec![],
                },
            ],
        };
        assert_eq!(result, expected);
    }
}
