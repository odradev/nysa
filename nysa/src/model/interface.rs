use solidity_parser::pt;

use crate::utils::{ast, map_collection};

use super::{func::NysaFunction, misc::NysaContract, Named};

pub struct InterfaceData {
    contract: NysaContract,
    fns: Vec<NysaFunction>,
}

impl InterfaceData {
    pub fn new(contract: &pt::ContractDefinition) -> Self {
        let fns = map_collection(ast::extract_functions(contract));
        let contract = NysaContract::from(contract);

        Self { contract, fns }
    }

    pub fn contract(&self) -> &NysaContract {
        &self.contract
    }

    pub fn fns(&self) -> &[NysaFunction] {
        self.fns.as_ref()
    }
}

impl Named for InterfaceData {
    fn name(&self) -> String {
        self.contract.name().to_string()
    }
}
