use c3_lang_linearization::{Class, C3};
use solidity_parser::pt::{self, ContractDefinition, Identifier};

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

    c3_lang_linearization::c3_linearization(c3).expect("Linearization failed")
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

fn to_class(id: &Identifier) -> Class {
    id.name.as_str().into()
}
