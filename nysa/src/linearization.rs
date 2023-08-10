use c3_lang_linearization::{Class, C3};
use solidity_parser::pt::{self, ContractDefinition, Identifier};

use crate::utils;

pub fn c3_linearization(contracts: &[&ContractDefinition]) -> C3 {
    // collect interfaces to exclude them from the inheritance path
    let interfaces = contracts
        .iter()
        .filter(|c| matches!(c.ty, pt::ContractTy::Interface(_)))
        .map(|c| to_class(&c.name))
        .collect::<Vec<_>>();

    let mut c3 = C3::new();
    // register only contracts, exclude interfaces
    contracts
        .iter()
        .filter(|c| !matches!(c.ty, pt::ContractTy::Interface(_)))
        .for_each(|contract| register_class(contract, &interfaces, &mut c3));
    
    let mut c3 = c3_lang_linearization::c3_linearization(c3).expect("Linearization failed");
    // linearization clears fns and vars so we must add them at the end
    contracts.iter().for_each(|contract| {
        register_fns(contract, &mut c3);
        register_vars(contract, &mut c3);
    });
    c3
}

fn register_class(contract: &ContractDefinition, interfaces: &[Class], c3: &mut C3) {
    let class = to_class(&contract.name);
    // Solidity you declare derived classes in reverse order, so to build the path,
    // the original vector must be reversed.
    let path = contract
        .base
        .iter()
        .rev()
        .map(|b| to_class(&b.name))
        .filter_map(|c| interfaces.iter().all(|i| i != &c).then(|| c))
        .collect::<Vec<_>>();
    c3.add(class, path);
}

fn register_fns(contract: &ContractDefinition, c3: &mut C3) {
    let class = to_class(&contract.name);
    for func in utils::ast::extract_functions(contract) {
        let fn_class = utils::func::parse_id(func);
        c3.register_fn(class.clone(), fn_class);
    }
}

fn register_vars(contract: &ContractDefinition, c3: &mut C3) {
    let class = to_class(&contract.name);
    for var in utils::ast::extract_vars(contract) {
        let var_id = &var.name;
        let var_class = to_class(var_id);
        c3.register_var(class.clone(), var_class)
    }
}

fn to_class(id: &Identifier) -> Class {
    id.name.as_str().into()
}
