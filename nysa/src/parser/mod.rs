use c3_lang_parser::c3_ast::PackageDef;

use crate::model::ContractData;

pub mod odra;

pub trait Parser {
    fn parse(data: &ContractData) -> PackageDef;
}
