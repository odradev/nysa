use c3_lang_linearization::{Class, Fn, C3};
use solidity_parser::pt::{self, ContractDefinition, FunctionDefinition, Identifier};

use crate::utils;

pub fn c3_linearization(contracts: &[&ContractDefinition]) -> C3 {
    let mut c3 = C3::new();
    contracts.iter().for_each(|contract| {
        register_class(contract, &mut c3);
        // register_fns(contract, &mut c3);
        // register_vars(contract, &mut c3);
    });
    let mut c3 = c3_lang_linearization::c3_linearization(c3).expect("Linearization failed");
    contracts.iter().for_each(|contract| {
        // register_class(contract, &mut c3);
        register_fns(contract, &mut c3);
        register_vars(contract, &mut c3);
    });
    c3
}

fn register_class(contract: &ContractDefinition, c3: &mut C3) {
    let class = to_class(&contract.name);
    let path = contract
        .base
        .iter()
        .map(|base| to_class(&base.name))
        .collect::<Vec<_>>();
    c3.add(class, path);
}

fn register_fns(contract: &ContractDefinition, c3: &mut C3) {
    let class = to_class(&contract.name);
    for func in utils::extract_functions(contract) {
        let fn_class = parse_func_id(func);
        c3.register_fn(class.clone(), fn_class);
    }
}

fn register_vars(contract: &ContractDefinition, c3: &mut C3) {
    let class = to_class(&contract.name);
    for var in utils::extract_vars(contract) {
        let var_id = &var.name;
        let var_class = to_class(var_id);
        c3.register_var(class.clone(), var_class)
    }
}

pub(crate) fn parse_func_id(def: &FunctionDefinition) -> Fn {
    let parse_unsafe = || -> Fn {
        def.name
            .as_ref()
            .map(|id| utils::to_snake_case(&id.name))
            .expect("Invalid func name")
            .into()
    };
    match &def.ty {
        // TODO: handle multiple constructors
        pt::FunctionTy::Constructor => "init".into(),
        pt::FunctionTy::Function => parse_unsafe(),
        pt::FunctionTy::Fallback => "__fallback".into(),
        pt::FunctionTy::Receive => "__receive".into(),
        pt::FunctionTy::Modifier => parse_unsafe(),
    }
}

#[cfg(test)]
mod t {
    use crate::{linearization::c3_linearization, utils};

    const SOLIDITY_CODE: &str = include_str!("../../example-owned-token/src/owned_token.sol");
    // const SOLIDITY_CODE: &str = include_str!("../../resources/mana_token.sol");

    #[test]
    fn test_c3() {
        let ast = crate::parse_to_solidity_ast(SOLIDITY_CODE);
        let contracts = utils::extract_contracts(&ast);

        let c3 = c3_linearization(&contracts);
        dbg!(c3.all_classes_str());
        for class in c3.all_classes_str() {
            dbg!(&class);
            dbg!(c3.varialbes_str(&class));
            dbg!(c3.functions_str(&class));
        }
        // dbg!(c3);
        assert!(false)
    }
}

fn to_class(id: &Identifier) -> Class {
    id.name.as_str().into()
}
