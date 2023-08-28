use proc_macro2::TokenStream;

use crate::model::ContractData;

pub mod odra;

/// Type that converts a pre-processed `data` into `PackageDef`.
pub trait Parser {
    fn parse(data: Vec<ContractData>) -> TokenStream;
}
