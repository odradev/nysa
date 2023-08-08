use c3_lang_linearization::{Class, C3};
use convert_case::{Case, Casing};
use quote::format_ident;
use solidity_parser::pt::{
    ContractDefinition, ContractPart, EventDefinition, FunctionDefinition, SourceUnitPart,
    VariableDefinition, ErrorDefinition,
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

pub(crate) fn parse_to_solidity_ast(input: &str) -> Vec<SourceUnitPart> {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: Vec<SourceUnitPart> = solidity_ast.0 .0;
    solidity_ast
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

/// Filters [ContractDefinition] from solidity ast.
pub(crate) fn extract_contracts<'a>(ast: &[SourceUnitPart]) -> Vec<&ContractDefinition> {
    ast.iter()
        .filter_map(|unit| match unit {
            SourceUnitPart::ContractDefinition(contract) => Some(contract.as_ref()),
            _ => None,
        })
        .collect::<Vec<_>>()
}

/// Filters [FunctionDefinition] from a contract.
pub(crate) fn extract_functions(contract: &ContractDefinition) -> Vec<&FunctionDefinition> {
    filter_source_part(contract, |part| match part {
        ContractPart::FunctionDefinition(func) => Some(func.as_ref()),
        _ => None,
    })
}

/// Filters [VariableDefinition] from a contract.
pub(crate) fn extract_vars(contract: &ContractDefinition) -> Vec<&VariableDefinition> {
    filter_source_part(contract, |part| match part {
        ContractPart::VariableDefinition(var) => Some(var.as_ref()),
        _ => None,
    })
}

/// Iterates over [SourceUnitPart]s and collects all [EventDefinition]s. An [EventDefinition] may be
/// at the top level or inside a [ContractDefinition].
pub(crate) fn extract_events(ast: &[SourceUnitPart]) -> Vec<&EventDefinition> {
    filter_source_unit_part(ast, |unit| {
        match unit {
            SourceUnitPart::ContractDefinition(contract) => {
                let events = filter_source_part(contract, |part| {
                    match part {
                        ContractPart::EventDefinition(ev) => Some(ev.as_ref()),
                        _ => None,
                    }
                });
                Some(events)
            }
            SourceUnitPart::EventDefinition(ev) => Some(vec![ev.as_ref()]),
            _ => None,
        }
    })
}


pub(crate) fn extract_errors(ast: &[SourceUnitPart]) -> Vec<&ErrorDefinition> {
    filter_source_unit_part(ast, |unit| {
        match unit {
            SourceUnitPart::ContractDefinition(contract) => {
                let errors = filter_source_part(contract, |part| {
                    match part {
                        ContractPart::ErrorDefinition(err) => Some(err.as_ref()),
                        _ => None,
                    }
                });
                Some(errors)
            }
            SourceUnitPart::ErrorDefinition(err) => Some(vec![err.as_ref()]),
            _ => None,
        }
    })
}

/// Extracts contract name with inherited contracts and wraps with c3 ast abstraction.
pub(crate) fn classes(contract: &ContractDefinition, c3: &C3) -> Vec<Class> {
    let contract_id = Class::from(contract.name.name.as_str());
    c3.path(&contract_id).expect("Invalid contract path")
}

fn filter_source_part<'a, F, V>(contract: &'a ContractDefinition, f: F) -> Vec<V>
where
    F: Fn(&'a ContractPart) -> Option<V>,
{
    contract.parts.iter().filter_map(f).collect::<Vec<_>>()
}

fn filter_source_unit_part<'a, F, V>(ast: &'a [SourceUnitPart], f: F) -> Vec<V>
where
    F: Fn(&'a SourceUnitPart) -> Option<Vec<V>>,
{
    ast.iter()
        .filter_map(f)
        .flatten()
        .collect::<Vec<_>>()
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
