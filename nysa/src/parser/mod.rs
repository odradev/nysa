use c3_lang_parser::c3_ast::PackageDef;

use crate::model::ContractData;

pub mod odra;

/// Type that converts a pre-processed `data` into `PackageDef`.
pub trait Parser {
    fn parse(data: ContractData) -> PackageDef;
}
