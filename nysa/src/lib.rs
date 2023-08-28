#![allow(unused_variables)]
#![feature(box_patterns)]

use model::ContractData;
use parser::Parser;

#[cfg(feature = "builder")]
pub mod builder;
mod c3;
mod model;
mod parser;
mod utils;

pub use parser::odra::OdraParser;
use proc_macro2::TokenStream;
use solidity_parser::pt::ContractDefinition;
use utils::ast;

/// Parses solidity code into a C3 linearized, Parser compatible ast (eg. Odra)
///
/// Example:
///
/// ```rust
/// # use quote::ToTokens;
/// # use proc_macro2::TokenStream;
/// # use nysa::OdraParser;
///
/// fn to_odra(solidity_code: String) {
///     let code: TokenStream = nysa::parse::<OdraParser, _>(solidity_code);
///     // ...
///     // more logic
/// }
///
/// ```
pub fn parse<P: Parser, I: AsRef<str>>(input: I) -> TokenStream {
    let solidity_ast =
        utils::ast::parse(input.as_ref()).expect("The input should be a valid solidity code");

    let contracts: Vec<&ContractDefinition> = ast::extract_contracts(&solidity_ast);
    let c3 = c3::linearization(&contracts);

    let contract_classes =
        c3::find_top_level_contracts(&contracts, &c3).expect("At least one contract expected");

    let data = contract_classes
        .iter()
        .map(|class| {
            ContractData::try_from((class, &solidity_ast))
                .expect("The ast should allow to create a valid PackageDef")
        })
        .collect();

    <P as Parser>::parse(data)
}
