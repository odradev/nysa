use solidity_parser::pt::{
    ContractDefinition, ContractPart, ErrorDefinition, EventDefinition, FunctionDefinition,
    SourceUnitPart, VariableDefinition,
};

pub(crate) fn parse(input: &str) -> Vec<SourceUnitPart> {
    let solidity_ast = solidity_parser::parse(&input, 0).unwrap();
    let solidity_ast: Vec<SourceUnitPart> = solidity_ast.0 .0;
    solidity_ast
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
    filter_source_unit_part(ast, |unit| match unit {
        SourceUnitPart::ContractDefinition(contract) => {
            let events = filter_source_part(contract, |part| match part {
                ContractPart::EventDefinition(ev) => Some(ev.as_ref()),
                _ => None,
            });
            Some(events)
        }
        SourceUnitPart::EventDefinition(ev) => Some(vec![ev.as_ref()]),
        _ => None,
    })
}

pub(crate) fn extract_errors(ast: &[SourceUnitPart]) -> Vec<&ErrorDefinition> {
    filter_source_unit_part(ast, |unit| match unit {
        SourceUnitPart::ContractDefinition(contract) => {
            let errors = filter_source_part(contract, |part| match part {
                ContractPart::ErrorDefinition(err) => Some(err.as_ref()),
                _ => None,
            });
            Some(errors)
        }
        SourceUnitPart::ErrorDefinition(err) => Some(vec![err.as_ref()]),
        _ => None,
    })
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
    ast.iter().filter_map(f).flatten().collect::<Vec<_>>()
}
