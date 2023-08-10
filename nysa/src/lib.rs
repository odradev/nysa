#![allow(unused_variables)]
#![feature(box_patterns)]

use c3_lang_parser::c3_ast::PackageDef;
use model::ContractData;
use parser::Parser;

#[cfg(feature = "builder")]
pub mod builder;
mod linearization;
mod model;
mod parser;
mod utils;

pub use parser::odra::OdraParser;

/// Parses solidity code into a C3 linearized, Parser compatible ast (eg. Odra)
///
/// Example:
///
/// ```rust
/// # use quote::ToTokens;
/// # use nysa::OdraParser;
///
/// fn to_odra(solidity_code: String) {
///     let c3_ast = nysa::parse::<OdraParser>(solidity_code);
///     let code = c3_ast.to_token_stream().to_string();
///     // ...
///     // more logic
/// }
///
/// ```
pub fn parse<P: Parser>(input: String) -> PackageDef {
    let solidity_ast = utils::ast::parse(&input);
    let contract_data = ContractData::new(&solidity_ast);

    <P as Parser>::parse(&contract_data)
}
