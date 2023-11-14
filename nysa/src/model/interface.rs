use solidity_parser::pt;

use crate::utils::{ast, map_collection};

use super::{func::Function, misc::ContractMetadata, Named};

/// An interface representation.
#[derive(Debug, Clone)]
pub struct InterfaceData {
    contract: ContractMetadata,
    fns: Vec<Function>,
}

impl InterfaceData {
    pub fn new(contract: &pt::ContractDefinition) -> Self {
        let fns = map_collection(ast::extract_functions(contract));
        let contract = ContractMetadata::from(contract);

        Self { contract, fns }
    }

    pub fn contract(&self) -> &ContractMetadata {
        &self.contract
    }

    pub fn fns(&self) -> &[Function] {
        self.fns.as_ref()
    }
}

impl Named for InterfaceData {
    fn name(&self) -> String {
        self.contract.name().to_string()
    }
}

impl From<&&pt::ContractDefinition> for InterfaceData {
    fn from(value: &&pt::ContractDefinition) -> Self {
        InterfaceData::new(value)
    }
}
