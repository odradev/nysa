use proc_macro2::TokenStream;

use crate::{model::ir::Package, ParserError};

mod context;
pub mod odra;

/// Type that converts a pre-processed `data` into `PackageDef`.
pub trait Parser {
    fn parse(package: Package) -> Result<TokenStream, ParserError>;
}
