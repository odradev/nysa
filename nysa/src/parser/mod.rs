use proc_macro2::TokenStream;
use solidity_parser::pt::ContractDefinition;

use crate::{
    c3,
    model::ContractData,
    utils::{ast, map_collection, SolidityAST},
};
use crate::{model::ir::Package, ParserError};

use self::common::{
    ContractErrorParser, ContractReferenceParser, CustomElementParser, CustomImports, ErrorParser,
    EventEmitParser, EventParser, ExpressionParser, ExtContractParser, FunctionParser,
    NumberParser, StringParser, TypeParser,
};

pub mod common;
pub mod context;
pub mod odra;
pub mod soroban;
pub(crate) mod syn_utils;
#[cfg(test)]
pub(crate) mod test_utils;

/// Type that converts a pre-processed `package` into [TokenStream].
pub trait Parser {
    type EventEmitParser: EventEmitParser;
    type ContractReferenceParser: ContractReferenceParser;
    type ContractErrorParser: ContractErrorParser;
    type ExpressionParser: ExpressionParser + StringParser + NumberParser;
    type FnParser: FunctionParser;
    type TypeParser: TypeParser;
    type ElementsParser: CustomElementParser
        + ExtContractParser
        + EventParser
        + ErrorParser
        + CustomImports;

    /// Parses pre-processed data into [TokenStream]. If an error occurs, the first encountered [ParserError] error is returned.
    fn parse(package: Package) -> Result<TokenStream, ParserError>;
}

pub(crate) fn preprocess(solidity_ast: &SolidityAST) -> Result<Package, ParserError> {
    let contracts: Vec<&ContractDefinition> = ast::extract_contracts(solidity_ast);
    let c3 = c3::linearization(&contracts);

    let top_lvl_classes =
        c3::find_top_level_contracts(&contracts, &c3).expect("At least one contract expected");

    let interfaces = map_collection(ast::extract_interfaces(&contracts));

    let events = map_collection(ast::extract_events(solidity_ast));
    let errors = map_collection(ast::extract_errors(solidity_ast));
    let enums = map_collection(ast::extract_enums(solidity_ast));
    let structs = ast::extract_structs(solidity_ast)
        .into_iter()
        .map(From::from)
        .collect();

    let contracts = top_lvl_classes
        .iter()
        .map(|class| ContractData::try_from((class, &contracts)))
        .collect::<Result<_, _>>()
        .expect("The ast should allow to create a valid PackageDef");

    Ok(Package::new(
        contracts, events, errors, enums, interfaces, structs,
    ))
}
