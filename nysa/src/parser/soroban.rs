use super::{context::GlobalContext, Parser};
use crate::{model::ir::Package, utils::AsStringVec, ParserError};
use proc_macro2::TokenStream;

/// Implementation of [Parser]. Generates code compatible with the Soroban Framework.
pub struct SorobanParser;

impl Parser for SorobanParser {
    fn parse(package: Package) -> Result<TokenStream, ParserError> {
        // register all metadata in the global context.
        let ctx = GlobalContext::new(
            package.events().as_string_vec(),
            package.interfaces().to_vec(),
            package.libraries().to_vec(),
            package.enums().as_string_vec(),
            package.errors().as_string_vec(),
            package.contracts().to_vec(),
            package.structs().to_vec(),
        );

        Ok(quote::quote! {})
    }
}
