#![allow(unused_variables)]
#![feature(box_patterns)]

use model::ir::Package;
use parser::Parser;

#[cfg(feature = "builder")]
pub mod builder;
mod c3;
mod model;
mod parser;
mod utils;

pub use parser::odra::OdraParser;
use proc_macro2::TokenStream;

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

    let package =
        Package::new(solidity_ast).expect("The ast should allow to create a valid Package");

    <P as Parser>::parse(package)
}
