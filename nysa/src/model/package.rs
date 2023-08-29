use solidity_parser::pt::ContractDefinition;

use crate::{
    c3,
    utils::{ast, map_collection, SolidityAST},
};

use super::{
    interface::InterfaceData,
    misc::{NysaError, NysaEvent},
    ContractData,
};

pub struct Package {
    contracts: Vec<ContractData>,
    events: Vec<NysaEvent>,
    errors: Vec<NysaError>,
    interfaces: Vec<InterfaceData>, // c3: C3,
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
            interfaces,
        })
    }

    pub fn events(&self) -> &[NysaEvent] {
        &self.events
    }

    pub fn errors(&self) -> &[NysaError] {
        &self.errors
    }

    pub fn contracts(&self) -> &[ContractData] {
        self.contracts.as_ref()
    }

    pub fn interfaces(&self) -> &[InterfaceData] {
        self.interfaces.as_ref()
    }
}
