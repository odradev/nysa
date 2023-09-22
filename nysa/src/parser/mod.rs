use proc_macro2::TokenStream;

use crate::{model::ir::Package, ParserError};

pub mod context;
pub mod odra;

/// Type that converts a pre-processed `package` into [TokenStream].
pub trait Parser {
    /// Parses pre-processed data into [TokenStream]. If an error occurs, the first encountered [ParserError] error is returned.
    fn parse(package: Package) -> Result<TokenStream, ParserError>;
}
