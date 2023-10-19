use solidity_parser::{
    pt::{
        ContractDefinition, ContractPart, ContractTy, EnumDefinition, ErrorDefinition,
        EventDefinition, FunctionDefinition, SourceUnitPart, StructDefinition, VariableDefinition,
    },
    Diagnostic,
};

pub(crate) fn parse(input: &str) -> Result<Vec<SourceUnitPart>, Vec<Diagnostic>> {
    let solidity_ast = solidity_parser::parse(&input, 0);
    solidity_ast.map(|ast| ast.0 .0)
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

/// Filters [ContractDefinition] from solidity ast.
pub(crate) fn extract_interfaces<'a>(
    contracts: &'a [&'a ContractDefinition],
) -> Vec<&ContractDefinition> {
    contracts
        .iter()
        .filter(|c| matches!(c.ty, ContractTy::Interface(_)))
        .map(|c| *c)
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

pub(crate) fn extract_enums(ast: &[SourceUnitPart]) -> Vec<&EnumDefinition> {
    filter_source_unit_part(ast, |unit| match unit {
        SourceUnitPart::ContractDefinition(contract) => {
            let events = filter_source_part(contract, |part| match part {
                ContractPart::EnumDefinition(ev) => Some(ev.as_ref()),
                _ => None,
            });
            Some(events)
        }
        SourceUnitPart::EnumDefinition(ev) => Some(vec![ev.as_ref()]),
        _ => None,
    })
}

pub(crate) fn extract_structs(ast: &[SourceUnitPart]) -> Vec<(Option<String>, &StructDefinition)> {
    filter_source_unit_part(ast, |unit| match unit {
        SourceUnitPart::ContractDefinition(contract) => {
            let events = filter_source_part(contract, |part| match part {
                ContractPart::StructDefinition(ev) => {
                    Some((Some(contract.name.name.to_owned()), ev.as_ref()))
                }
                _ => None,
            });
            Some(events)
        }
        SourceUnitPart::StructDefinition(ev) => Some(vec![(None, ev.as_ref())]),
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
