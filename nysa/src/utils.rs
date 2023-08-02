use c3_lang_linearization::{Class, C3};
use convert_case::{Case, Casing};
use quote::format_ident;
use solidity_parser::pt::{
    ContractDefinition, ContractPart, FunctionDefinition, SourceUnitPart, VariableDefinition,
};

pub fn to_snake_case_ident(name: &str) -> proc_macro2::Ident {
    format_ident!("{}", to_snake_case(name))
}

pub fn to_snake_case(input: &str) -> String {
    if input.starts_with('_') {
        // `to_case()` consumes the heading `_`
        format!("_{}", input.to_case(Case::Snake))
    } else {
        format!("{}", input.to_case(Case::Snake))
    }
}

pub(crate) fn extract_contracts<'a>(ast: &[SourceUnitPart]) -> Vec<&ContractDefinition> {
    ast.iter()
        .filter_map(|unit| match unit {
            SourceUnitPart::ContractDefinition(contract) => Some(contract.as_ref()),
            _ => None,
        })
        .collect::<Vec<_>>()
}

pub(crate) fn extract_functions(contract: &ContractDefinition) -> Vec<&FunctionDefinition> {
    contract
        .parts
        .iter()
        .filter_map(|part| match part {
            ContractPart::FunctionDefinition(func) => Some(func.as_ref()),
            _ => None,
        })
        .collect::<Vec<_>>()
}

pub(crate) fn extract_vars(contract: &ContractDefinition) -> Vec<&VariableDefinition> {
    contract
        .parts
        .iter()
        .filter_map(|part| match part {
            ContractPart::VariableDefinition(var) => Some(var.as_ref()),
            _ => None,
        })
        .collect::<Vec<_>>()
}

/// Extracts contract name with inherited contracts and wraps with c3 ast abstraction.
pub(crate) fn classes(contract: &ContractDefinition, c3: &C3) -> Vec<Class> {
    let contract_id = Class::from(contract.name.name.as_str());
    c3.path(&contract_id).expect("Invalid contract path")
}

#[cfg(test)]
mod t {
    use crate::utils::to_snake_case_ident;
    use proc_macro2::{Ident, Span};

    #[test]
    fn to_snake_case_ident_works() {
        assert_eq!(ident("my_value"), to_snake_case_ident("MyValue"));
        assert_eq!(ident("value"), to_snake_case_ident("value"));
        assert_eq!(ident("value"), to_snake_case_ident("Value"));
        assert_eq!(ident("_value"), to_snake_case_ident("_value"));
    }

    fn ident(string: &str) -> Ident {
        Ident::new(string, Span::call_site())
    }
}
