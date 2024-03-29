#![allow(unused_variables)]
#![feature(box_patterns, int_roundings)]

#[cfg(feature = "builder")]
pub mod builder;
mod c3;
mod error;
mod model;
mod parser;
mod utils;

pub use error::ParserError;
pub use parser::{odra::OdraParser, Parser};
use proc_macro2::TokenStream;

/// Parses solidity code into a [TokenStream], [Parser] compatible ast (eg. Odra)
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
    let solidity_ast = utils::ast::parse(input).expect("The input should be a valid solidity code");

    let package =
        parser::preprocess(&solidity_ast).expect("The ast should allow to create a valid Package");

    <P as Parser>::parse(package).unwrap()
}
