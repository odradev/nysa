use solidity_parser::pt::ContractDefinition;

use crate::{
    c3,
    utils::{ast, map_collection, SolidityAST},
};

use super::{
    interface::InterfaceData,
    misc::{Enum, Error, Event},
    ContractData,
};

pub struct Package {
    contracts: Vec<ContractData>,
    events: Vec<Event>,
    errors: Vec<Error>,
    enums: Vec<Enum>,
    interfaces: Vec<InterfaceData>,
}

impl Package {
    pub fn new(ast: SolidityAST) -> Result<Self, &'static str> {
        let contracts: Vec<&ContractDefinition> = ast::extract_contracts(&ast);
        let c3 = c3::linearization(&contracts);

        let contract_classes =
            c3::find_top_level_contracts(&contracts, &c3).expect("At least one contract expected");

        let interfaces = ast::extract_interfaces(&contracts)
            .iter()
            .map(|i| InterfaceData::new(i))
            .collect::<Vec<_>>();

        let events = map_collection(ast::extract_events(&ast));
        let errors = map_collection(ast::extract_errors(&ast));
        let enums = map_collection(ast::extract_enums(&ast));

        let contracts = contract_classes
            .iter()
            .map(|class| {
                ContractData::try_from((class, &contracts))
                    .expect("The ast should allow to create a valid PackageDef")
            })
            .collect();

        Ok(Package {
            contracts,
            events,
            errors,
            enums,
            interfaces,
        })
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub fn contracts(&self) -> &[ContractData] {
        self.contracts.as_ref()
    }

    pub fn interfaces(&self) -> &[InterfaceData] {
        self.interfaces.as_ref()
    }

    pub fn enums(&self) -> &[Enum] {
        self.enums.as_ref()
    }
}
