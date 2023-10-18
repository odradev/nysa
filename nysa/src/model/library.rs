use solidity_parser::pt;

use crate::utils::{ast, map_collection};

use super::{func::Function, Named};

pub struct LibraryData {
    name: String,
    fns: Vec<Function>,
}

impl From<&&pt::ContractDefinition> for LibraryData {
    fn from(value: &&pt::ContractDefinition) -> Self {
        Self::new(value)
    }
}

impl LibraryData {
    pub fn new(contract: &pt::ContractDefinition) -> Self {
        let fns = map_collection(ast::extract_functions(contract));
        let name = contract.name.name.clone();

        Self { name, fns }
    }

    pub fn fns(&self) -> &[Function] {
        self.fns.as_ref()
    }
}

impl Named for LibraryData {
    fn name(&self) -> String {
        self.name.clone()
    }
}
