use solidity_parser::pt;

use crate::utils::{ast, map_collection};

use super::{func::NysaFunction, misc::NysaContract};

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
