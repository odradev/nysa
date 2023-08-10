use c3_lang_linearization::{Class, C3};
use solidity_parser::pt::ContractDefinition;

/// Extracts contract name with inherited contracts and wraps with c3 ast abstraction.
pub(crate) fn classes(contract: &ContractDefinition, c3: &C3) -> Vec<Class> {
    let contract_id = Class::from(contract.name.name.as_str());
    c3.path(&contract_id).expect("Invalid contract path")
}

pub(crate) fn get_base_contracts<'a>(
    top_lvl_contract: &'a ContractDefinition,
    contracts: Vec<&'a ContractDefinition>,
    c3: &C3,
) -> Vec<&'a ContractDefinition> {
    let classes = classes(top_lvl_contract, c3)
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();
    contracts
        .iter()
        .filter(|c| classes.contains(&c.name.name))
        .map(|c| *c)
        .collect::<Vec<_>>()
}