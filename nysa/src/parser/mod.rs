use proc_macro2::TokenStream;
use solidity_parser::pt::ContractDefinition;

use crate::{
    c3,
    model::{
        ir::{Expression, InterfaceData, Struct},
        ContractData,
    },
    utils::{ast, map_collection, SolidityAST},
};
use crate::{model::ir::Package, ParserError};

pub mod context;
pub mod odra;

/// Type that converts a pre-processed `package` into [TokenStream].
pub trait Parser {
    /// Parses pre-processed data into [TokenStream]. If an error occurs, the first encountered [ParserError] error is returned.
    fn parse(package: Package) -> Result<TokenStream, ParserError>;
}

pub(crate) fn preprocess(solidity_ast: &SolidityAST) -> Result<Package, ParserError> {
    let contracts: Vec<&ContractDefinition> = ast::extract_contracts(solidity_ast);
    let c3 = c3::linearization(&contracts);

    let contract_classes =
        c3::find_top_level_contracts(&contracts, &c3).expect("At least one contract expected");

    let interfaces = ast::extract_interfaces(&contracts)
        .iter()
        .map(|i| InterfaceData::new(i))
        .collect::<Vec<_>>();

    let events = map_collection(ast::extract_events(solidity_ast));
    let errors = map_collection(ast::extract_errors(solidity_ast));
    let enums = map_collection(ast::extract_enums(solidity_ast));
    let structs = ast::extract_structs(solidity_ast)
        .into_iter()
        .map(|(namespace, def)| {
            let name = def.name.name.to_owned();
            let fields = def
                .fields
                .iter()
                .map(|v| (v.name.name.to_owned(), Expression::from(&v.ty)))
                .collect();
            Struct {
                name,
                fields,
                namespace,
            }
        })
        .collect();

    let contracts = contract_classes
        .iter()
        .map(|class| {
            ContractData::try_from((class, &contracts))
                .expect("The ast should allow to create a valid PackageDef")
        })
        .collect();

    Ok(Package::new(
        contracts, events, errors, enums, interfaces, structs,
    ))
}
