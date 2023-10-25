use solidity_parser::pt;

use crate::utils::{ast, map_collection};

use super::{func::Function, misc::Contract, Named};

#[derive(Debug, Clone)]
pub struct InterfaceData {
    contract: Contract,
    fns: Vec<Function>,
}

impl InterfaceData {
    pub fn new(contract: &pt::ContractDefinition) -> Self {
        let fns = map_collection(ast::extract_functions(contract));
        let contract = Contract::from(contract);

        Self { contract, fns }
    }

    pub fn contract(&self) -> &Contract {
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
