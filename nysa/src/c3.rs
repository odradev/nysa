use std::{collections::HashSet, vec};

use c3_lang_linearization::{Class, C3};
use solidity_parser::pt::{self, ContractDefinition, Identifier};

/// Applies C3 superclass linearization algorithm to solidity [ContractDefinition]
///
/// C3 superclass linearization is an algorithm used to obtain the order
/// in which methods should be inherited in the presence of multiple inheritance.
///
/// The output of C3 superclass linearization is a deterministic Method Resolution Order (MRO).
///
/// # Arguments
///
/// * `contracts` - A [ContractDefinition] slice that holds solidity contact abstract syntax tree (AST).
///
/// # Panics
///
/// * if c3 linearization fails from any reason
pub fn linearization(contracts: &[&ContractDefinition]) -> C3 {
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

/// Filters ids of top-level contracts from a slice. A top-level contract is a contract that is the root of the inheritance graph.
///
/// # Example
///
/// Having contracts [A, B, C, D, E, F] their dependencies are as follow:
/// B is A
/// C is A
/// D is B and C (so D is A, B and C)
/// E is B and F (so E is A, B and F)
///
/// From all of those contracts D and E are top level contracts.
pub fn find_top_level_contracts(
    contracts: &[&ContractDefinition],
    c3: &C3,
) -> Result<Vec<Class>, &'static str> {
    // The contract defined as last is considered as a top level contract.
    // For instance: if there there a few base contracts (interfaces, abstract, etc.) that a contract inherits
    // from, these contract are defined first, and then the ultimate contract.
    let contract = contracts.last().ok_or("No contract found")?;

    let mut contact_class: Class = contract.name.name.as_str().into();
    let mut result: Vec<Class> = vec![contact_class.clone()];

    let mut class_set = HashSet::<Class>::from_iter(c3.all_classes());

    while !class_set.is_empty() {
        let contract_path = c3.path(&contact_class).expect("Invalid contract path");
        let contract_path = HashSet::<Class>::from_iter(contract_path);
        let diff = class_set.difference(&contract_path);
        class_set = HashSet::<Class>::from_iter(diff.into_iter().map(Clone::clone));
        if let Some(class) = class_set.iter().last() {
            contact_class = class.clone();
            result.push(class.clone());
        }
    }
    result.sort();
    Ok(result)
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
