use std::error::Error;

use c3_lang_linearization::{Class, Fn, C3};
use c3_lang_parser::c3_ast::ClassNameDef;
use solidity_parser::pt::{ContractDefinition, FunctionDefinition, VariableDefinition, SourceUnitPart, EventDefinition, ErrorDefinition};

use crate::{linearization::{parse_func_id, self}, utils};

type FnImpls<'a> = (String, Vec<(String, &'a FunctionDefinition)>);
pub struct ContractData<'a> {
    contract: &'a ContractDefinition,
    base_contracts: Vec<&'a ContractDefinition>,
    events: Vec<&'a EventDefinition>,
    errors: Vec<&'a ErrorDefinition>,
    c3: C3,
}

impl<'a> ContractData<'a> {
    pub fn new(
        solidity_ast: &'a [SourceUnitPart],
    ) -> Self {
        let contracts: Vec<&ContractDefinition> = utils::extract_contracts(solidity_ast);
        let contract = contracts.last().expect("Contract not found").to_owned();

        let events = utils::extract_events(solidity_ast);
        let errors = utils::extract_errors(solidity_ast);

        let c3 = linearization::c3_linearization(&contracts);
        let base_contracts = utils::get_base_contracts(contract, contracts, &c3);
        Self {
            contract,
            base_contracts,
            events,
            errors,
            c3,
        }
    }

    /// Extracts contract name and wraps with c3 ast abstraction.
    pub fn c3_class(&self) -> Class {
        Class::from(self.contract.name.name.as_str())
    }

    // Extracts contract name and wraps with c3 ast abstraction.
    ///
    /// May contain one or more class name
    pub fn c3_class_name_def(&self) -> ClassNameDef {
        ClassNameDef {
            classes: self.c3_path(),
        }
    }

    /// Extracts contract name with inherited contracts and wraps with c3 ast abstraction.
    pub fn c3_path(&self) -> Vec<Class> {
        let contract_id = self.c3_class();
        self.c3.path(&contract_id).expect("Invalid contract path")
    }

    pub fn c3_fn_implementations(&self) -> Vec<FnImpls> {
        let all_functions = self.all_functions();
        let mut result = vec![];
        for fn_name in self.c3_functions_str() {
            let implementations = self
                .all_functions()
                .into_iter()
                .filter_map(|(contract_name, functions)| {
                    find_fn(contract_name, functions, &fn_name)
                })
                .collect::<Vec<_>>();
            result.push((fn_name, implementations));
        }
        result
    }

    pub fn c3_functions_str(&self) -> Vec<String> {
        let contract_name_str = self.contract.name.name.as_str();
        self.c3.functions_str(contract_name_str)
    }

    pub fn c3_vars(&self) -> Vec<&VariableDefinition> {
        let mut vars = self
            .base_contracts
            .iter()
            .map(|contract| utils::extract_vars(contract))
            .flatten()
            .collect::<Vec<_>>();
        vars.dedup();
        vars
    }

    pub fn c3_events(&self) -> &[&EventDefinition] {
        &self.events
    }

    pub fn c3_errors(&self) -> &[&ErrorDefinition] {
        &self.errors
    }

    pub fn c3_events_str(&self) -> Vec<String> {
        self.events.iter().map(|ev| ev.name.name.to_owned()).collect()
    }

    fn all_functions(&self) -> Vec<(String, Vec<&FunctionDefinition>)> {
        self.base_contracts
            .iter()
            .map(|contract| {
                (
                    contract.name.name.clone(),
                    utils::extract_functions(contract),
                )
            })
            .collect::<Vec<_>>()
    }
}

fn find_fn<'a>(
    contract_name: String,
    functions: Vec<&'a FunctionDefinition>,
    name: &str,
) -> Option<(String, &'a FunctionDefinition)> {
    let result = functions
        .iter()
        .find(|f| parse_func_id(f) == Fn::from(name))
        .map(|f| *f);
    result.map(|f| (contract_name, f))
}
